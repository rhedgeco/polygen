use std::hash::Hash;

use super::PolyStruct;

#[derive(Debug, Clone, Copy)]
pub struct PolyFn {
    pub module: &'static str,
    pub name: &'static str,
    pub export_name: &'static str,
    pub params: FnParams,
}

impl Eq for PolyFn {}
impl PartialEq for PolyFn {
    fn eq(&self, other: &Self) -> bool {
        self.module == self.module && self.name == other.name
    }
}

impl Hash for PolyFn {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module.hash(state);
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FnParams {
    pub inputs: &'static [FnInput],
    pub output: Option<PolyStruct>,
}

#[derive(Debug, Clone, Copy)]
pub struct FnInput {
    pub name: &'static str,
    pub ty: &'static PolyStruct,
}
