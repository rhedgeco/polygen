use std::sync::{Arc, Mutex};

use rhai::serde::to_dynamic;
use thiserror::Error;

pub struct PolyScript {
    name: String,
    ast: rhai::AST,
    engine: Arc<rhai::Engine>,
    store: Mutex<rhai::Dynamic>,
}

impl PolyScript {
    pub(super) fn load(
        name: impl AsRef<str>,
        content: impl AsRef<str>,
        engine: Arc<rhai::Engine>,
    ) -> Result<Self, ScriptError> {
        let name = name.as_ref();
        let ast = engine
            .compile(content)
            .map_err(|e| ScriptError::ParseError(name.into(), e))?;

        // validate render and process functions
        let mut render = false;
        let mut process = false;
        for f in ast.iter_functions() {
            if !render && f.name == "render" && f.params.len() == 0 {
                render = true;
            } else if !process && f.name == "process" && f.params.len() == 1 {
                process = true;
            } else if render && process {
                break;
            }
        }

        // return error if either function is missing
        if !render && !process {
            return Err(ScriptError::MissingProcessRender(name.into()));
        } else if !process {
            return Err(ScriptError::MissingProcess(name.into()));
        } else if !render {
            return Err(ScriptError::MissingRender(name.into()));
        }

        let dynamic = to_dynamic(rhai::Map::default()).unwrap();
        Ok(Self {
            name: name.into(),
            ast,
            engine,
            store: Mutex::new(dynamic),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn process(&self, item: rhai::Dynamic) -> Result<(), Box<rhai::EvalAltResult>> {
        let mut store_guard = self.store.lock().unwrap();
        let options = rhai::CallFnOptions::new().bind_this_ptr(&mut *store_guard);

        let mut scope = rhai::Scope::new();
        let package_name = std::env::var("CARGO_PKG_NAME").unwrap_or("pkg_error".to_string());
        scope.push_constant("PACKAGE_NAME", package_name);

        self.engine
            .call_fn_with_options(options, &mut scope, &self.ast, "process", (item,))?;

        Ok(())
    }

    pub fn render(&self) -> Result<String, Box<rhai::EvalAltResult>> {
        let mut store_guard = self.store.lock().unwrap();
        let options = rhai::CallFnOptions::new().bind_this_ptr(&mut *store_guard);

        let mut scope = rhai::Scope::new();
        let package_name = std::env::var("CARGO_PKG_NAME").unwrap_or("pkg_error".to_string());
        scope.push_constant("PACKAGE_NAME", package_name);

        self.engine
            .call_fn_with_options::<String>(options, &mut scope, &self.ast, "render", ())
    }
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Script '{0}' is missing 'process(item)' function.")]
    MissingProcess(String),
    #[error("Script '{0}' is missing 'render()' function.")]
    MissingRender(String),
    #[error("Script '{0}' is missing 'process(item)' and 'render()' function.")]
    MissingProcessRender(String),
    #[error("Error parsing script '{0}': {1}")]
    ParseError(String, rhai::ParseError),
}
