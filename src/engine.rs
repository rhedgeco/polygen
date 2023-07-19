use glob::glob;
use rhai::{module_resolvers::FileModuleResolver, serde::to_dynamic, Dynamic, Engine, Scope, AST};
use std::{fs, io, path::PathBuf, sync::Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("'{name}' - {error}")]
pub struct ValidationError {
    name: String,
    error: anyhow::Error,
}

struct PolyScript {
    ast: AST,
    name: String,
    items: Mutex<Vec<Dynamic>>,
}

impl PolyScript {
    pub fn new(name: String, ast: AST) -> Self {
        Self {
            ast,
            name,
            items: Mutex::new(Vec::new()),
        }
    }
}

pub struct PolygenEngine {
    log: PolyLog,
    engine: Engine,
    scripts: Vec<PolyScript>,
}

impl PolygenEngine {
    pub fn new(script_dir: &str) -> Self {
        // create engine
        let mut poly = Self {
            engine: Engine::new(),
            scripts: Vec::new(),
            log: PolyLog::new(),
        };

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
                Ok(ast) => poly.scripts.push(PolyScript::new(name, ast)),
                Err(e) => {
                    poly.log.error(&e.to_string());
                    continue;
                }
            }
        }

        poly
    }

    pub fn process_item(&self, item: syn_serde::Item) -> Result<(), ValidationError> {
        let dynamic_item = to_dynamic(item).expect("Internal Error: Bad source item");

        for script in self.scripts.iter() {
            let args = (dynamic_item.clone(),);
            let processed_item: Dynamic = self
                .engine
                .call_fn(&mut Scope::new(), &script.ast, "process", args)
                .map_err(|error| {
                    // process runtime errors to make them prettier
                    use rhai::EvalAltResult::*;
                    let name = script.name.clone();
                    let ErrorInFunctionCall(_, _, error, _) = *error else {
                        let error = anyhow::Error::new(error);
                        return ValidationError { name, error };
                    };

                    let ErrorRuntime(item, _) = *error else {
                        let error = anyhow::Error::new(error);
                        return ValidationError { name, error };
                    };

                    let error = anyhow::Error::msg(item.to_string());
                    return ValidationError { name, error };
                })?;

            script.items.lock().unwrap().push(processed_item);
        }

        Ok(())
    }

    pub fn flush_bindings(&self, binding_dir: &str) -> Result<(), ValidationError> {
        let dir = PathBuf::from(binding_dir);
        for script in self.scripts.iter() {
            let file_path = dir.join(script.name.clone());
            let items_guard = script.items.lock().unwrap();
            let dynamic_items =
                to_dynamic(&*items_guard).expect("Internal Error: Bad source items");
            let args = (dynamic_items,);
            let binding: String = self
                .engine
                .call_fn(&mut Scope::new(), &script.ast, "render", args)
                .map_err(|error| {
                    let error = anyhow::Error::new(error);
                    let name = script.name.clone();
                    ValidationError { name, error }
                })?;

            fs::write(file_path, binding).map_err(|error| {
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

    pub fn error(&mut self, text: &str) {
        self.log += &format!("[ERROR]: {text}\n");
    }

    pub fn full_text(&self) -> &str {
        &self.log
    }
}
