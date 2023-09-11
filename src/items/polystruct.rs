use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
pub struct PolyStruct {
    pub module: &'static str,
    pub name: &'static str,
    pub fields: &'static [StructField],
    pub generics: &'static [PolyGeneric],
}

impl Eq for PolyStruct {}
impl PartialEq for PolyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.module == self.module && self.name == other.name
    }
}

impl Hash for PolyStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module.hash(state);
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StructField {
    pub name: &'static str,
    pub ty: FieldType,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Generic(&'static str),
    Typed(&'static PolyStruct),
}

#[derive(Debug, Clone, Copy)]
pub struct PolyGeneric {
    pub ident: &'static str,
    pub ty: &'static PolyStruct,
}
