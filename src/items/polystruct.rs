use std::hash::Hash;

use super::PolyIdent;

#[derive(Debug, Clone, Copy)]
pub struct PolyStruct {
    pub ident: PolyIdent,
    pub fields: &'static [PolyField],
}

impl Eq for PolyStruct {}
impl PartialEq for PolyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Hash for PolyStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PolyField {
    pub name: &'static str,
    pub ty: PolyStruct,
}
