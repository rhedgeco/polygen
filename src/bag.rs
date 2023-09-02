use indexmap::{IndexMap, IndexSet};

use crate::{
    __private::ExportedPolyFn,
    items::{PolyFn, PolyStruct},
};

pub struct PolyBag {
    module: PolyMod,
}

impl PolyBag {
    pub fn new(name: impl Into<String>) -> Self {
        let module = PolyMod::build(name);
        Self { module }
    }

    pub fn root_module(&self) -> &PolyMod {
        &self.module
    }

    pub fn register_function<T: ExportedPolyFn>(mut self) -> Self {
        // get reference to the function
        let func = &T::FUNCTION;

        // register all its inputs
        for input in func.inputs {
            let root_struct = input.ty.root_struct();
            let target_mod = self.module.get_target_mod(root_struct.module);
            target_mod.structs.insert(*root_struct);
        }

        // register its output
        if let Some(out) = &func.output {
            let root_struct = out.root_struct();
            let target_mod = self.module.get_target_mod(root_struct.module);
            target_mod.structs.insert(*root_struct);
        }

        // insert the function
        let target_mod = self.module.get_target_mod(func.module);
        target_mod.functions.insert(*func);
        self
    }
}

#[derive(Debug)]
pub struct PolyMod {
    name: String,
    structs: IndexSet<PolyStruct>,
    functions: IndexSet<PolyFn>,
    modules: IndexMap<String, PolyMod>,
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
