use quote::{__private::TokenStream, quote, ToTokens};

pub fn polystruct(item: &syn::ItemStruct) -> TokenStream {
    // create initial token stream
    let mut output = quote!( #item );

    // fail if the struct has generics
    if !item.generics.params.empty_or_trailing() {
        return quote! {
            #output

            compile_error!("Polygen does not support generic types");
        };
    }

    // fail if the struct is not #[repr(C)]
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
        output = quote! {
            #output

            compile_error!("Polygen structs must use #[repr(C)] or #[repr(transparent)] abi");
        };
    }

    // impl the 'exported_by_polygen' trait for this struct
    let name = &item.ident;
    output = quote! {
        #output

        unsafe impl polygen::__private::exported_by_polygen for #name {}
    };

    // assert that all fields are also exported by polygen
    for field in &item.fields {
        let ty = &field.ty;
        output = quote! {
            #output

            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __assert_exported(_item: #ty) { __accept_exported(_item); }
            };
        };
    }

    output
}
