use proc_macro2::TokenStream;
use serde::Serialize;

use super::{PolyError, PolyResult};

#[derive(Serialize)]
pub struct PolyImpl {
    #[serde(skip)]
    assertions: TokenStream,
}

impl PolyImpl {
    pub fn assertions(&self) -> &TokenStream {
        &self.assertions
    }

    pub fn build(_item: &syn::ItemImpl) -> PolyResult<Self> {
        Err(PolyError::simple(
            "polygen impl support is still in development",
        ))
    }
}
