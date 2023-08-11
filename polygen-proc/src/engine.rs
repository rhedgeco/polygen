mod functions;
mod script;

use std::{
    fs, io,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use proc_macro2::TokenStream;
use quote::quote;
use rhai::module_resolvers::FileModuleResolver;
use script::PolyScript;
use thiserror::Error;

use crate::items::{PolyError, PolyErrorBuilder, PolyItem, PolyResult};

use self::script::ScriptError;

pub const SCRIPT_DIR: &str = "./polygen";
pub const OUTPUT_DIR: &str = "./target/polygen";

pub struct PolyEngine {
    scripts: Vec<PolyScript>,
}

impl PolyEngine {
    /// Gets and initializes a global instance of the polygen engine
    pub fn get_instance() -> Result<&'static Self, TokenStream> {
        static ENGINE: OnceLock<Result<PolyEngine, EngineError>> = OnceLock::new();
        match ENGINE.get_or_init(|| Self::load()) {
            Ok(instance) => Ok(instance),
            Err(error) => {
                let message = format!("Polygen Load Error: {error}");
                Err(quote!( compile_error!(#message); ))
            }
        }
    }

    fn load() -> Result<Self, EngineError> {
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
            .register_fn("regex_match", functions::regex_match)
            .register_fn("docformat", functions::docformat);

        // set up module resolvers for engine
        let script_path = PathBuf::from(SCRIPT_DIR);
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

    pub fn process(&self, item: &PolyItem) -> PolyResult<()> {
        let mut errors = PolyErrorBuilder::new();
        for script in &self.scripts {
            if let Err(mut error) = script.process(item) {
                // loop unwrap error to get to root error
                use rhai::EvalAltResult::*;
                while let ErrorInFunctionCall(_, _, inner, _) = *error {
                    error = inner;
                }

                // process runtime errors to make them prettier
                let error = match *error {
                    ErrorRuntime(e, _) => format!("{e}"),
                    error => format!("{error}"),
                };

                // combine output with new error message
                let message = format!("{} - {}", script.name(), error);
                errors.merge(PolyError::simple(message))
            }
        }

        errors.fork()?;
        Ok(())
    }
}

#[derive(Debug, Error)]
enum EngineError {
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
