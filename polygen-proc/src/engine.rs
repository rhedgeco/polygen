mod functions;
mod script;

use std::{fs, io, path::PathBuf, sync::Arc};

use rhai::module_resolvers::FileModuleResolver;
use script::PolyScript;
use thiserror::Error;

use self::script::ScriptError;

pub struct PolyEngine {
    scripts: Vec<PolyScript>,
}

impl PolyEngine {
    pub fn load(script_dir: impl AsRef<str>) -> Result<Self, EngineError> {
        let mut engine = rhai::Engine::new();

        engine
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
        let script_path = PathBuf::from(script_dir.as_ref());
        let resolver = FileModuleResolver::new_with_path(script_path.clone());
        engine.set_module_resolver(resolver);

        // wrap in arc for use in scripts
        let engine = Arc::new(engine);

        // load all the scripts
        let mut scripts = Vec::new();
        for entry in fs::read_dir(script_path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let name = match path.file_stem() {
                Some(stem) => stem.to_string_lossy().to_string(),
                None => "blank".to_string(),
            };

            let content = fs::read_to_string(path)?;
            scripts.push(PolyScript::load(name, content, engine.clone())?);
        }

        // create and return engine
        Ok(Self { scripts })
    }

    // get all scripts currently loaded by the engine
    pub fn scripts(&self) -> &[PolyScript] {
        &self.scripts
    }
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("{0}")]
    IOError(io::Error),
    #[error("{0}")]
    ScriptError(ScriptError),
}

impl From<io::Error> for EngineError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ScriptError> for EngineError {
    fn from(value: ScriptError) -> Self {
        Self::ScriptError(value)
    }
}
