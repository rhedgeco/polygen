use rhai::serde::to_dynamic;

use super::{BuildResult, PolyError, PolyFn, PolyImpl, PolyStruct};

pub enum PolyItem {
    Struct(PolyStruct),
    Fn(PolyFn),
    Impl(PolyImpl),
}

impl PolyItem {
    pub fn build(item: &syn::Item) -> BuildResult<Self> {
        match item {
            syn::Item::Struct(item) => Ok(PolyStruct::build(item)?.map(|item| Self::Struct(item))),
            syn::Item::Fn(item) => Ok(PolyFn::build(item)?.map(|item| Self::Fn(item))),
            syn::Item::Impl(item) => Ok(PolyImpl::build(item)?.map(|item| Self::Impl(item))),
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
            Self::Impl(item) => (
                "impl",
                to_dynamic(item).expect("Internal Error: Impl dynamic conversion."),
            ),
        }
    }
}
