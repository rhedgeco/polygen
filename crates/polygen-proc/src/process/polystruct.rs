use quote::{__private::TokenStream, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

pub fn polystruct(item: &mut syn::ItemStruct) -> TokenStream {
    // create initial token stream
    let mut output = quote!();

    // early fail if the struct has generics
    if !item.generics.params.empty_or_trailing() {
        return quote_spanned! { item.generics.params.span() =>
            compile_error!("Polygen does not support generic types.");
            #item
        };
    }

    // check if the struct has a valid repr
    use syn::Meta::*;
    if !item.attrs.iter().any(|attr| match &attr.meta {
        List(syn::MetaList {
            path,
            delimiter: _,
            tokens: _,
            // nothing else needs to be checked
            // because only C and transparent ABIs are allowed on structs
        }) if path.to_token_stream().to_string() == format!("repr") => true,
        _ => false,
    }) {
        // add a #[repr(C)] hint if none exist
        item.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    // impl the 'exported_by_polygen' trait for this struct
    let name = &item.ident;
    output.extend(quote! {
        unsafe impl polygen::__private::exported_by_polygen for #name {}
    });

    // assert that all fields are also exported by polygen
    for field in &item.fields {
        let ty = &field.ty;
        output.extend(quote_spanned! { ty.span() =>
            const _: fn() = || {
                fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __accept_exported(_item: #ty) { __assert_exported(_item); }
            };
        });
    }

    // add the struct and return
    output.extend(quote!(#item));
    output
}
