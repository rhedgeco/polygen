use std::hash::Hash;

use super::PolyIdent;

#[derive(Debug, Clone, Copy)]
pub enum PolyType {
    Ref(&'static PolyType),
    RefMut(&'static PolyType),
    PtrMut(&'static PolyType),
    PtrConst(&'static PolyType),
    Struct(PolyStruct),
}

impl PolyType {
    pub fn nesting_depth(&self) -> usize {
        match self {
            Self::Struct(_) => 1,
            Self::Ref(t) | Self::RefMut(t) | Self::PtrMut(t) | Self::PtrConst(t) => {
                1 + t.nesting_depth()
            }
        }
    }

    pub fn root_struct(&self) -> &PolyStruct {
        match self {
            Self::Struct(s) => s,
            Self::Ref(t) | Self::RefMut(t) | Self::PtrMut(t) | Self::PtrConst(t) => t.root_struct(),
        }
    }
}

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
    pub ty: PolyType,
}
