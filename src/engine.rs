use glob::glob;
use rhai::{serde::to_dynamic, Dynamic, Engine, Scope, AST};
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
    sync::Mutex,
};
use tera::Tera;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("'{name}' - {error}")]
pub struct ValidationError {
    name: String,
    error: Box<rhai::EvalAltResult>,
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
    tera: Tera,
    engine: Engine,
    scripts: Vec<PolyScript>,
    log: PolyLog,
}

impl PolygenEngine {
    pub fn new(script_dir: &str) -> Self {
        // create polyengine
        let mut poly = Self {
            tera: Tera::default(),
            engine: Engine::new(),
            scripts: Vec::new(),
            log: PolyLog::new(),
        };

        // load rhai scripts
        let dir = PathBuf::from(script_dir);
        let rhai_glob = dir.join("**").join("*.rhai").to_string_lossy().to_string();
        poly.log.info(&format!(
            "Loading scripts and templates from '{script_dir}'"
        ));
        let rhai_paths = match glob(&rhai_glob) {
            Ok(paths) => paths,
            Err(e) => {
                poly.log.error(&e.to_string());
                return poly;
            }
        };

        // load all data
        for path in rhai_paths {
            let rhai_path = match path {
                Ok(entry) => entry,
                Err(e) => {
                    poly.log.error(&e.to_string());
                    continue;
                }
            };

            // load tera template
            let rhai_path_string = rhai_path.to_string_lossy().to_string();
            poly.log
                .info(&format!("Found rhai script at '{rhai_path_string}'"));
            let tera_path = rhai_path
                .with_extension("tera")
                .to_string_lossy()
                .to_string();
            poly.log
                .info(&format!("Loading associated tera file '{tera_path}'"));
            let name = rhai_path
                .with_extension("")
                .file_name()
                .map_or("untitled".to_string(), |s| s.to_string_lossy().to_string());
            if let Err(e) = poly.tera.add_template_file(tera_path, Some(&name)) {
                poly.log.error(&e.to_string());
                continue;
            }

            // load rhai script
            poly.log
                .info(&format!("Loading rhai script '{rhai_path_string}'"));
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
        let dynamic_item = to_dynamic(item).expect("Bad source item");

        for script in self.scripts.iter() {
            let name = "process";
            let mut scope = Scope::new();
            let args = (dynamic_item.clone(),);
            let processed_item: Dynamic = self
                .engine
                .call_fn(&mut scope, &script.ast, name, args)
                .map_err(|error| {
                    let name = script.name.clone();
                    ValidationError { name, error }
                })?;

            script.items.lock().unwrap().push(processed_item);
        }

        Ok(())
    }

    pub fn flush_bindings(&self, binding_dir: &str) -> anyhow::Result<()> {
        let dir = PathBuf::from(binding_dir);
        for script in self.scripts.iter() {
            let item_guard = script.items.lock().unwrap();
            let mut context = tera::Context::new();
            context.insert("items", &*item_guard);
            let file_path = dir.join(script.name.clone());
            let file = File::create(file_path)?;
            self.tera.render_to(&script.name, &context, file)?;
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
