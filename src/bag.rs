use indexmap::IndexSet;

use crate::{
    __private::ExportedPolyFn,
    items::{PolyFn, PolyStruct},
};

#[derive(Debug, Default)]
pub struct PolyBag {
    structs: IndexSet<&'static PolyStruct>,
    functions: Vec<&'static PolyFn>,
}

impl PolyBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: ExportedPolyFn>(&mut self) {
        self.functions.push(&T::FUNCTION);
        for input in T::FUNCTION.inputs {
            self.structs.insert(input.ty.root_struct());
        }
    }

    pub fn structs(&self) -> impl Iterator<Item = &&'static PolyStruct> {
        self.structs.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &&'static PolyFn> {
        self.functions.iter()
    }
}
