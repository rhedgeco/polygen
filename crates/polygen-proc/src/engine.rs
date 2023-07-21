use glob::glob;
use hashbrown::HashSet;
use proc_macro::TokenStream;
use quote::quote;
use rhai::{module_resolvers::FileModuleResolver, serde::to_dynamic, Dynamic, Engine, Scope, AST};
use std::{env, fs, io, path::PathBuf, sync::Mutex};
use syn_serde::Syn;

use crate::{functions, process};

pub struct PolygenEngine {
    engine: Engine,
    scripts: Vec<PolyScript>,
    build_dir: PathBuf,
    package_name: String,
}

impl PolygenEngine {
    pub fn new(script_dir: &str, build_dir: &str) -> Self {
        // create simple logging structure
        let mut log = PolyLog::new();
        log.info("-- Initializing Polygen Engine --");

        // create engine
        let mut poly = Self {
            engine: Engine::new(),
            scripts: Vec::new(),
            build_dir: PathBuf::from(build_dir),
            package_name: env::var("CARGO_PKG_NAME").unwrap_or("untitled".to_string()),
        };

        // set up rhai engine
        poly.engine
            .set_max_expr_depths(32, 32) // idk this felt right lol
            .register_fn("indent", functions::indent)
            .register_fn("replace", functions::replace)
            .register_fn("as_camel_case", functions::as_camel_case)
            .register_fn("as_pascal_case", functions::as_pascal_case)
            .register_fn("as_snake_case", functions::as_snake_case)
            .register_fn("as_capital_snake_case", functions::as_capital_snake_case)
            .register_fn("as_kebab_case", functions::as_kebab_case)
            .register_fn("as_capital_kebab_case", functions::as_capital_kebab_case)
            .register_fn("as_train_case", functions::as_train_case)
            .register_fn("as_title_case", functions::as_title_case)
            .register_fn("docformat", functions::docformat);

        // set up module resolvers for engine
        let dir = PathBuf::from(script_dir);
        let resolver = FileModuleResolver::new_with_path(dir.clone());
        poly.engine.set_module_resolver(resolver);

        // load rhai scripts
        let rhai_glob = dir.join("*.rhai").to_string_lossy().to_string();
        log.info(&format!("Loading scripts at '{script_dir}'"));
        let rhai_paths = match glob(&rhai_glob) {
            Ok(paths) => paths,
            Err(e) => {
                log.error(&e.to_string());
                return poly;
            }
        };

        // loop over all scripts
        for path in rhai_paths {
            let rhai_path = match path {
                Ok(entry) => entry,
                Err(e) => {
                    log.error(&e.to_string());
                    continue;
                }
            };

            // load rhai script
            let name = rhai_path
                .with_extension("")
                .file_name()
                .map_or("untitled".to_string(), |s| s.to_string_lossy().to_string());
            let rhai_str = rhai_path.to_string_lossy().to_string();
            log.info(&format!("Loading rhai script '{rhai_str}'"));
            match poly.engine.compile_file(rhai_path) {
                Err(e) => {
                    log.error(&e.to_string());
                    continue;
                }
                Ok(ast) => {
                    let mut found_process = false;
                    let mut found_render = false;
                    for f in ast.iter_functions() {
                        if f.name == "process" && f.params.len() == 1 {
                            found_process = true;
                        } else if f.name == "render" && f.params.len() == 1 {
                            found_render = true;
                        }
                    }

                    if found_process && found_render {
                        log.info(&format!("Generator registered -> {name}"));
                        poly.scripts.push(PolyScript::new(name, ast));
                    } else {
                        log.warn(&format!(
                            "Failed to load script '{rhai_str}'.\n\
                            - Scripts must contain 'process(type)' and 'render(items)' functions.\n\
                            - All scripts in the root of './polygen' will try to be loaded.\n\
                            - If this is a utility script consider placing it in a 'utils' folder."
                        ));
                    }
                }
            }
        }

        // write last log message and flush
        log.info(&format!(
            "Initialization Complete - {} Generators registered.",
            poly.scripts.len(),
        ));

        // flush the logs to the
        log.flush(&poly.build_dir)
            .expect("Failed to write to './target/polygen.log'");

        poly
    }

