use indexmap::{IndexMap, IndexSet};
use serde::Serialize;

use crate::{
    __private::ExportedPolyFn,
    items::{FieldType, PolyFn, PolyStruct, PolyType},
};

#[derive(Debug, Serialize)]
pub struct PolyBag {
    #[serde(flatten)]
    root_module: PolyMod,
}

impl PolyBag {
    pub fn new(name: impl Into<String>) -> Self {
        let root_module = PolyMod::build(name);
        Self { root_module }
    }

    pub fn root_module(&self) -> &PolyMod {
        &self.root_module
    }

    pub fn register_function<T: ExportedPolyFn>(mut self) -> Self {
        // get reference to the function
        let func = &T::FUNCTION;

        // register all its inputs
        for input in func.params.inputs {
            if let PolyType::Struct(s) = input.ty {
                self.insert_struct_data(s);
            }
        }

        // register its output
        if let Some(out) = &func.params.output {
            if let PolyType::Struct(s) = out {
                self.insert_struct_data(s);
            }
        }

        // insert the function
        let target_mod = self.root_module.get_target_mod(func.module);
        target_mod.functions.insert(*func);
        self
    }

    fn insert_struct_data(&mut self, s: &PolyStruct) {
        // register all nested structs
        for field in s.fields {
            if let FieldType::Typed(PolyType::Struct(s)) = field.ty {
                self.insert_struct_data(s);
            }
        }

        // register all generic types
        for generic in s.generics {
            if let PolyType::Struct(s) = generic.ty {
                self.insert_struct_data(s);
            }
        }

        // register current struct
        let target_mod = self.root_module.get_target_mod(s.module);
        target_mod.structs.insert(*s);
    }
}

#[derive(Debug, Serialize)]
pub struct PolyMod {
    name: String,
    functions: IndexSet<PolyFn>,
    modules: IndexMap<String, PolyMod>,
    structs: IndexSet<PolyStruct>,
}

impl PolyMod {
    fn build(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structs: Default::default(),
            functions: Default::default(),
            modules: Default::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn structs(&self) -> impl Iterator<Item = &PolyStruct> {
        self.structs.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &PolyFn> {
        self.functions.iter()
    }

    pub fn modules(&self) -> impl Iterator<Item = &PolyMod> {
        self.modules.values()
    }

    fn get_target_mod(&mut self, mod_path: impl AsRef<str>) -> &mut PolyMod {
        let mut target_mod = self;
        for mod_name in mod_path.as_ref().split("::").skip(1) {
            use indexmap::map::Entry as E;
            target_mod = match target_mod.modules.entry(mod_name.into()) {
                E::Occupied(e) => e.into_mut(),
                E::Vacant(e) => e.insert(PolyMod::build(mod_name)),
            };
        }

        target_mod
    }
}
