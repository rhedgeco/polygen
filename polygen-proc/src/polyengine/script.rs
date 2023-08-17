use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use rhai::serde::to_dynamic;
use thiserror::Error;

use crate::polyitems::PolyItem;

static PACKAGE_NAME: Lazy<String> =
    Lazy::new(|| std::env::var("CARGO_PKG_NAME").unwrap_or("no_package_name".to_string()));

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

        // validate that script has a render function
        if !ast
            .iter_functions()
            .any(|f| f.name == "render" && f.params.is_empty())
        {
            return Err(ScriptError::MissingRender(name.into()));
        }

        // create dynamic map to be used as 'this' parameter later
        let dynamic = to_dynamic(rhai::Map::default()).unwrap();
        let store = Mutex::new(dynamic);

        // create and return script
        Ok(Self {
            name: name.into(),
            ast,
            engine,
            store,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn process(&self, item: &PolyItem) -> Result<(), Box<rhai::EvalAltResult>> {
        let (name, item) = item.as_dynamic();
        self.call(name, (item,))
    }

    pub fn render(&self) -> Result<String, Box<rhai::EvalAltResult>> {
        self.call("render", ())
    }

    fn call<T: Send + Sync + Clone + 'static>(
        &self,
        name: impl AsRef<str>,
        args: impl rhai::FuncArgs,
    ) -> Result<T, Box<rhai::EvalAltResult>> {
        let mut store_guard = self.store.lock().unwrap();
        let mut scope = rhai::Scope::new();
        scope.push_constant("PACKAGE_NAME", PACKAGE_NAME.to_string());
        self.engine.call_fn_with_options(
            rhai::CallFnOptions::new().bind_this_ptr(&mut *store_guard),
            &mut scope,
            &self.ast,
            name,
            args,
        )
    }
}

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Script '{0}' is missing 'render(item)' function.")]
    MissingRender(String),
    #[error("Error parsing script '{0}': {1}")]
    ParseError(String, rhai::ParseError),
}
