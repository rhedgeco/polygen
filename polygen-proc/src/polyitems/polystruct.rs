use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Serialize;

use crate::polyitems::PolyError;

use super::{BuildResult, PolyBuild, PolyField};

#[derive(Serialize)]
pub struct PolyStruct {
    vis: bool,
    name: String,
    attrs: Vec<String>,
    fields: Vec<PolyField>,
}

impl PolyStruct {
    pub fn build(item: &syn::ItemStruct) -> BuildResult<Self> {
        // return error if the struct has generics
        if !item.generics.params.empty_or_trailing() {
            return Err(PolyError::spanned(
                &item.generics.params,
                "Polygen does not support generic structs.",
            ));
        }

        // create an empty error to append to
        let mut errors = PolyError::empty();

        // create stream for assertions
        let mut assertions = TokenStream::new();

        // build fields and assert that all fields are also exported by polygen
        let mut fields = Vec::new();
        for (index, field) in item.fields.iter().enumerate() {
            match PolyField::build(index, field) {
                Ok(build) => {
                    assertions.extend(build.assertions);
                    fields.push(build.polyitem);
                }
                Err(error) => errors.merge(error),
            }
        }

        // fork if there are any errors
        errors.fork()?;

        // impl the 'exported_by_polygen' trait for this struct
        let name = &item.ident;
        assertions.extend(quote! {
            unsafe impl polygen::__private::exported_by_polygen for #name {}
        });

        // collect the attributes into a string list
        let mut attrs = Vec::new();
        for attr in &item.attrs {
            attrs.push(attr.meta.to_token_stream().to_string());
        }

        // complete the build and assertions
        PolyBuild::build(
            Self {
                vis: item.vis.to_token_stream().to_string() == "pub",
                name: item.ident.to_string(),
                attrs,
                fields,
            },
            assertions,
        )
    }
}
