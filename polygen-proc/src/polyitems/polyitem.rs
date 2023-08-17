use rhai::serde::to_dynamic;

use super::{BuildResult, PolyError, PolyFn, PolyStruct};

pub enum PolyItem {
    Struct(PolyStruct),
    Fn(PolyFn),
}

impl PolyItem {
    pub fn build(item: &syn::Item) -> BuildResult<Self> {
        match item {
            syn::Item::Struct(item) => PolyStruct::build(item)?.map(|item| Self::Struct(item)),
            syn::Item::Fn(item) => PolyFn::build(item)?.map(|item| Self::Fn(item)),
            _ => Err(PolyError::simple("This item is unsupported by polygen")),
        }
    }

    pub fn as_dynamic(&self) -> (&str, rhai::Dynamic) {
        match self {
            Self::Struct(item) => (
                "struct",
                to_dynamic(item).expect("Internal Error: Struct dynamic conversion."),
            ),
            Self::Fn(item) => (
                "function",
                to_dynamic(item).expect("Internal Error: Function dynamic conversion."),
            ),
        }
    }
}
