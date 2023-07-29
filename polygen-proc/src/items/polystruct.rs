use quote::{__private::TokenStream, quote, quote_spanned, ToTokens};
use serde::Serialize;
use syn::spanned::Spanned;

use super::PolyField;

#[derive(Serialize)]
pub struct PolyStruct {
    vis: bool,
    name: String,
    fields: Vec<PolyField>,
}

impl PolyStruct {
    pub fn build(mut item: syn::ItemStruct) -> (Self, TokenStream) {
        // create initial stream
        let mut stream = quote!();

        // force this struct to use #[repr(C)]
        // this will make any other usage an error
        item.attrs.push(syn::parse_quote!(#[repr(C)]));

        // add error if the struct has generics
        if !item.generics.params.empty_or_trailing() {
            stream.extend(quote_spanned! { item.generics.params.span() =>
                compile_error!("Polygen does not support generic structs.");
            });
        }

        // impl the 'exported_by_polygen' trait for this struct
        let name = &item.ident;
        stream.extend(quote! {
            unsafe impl polygen::__private::exported_by_polygen for #name {}
        });

        // build fields and assert that all fields are also exported by polygen
        let mut fields = Vec::new();
        for (index, field) in item.fields.iter().enumerate() {
            let ty = &field.ty;
            stream.extend(quote_spanned! { field.span() =>
                const _: fn() = || {
                    fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                    fn __accept_exported(_item: #ty) { __assert_exported(_item); }
                };
            });

            match PolyField::new(index, field) {
                Ok(field) => fields.push(field),
                Err(error) => stream.extend(error),
            }
        }

        // append item to the end of the stream
        stream.extend(quote!( #item ));

        (
            Self {
                vis: item.vis.to_token_stream().to_string() == "pub",
                name: item.ident.to_string(),
                fields,
            },
            stream,
        )
    }
}
