use std::hash::Hash;

use super::{PolyField, PolyType};

#[derive(Debug, Clone, Copy)]
pub struct PolyFn {
    pub ident: &'static str,
    pub module: &'static str,
    pub export_ident: &'static str,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}

impl Eq for PolyFn {}
impl PartialEq for PolyFn {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.module == other.module
    }
}

impl Hash for PolyFn {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.module.hash(state);
    }
}
