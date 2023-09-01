use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
pub enum PolyType {
    Ref(&'static PolyType),
    RefMut(&'static PolyType),
    PtrMut(&'static PolyType),
    PtrConst(&'static PolyType),
    Struct(PolyStruct),
}

impl PolyType {
    pub fn root_struct(&self) -> &PolyStruct {
        match self {
            Self::Struct(s) => s,
            Self::Ref(t) | Self::RefMut(t) | Self::PtrMut(t) | Self::PtrConst(t) => t.root_struct(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PolyStruct {
    pub ident: &'static str,
    pub module: &'static str,
    pub fields: &'static [PolyField],
}

impl Eq for PolyStruct {}
impl PartialEq for PolyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.module == other.module
    }
}

impl Hash for PolyStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.module.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PolyField {
    pub name: &'static str,
    pub ty: PolyType,
}
