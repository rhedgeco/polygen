use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use rhai::serde::to_dynamic;
use serde::Serialize;
use syn::spanned::Spanned;

use super::{PolyError, PolyErrorBuilder, PolyField, PolyItem, PolyResult};

#[derive(Serialize)]
pub struct PolyStruct {
    vis: bool,
    name: String,
    attrs: Vec<String>,
    fields: Vec<PolyField>,

    #[serde(skip)]
    assertions: TokenStream,
}

impl PolyStruct {
    pub fn assertions(&self) -> &TokenStream {
        &self.assertions
    }

    pub fn build(item: &syn::ItemStruct) -> PolyResult<Self> {
        // add error if the struct has generics
        if !item.generics.params.empty_or_trailing() {
            return Err(PolyError::build(
                &item.generics.params,
                "Polygen does not support generic structs.",
            ));
        }

        // create assertion stream for the struct
        let mut assertions = TokenStream::new();

        // impl the 'exported_by_polygen' trait for this struct
        let name = &item.ident;
        assertions.extend(quote! {
            unsafe impl polygen::__private::exported_by_polygen for #name {}
        });

        // create error builder
        let mut errors = PolyErrorBuilder::new();

        // build fields and assert that all fields are also exported by polygen
        let mut fields = Vec::new();
        for (index, field) in item.fields.iter().enumerate() {
            let ty = &field.ty;
            assertions.extend(quote_spanned! { field.span() =>
                const _: fn() = || {
                    fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                    fn __accept_exported(_item: #ty) { __assert_exported(_item); }
                };
            });

            match PolyField::new(index, field) {
                Ok(field) => fields.push(field),
                Err(error) => errors.merge(error),
            }
        }

        // fork if there are any errors
        errors.fork()?;

        // collect the attributes into a string list
        let mut attrs = Vec::new();
        for attr in &item.attrs {
            attrs.push(attr.meta.to_token_stream().to_string());
        }

        // return the poly struct
        Ok(Self {
            vis: item.vis.to_token_stream().to_string() == "pub",
            name: item.ident.to_string(),
            attrs,
            fields,
            assertions,
        })
    }
}
