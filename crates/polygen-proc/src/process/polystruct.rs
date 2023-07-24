use quote::{__private::TokenStream, quote};

pub fn polystruct(item: &syn::ItemStruct) -> TokenStream {
    let name = &item.ident;
    let (gen_impl, gen_type, gen_where) = item.generics.split_for_impl();

    // create initial output token stream
    let mut output = quote!(#item);

    // impl the 'exported_by_polygen' trait for this struct
    output = quote! {
        #output

        unsafe impl #gen_impl polygen::__private::exported_by_polygen for #name #gen_type
            #gen_where {}
    };

    // assert that all fields are also exported by polygen
    for field in &item.fields {
        let ty = &field.ty;
        output = quote! {
            #output

            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(item: T) {}
                fn __assert_exported(item: #ty) {
                    __accept_exported(item);
                }
            };
        };
    }

    output
}
