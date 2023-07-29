use quote::{__private::TokenStream, quote_spanned};
use serde::Serialize;
use syn::spanned::Spanned;

use super::PolyResult;

#[derive(Serialize)]
pub struct PolyType {
    name: String,
}

impl PolyType {
    pub fn new(item: &syn::Type) -> PolyResult<Self> {
        use syn::Type::*;
        match item {
            Path(path) => match path.path.segments.last() {
                Some(segment) => Ok(Self {
                    name: segment.ident.to_string(),
                }),
                None => Err(bad_type(item)),
            },
            item => Err(bad_type(&item)),
        }
    }
}

fn bad_type(span: &impl Spanned) -> TokenStream {
    quote_spanned! {span.span() =>
        compile_error!("This type is not supported by polygen.");
    }
}
