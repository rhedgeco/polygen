use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::Serialize;

use super::{PolyError, PolyResult};

#[derive(Serialize)]
pub struct PolyImpl {
    #[serde(skip)]
    stream: TokenStream,
}

impl ToTokens for PolyImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.stream.clone())
    }
}

impl PolyImpl {
    pub fn build(_item: &syn::ItemImpl) -> PolyResult<Self> {
        Err(PolyError::simple(
            "polygen impl support is still in development",
        ))
    }
}
