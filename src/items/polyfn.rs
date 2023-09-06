use std::hash::Hash;

use super::{PolyField, PolyIdent, PolyType};

#[derive(Debug, Clone, Copy)]
pub struct PolyFn {
    pub ident: PolyIdent,
    pub inputs: &'static [PolyField],
    pub output: Option<PolyType>,
}

impl Eq for PolyFn {}
impl PartialEq for PolyFn {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Hash for PolyFn {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}
