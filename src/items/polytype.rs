use std::hash::Hash;

pub enum PolyType {
    Ref(&'static PolyType),
    RefMut(&'static PolyType),
    PtrMut(&'static PolyType),
    PtrConst(&'static PolyType),
    Struct(PolyStruct),
}

pub struct PolyStruct {
    pub ident: &'static str,
    pub module: &'static str,
    pub fields: &'static [PolyField],
}

impl Hash for PolyStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.module.hash(state);
    }
}

pub struct PolyField {
    pub name: &'static str,
    pub ty: PolyType,
}
