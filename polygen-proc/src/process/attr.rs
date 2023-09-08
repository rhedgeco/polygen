use std::ops::Deref;

use proc_macro2::Ident;
use syn::{parse::Parse, punctuated::Punctuated, Token};

pub struct PolyAttr {
    items: Vec<String>,
}

impl Deref for PolyAttr {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl Parse for PolyAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let idents = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let mut items = Vec::with_capacity(idents.len());
        for ident in idents.into_iter() {
            items.push(ident.to_string());
        }

        Ok(Self { items })
    }
}
