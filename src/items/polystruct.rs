use std::hash::Hash;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PolyStruct {
    pub module: &'static str,
    pub name: &'static str,
    pub fields: &'static [StructField],
    pub generics: &'static [PolyGeneric],
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PolyType {
    #[serde(rename = "primitive")]
    Primitive(&'static str),
    #[serde(rename = "struct")]
    Struct(PolyStruct),
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

#[derive(Debug, Clone, Copy, Serialize)]
pub struct StructField {
    pub name: &'static str,
    pub ty: FieldType,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum FieldType {
    #[serde(rename = "generic")]
    Generic(&'static str),
    #[serde(rename = "typed")]
    Typed(&'static PolyType),
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PolyGeneric {
    pub ident: &'static str,
    pub ty: &'static PolyType,
}
