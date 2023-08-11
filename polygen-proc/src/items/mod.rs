mod polyfield;
mod polyfunction;
mod polystruct;
mod polytype;

pub use polyfield::*;
pub use polyfunction::*;
pub use polystruct::*;
pub use polytype::*;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use rhai::serde::to_dynamic;
use syn::spanned::Spanned;

pub type PolyResult<T> = Result<T, PolyError>;

pub struct PolyError {
    stream: TokenStream,
}

#[derive(Default)]
pub struct PolyErrorBuilder {
    error: Option<PolyError>,
}

impl PolyErrorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, error: PolyError) {
        match &mut self.error {
            Some(e) => e.merge(error),
            e => *e = Some(error),
        }
    }

    pub fn fork(self) -> Result<Self, PolyError> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self),
        }
    }
}

impl PolyError {
    pub fn simple(message: impl AsRef<str>) -> Self {
        let message = message.as_ref();
        Self {
            stream: quote!( compile_error!(#message); ),
        }
    }

    pub fn build(span: &impl Spanned, message: impl AsRef<str>) -> Self {
        let message = message.as_ref();
        let stream = quote_spanned! ( span.span() => compile_error!(#message); );
        Self { stream }
    }

    pub fn stream(&self) -> &TokenStream {
        &self.stream
    }

    fn merge(&mut self, other: Self) {
        self.stream.extend(other.stream);
    }
}

pub enum PolyItem {
    Struct(PolyStruct),
    Fn(PolyFn),
}

impl PolyItem {
    pub fn build(item: &mut syn::Item) -> PolyResult<Self> {
        use syn::Item::*;
        match item {
            Struct(item) => Ok(Self::Struct(PolyStruct::build(item)?)),
            Fn(item) => Ok(Self::Fn(PolyFn::build(item)?)),
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

    pub fn assertions(&self) -> &TokenStream {
        match self {
            Self::Struct(item) => item.assertions(),
            Self::Fn(item) => item.assertions(),
        }
    }
}
