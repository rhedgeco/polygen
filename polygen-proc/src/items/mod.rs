mod polyfield;
mod polyfunction;
mod polyimpl;
mod polystruct;
mod polytype;
mod utils;

pub use polyfield::*;
pub use polyfunction::*;
pub use polyimpl::*;
pub use polystruct::*;
pub use polytype::*;
pub use utils::*;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
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
    Impl(PolyImpl),
}

impl ToTokens for PolyItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(item) => item.to_tokens(tokens),
            Self::Fn(item) => item.to_tokens(tokens),
            Self::Impl(item) => item.to_tokens(tokens),
        }
    }
}

impl PolyItem {
    pub fn build(item: &syn::Item) -> PolyResult<Self> {
        use syn::Item::*;
        match item {
            Struct(item) => Ok(Self::Struct(PolyStruct::build(item)?)),
            Fn(item) => Ok(Self::Fn(PolyFn::build(item)?)),
            Impl(item) => Ok(Self::Impl(PolyImpl::build(item)?)),
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