    pub fn process(&self, item: &syn::Item) -> TokenStream {
        // process the item using compiler assertions first
        use syn::Item::*;
        let processed_item = match item {
            Struct(item) => process::polystruct(item),
            Fn(item) => process::polyfunction(item),
            item => return process::unsupported(item).into(),
        };

        // then build a dynamic item for use in scripts
        let dynamic_item = to_dynamic(item.to_adapter()).expect("Internal Error: Bad source item");

        // loop over every script
        for script in self.scripts.iter() {
            // execute the process function
            match self.engine.call_fn::<Dynamic>(
                &mut Scope::new(),
                &script.ast,
                "process",
                (dynamic_item.clone(),),
            ) {
                // if the function succeded, save the processed item
                Ok(dynamic) => {
                    let mut items = script.items.lock().unwrap();
                    match item {
                        Struct(item) => items.insert_struct(item.ident.to_string(), dynamic),
                        Fn(item) => items.insert_function(item.sig.ident.to_string(), dynamic),
                        _ => return quote!("Internal Error: Item processed but not saved").into(),
                    }
                }

                // if the function failed, early return the error
                Err(mut error) => {
                    // unravel any nested errors from function calls to get to the root error
                    use rhai::EvalAltResult::*;
                    while let ErrorInFunctionCall(_, _, inner, _) = *error {
                        error = inner;
                    }

                    // process runtime errors to make them prettier
                    let error = match *error {
                        ErrorRuntime(item, _) => format!("{item}"),
                        error => format!("{error}"),
                    };

                    // build the compiler error and return the stream
                    let error_message = format!("{} - {error}", script.name);
                    return quote! {
                        compile_error!(#error_message);
                        #processed_item
                    }
                    .into();
                }
            }

            // execute the render function
            let mut scope = Scope::new();
            scope.push_constant("PACKAGE_NAME", self.package_name.clone());
            match self.engine.call_fn::<String>(
                &mut scope,
                &script.ast,
                "render",
                (script.items.lock().unwrap().build_map(),),
            ) {
                // save the rendered file
                Ok(rendered) => {
                    let render_dir = self.build_dir.join("generated");
                    let render_path = render_dir.join(&script.name);
                    if let Err(error) = fs::create_dir_all(render_dir) {
                        let error_message = format!("{} - {error}", script.name);
                        return quote! {
                            compile_error!(#error_message);
                            #processed_item
                        }
                        .into();
                    }

                    if let Err(error) = fs::write(render_path, rendered) {
                        let error_message = format!("{} - {error}", script.name);
                        return quote! {
                            compile_error!(#error_message);
                            #processed_item
                        }
                        .into();
                    }
                }
                // bubble up the rendering error
                Err(error) => {
                    let error_message = format!("{} - {error}", script.name);
                    return quote! {
                        compile_error!(#error_message);
                        #processed_item
                    }
                    .into();
                }
            }
        }

        // return the successfully processed and validated item
        processed_item.into()
    }
}

struct PolyScript {
    ast: AST,
    name: String,
    items: Mutex<PolyItems>,
}

impl PolyScript {
    pub fn new(name: String, ast: AST) -> Self {
        Self {
            ast,
            name,
            items: Default::default(),
        }
    }
}

#[derive(Default)]
struct PolyItems {
    struct_names: HashSet<String>,
    function_names: HashSet<String>,

    structs: Vec<Dynamic>,
    functions: Vec<Dynamic>,
}

impl PolyItems {
    pub fn build_map(&mut self) -> rhai::Map {
        let structs = to_dynamic(&self.structs).expect("Internal Error: Bad structs vec");
        let functions = to_dynamic(&self.functions).expect("Internal Error: Bad structs vec");
        let mut map = rhai::Map::new();
        map.insert("structs".into(), structs);
        map.insert("functions".into(), functions);
        map
    }

    pub fn insert_struct(&mut self, name: String, dynamic: Dynamic) {
        if !self.struct_names.insert(name) {
            return;
        }

        self.structs.push(dynamic);
    }

    pub fn insert_function(&mut self, name: String, dynamic: Dynamic) {
        if !self.function_names.insert(name) {
            return;
        }

        self.functions.push(dynamic);
    }
}

#[derive(Default)]
struct PolyLog {
    log: String,
}

impl PolyLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info(&mut self, text: &str) {
        self.log += &format!("[INFO]: {text}\n");
    }

    pub fn warn(&mut self, text: &str) {
        self.log += &format!("[WARN]: {text}\n");
    }

    pub fn error(&mut self, text: &str) {
        self.log += &format!("[ERROR]: {text}\n");
    }

    pub fn flush(self, dir: &PathBuf) -> io::Result<()> {
        fs::create_dir_all(dir)?;
        fs::write(dir.join("polygen.log"), self.log)
    }
}
