use glob::glob;
use rhai::{module_resolvers::FileModuleResolver, serde::to_dynamic, Engine, Scope, AST};
use std::{env, fs, io, path::PathBuf};
use thiserror::Error;

use crate::functions;

#[derive(Debug, Error)]
#[error("'{name}' - {error}")]
pub struct ValidationError {
    name: String,
    error: anyhow::Error,
}

struct PolyScript {
    ast: AST,
    name: String,
}

impl PolyScript {
    pub fn new(name: String, ast: AST) -> Self {
        Self { ast, name }
    }
}

pub struct PolygenEngine {
    log: PolyLog,
    engine: Engine,
    scripts: Vec<PolyScript>,
    build_dir: PathBuf,
    package_name: String,
}

impl PolygenEngine {
    pub fn new(script_dir: &str, build_dir: &str) -> Self {
        // create engine
        let mut poly = Self {
            engine: Engine::new(),
            scripts: Vec::new(),
            log: PolyLog::new(),
            build_dir: PathBuf::from(build_dir),
            package_name: env::var("CARGO_PKG_NAME").unwrap_or("untitled".to_string()),
        };
        poly.log.info("-- Initializing Polygen Engine --");

        // set up rhai engine
        poly.engine.set_max_expr_depths(32, 32); // idk this felt right lol
        poly.engine
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
        poly.log.info(&format!("Loading scripts at '{script_dir}'"));
        let rhai_paths = match glob(&rhai_glob) {
            Ok(paths) => paths,
            Err(e) => {
                poly.log.error(&e.to_string());
                return poly;
            }
        };

        // loop over all scripts
        for path in rhai_paths {
            let rhai_path = match path {
                Ok(entry) => entry,
                Err(e) => {
                    poly.log.error(&e.to_string());
                    continue;
                }
            };

            // load rhai script
            let name = rhai_path
                .with_extension("")
                .file_name()
                .map_or("untitled".to_string(), |s| s.to_string_lossy().to_string());
            let rhai_str = rhai_path.to_string_lossy().to_string();
            poly.log.info(&format!("Loading rhai script '{rhai_str}'"));
            match poly.engine.compile_file(rhai_path) {
                Err(e) => {
                    poly.log.error(&e.to_string());
                    continue;
                }
                Ok(ast) => {
                    let has_build_function = ast
                        .iter_functions()
                        .any(|f| f.name == "build" && f.params.len() == 1);

                    if has_build_function {
                        poly.log.info(&format!("Generator registered -> {name}"));
                        poly.scripts.push(PolyScript::new(name, ast));
                    } else {
                        poly.log.warn(&format!(
                            "Failed to load script '{rhai_str}'.\n\
                            - Scripts must contain a 'build(item)' function.\n\
                            - All scripts in the root of './polygen' will try to be loaded.\n\
                            - If this is a utility script consider placing it in a 'utils' folder."
                        ));
                    }
                }
            }
        }

        poly.log.info(&format!(
            "Initialization Complete - {} Generators registered.",
            poly.scripts.len(),
        ));

        poly
    }

    pub fn build_item(&self, item: syn_serde::Item) -> Result<(), ValidationError> {
        let dynamic_item = to_dynamic(item).expect("Internal Error: Bad source item");

        for script in self.scripts.iter() {
            // first build the item into
            let args = (dynamic_item.clone(),);
            let mut scope = Scope::new();
            scope.push_constant("PACKAGE_NAME", self.package_name.clone());
            let built_item: String = self
                .engine
                .call_fn(&mut scope, &script.ast, "build", args)
                .map_err(|mut error| {
                    // unravel any nested errors from function calls to get to the root error
                    use rhai::EvalAltResult::*;
                    while let ErrorInFunctionCall(_, _, inner, _) = *error {
                        error = inner;
                    }

                    // process runtime errors to make them prettier
                    let error = match *error {
                        ErrorRuntime(item, _) => anyhow::Error::msg(format!("{item}")),
                        error => anyhow::Error::new(error),
                    };

                    let name = script.name.clone();
                    return ValidationError { name, error };
                })?;

            // then save the item to disk
            let item_dir = self.build_dir.join("items");
            fs::create_dir_all(item_dir.clone()).map_err(|error| {
                let name = script.name.clone();
                let error = anyhow::Error::new(error);
                ValidationError { name, error }
            })?;

            let file_path = item_dir.join(script.name.clone());
            fs::write(file_path, built_item).map_err(|error| {
                let name = script.name.clone();
                let error = anyhow::Error::new(error);
                ValidationError { name, error }
            })?;
        }

        Ok(())
    }

    pub fn flush_logs(&self, log_path: PathBuf) -> io::Result<()> {
        let logs = self.log.full_text();
        fs::write(log_path, logs)
    }
}

struct PolyLog {
    log: String,
}

impl PolyLog {
    pub fn new() -> Self {
        Self { log: String::new() }
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

    pub fn full_text(&self) -> &str {
        &self.log
    }
}
