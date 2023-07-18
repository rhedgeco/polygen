use rhai::{serde::to_dynamic, Engine, Scope, AST};
use std::{fs, io, path::PathBuf};
use thiserror::Error;

struct PolyScript {
    name: String,
    ast: AST,
}

struct PolyLog {
    log: String,
}

impl PolyLog {
    pub fn new() -> Self {
        Self { log: String::new() }
    }

    pub fn log(&mut self, text: &str) {
        self.log += &format!("[LOG]: {text}\n");
    }

    pub fn error(&mut self, text: &str) {
        self.log += &format!("[ERROR]: {text}\n");
    }

    pub fn full_text(&self) -> &str {
        &self.log
    }
}

#[derive(Debug, Error)]
#[error("'{name}' - {error}")]
pub struct ValidationError {
    name: String,
    error: Box<rhai::EvalAltResult>,
}

pub struct PolygenEngine {
    engine: Engine,
    scripts: Vec<PolyScript>,
    log: PolyLog,
}

impl PolygenEngine {
    pub fn new(script_dir: &str) -> Self {
        let mut poly = Self {
            engine: Engine::new(),
            scripts: Vec::new(),
            log: PolyLog::new(),
        };

        let dir = PathBuf::from(script_dir);
        poly.log
            .log(&format!("Loading scripts from '{}'", script_dir));
        let read_dir = match fs::read_dir(dir) {
            Ok(read_dir) => read_dir,
            Err(e) => {
                poly.log.error(&e.to_string());
                return poly;
            }
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    poly.log.error(&e.to_string());
                    continue;
                }
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let name = path
                .with_extension("")
                .file_name()
                .map_or("untitled".to_string(), |s| s.to_string_lossy().to_string());

            let path_string = path.to_string_lossy().to_string();
            poly.log.log(&format!("Found file '{path_string}'"));

            if path.extension().map(|s| s == "rhai").unwrap_or(false) {
                poly.log.log(&format!("Loading rhai script '{name}'"));
                match poly.engine.compile_file(path) {
                    Ok(ast) => poly.scripts.push(PolyScript { name, ast }),
                    Err(e) => {
                        poly.log.error(&e.to_string());
                        continue;
                    }
                };
            }
        }

        poly
    }

    pub fn validate_item(&self, item: syn_serde::Item) -> Result<(), ValidationError> {
        let dynamic_item = to_dynamic(item).expect("Bad source item");

        for script in self.scripts.iter() {
            let name = "validate";
            let mut scope = Scope::new();
            let args = (dynamic_item.clone(),);
            self.engine
                .call_fn::<()>(&mut scope, &script.ast, name, args)
                .map_err(|error| {
                    let name = script.name.clone();
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
