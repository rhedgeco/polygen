use quote::{__private::TokenStream, quote};

pub fn polystruct(polystruct: &syn::ItemStruct) -> TokenStream {
    let name = &polystruct.ident;
    let (gen_impl, gen_type, gen_where) = polystruct.generics.split_for_impl();

    let mut field_assertions = quote!();
    for field in &polystruct.fields {
        let ty = &field.ty;
        let assertion = quote! {
            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(item: T) {}
                fn __assert_exported(item: #ty) {
                    __accept_exported(item);
                }
            };
        };

        field_assertions.extend(assertion);
    }

    quote! {
        // add the original struct unchanged
        #polystruct

        // add field assertions
        #field_assertions

        // implement a marker type for other field assertions
        unsafe impl #gen_impl polygen::__private::exported_by_polygen for #name #gen_type #gen_where {}

        // assert that this type is FFI safe
        const _: fn() = || {
            extern "C" {
                #[deny(improper_ctypes)]
                fn __assert_ffi_safe #gen_impl (item: #name #gen_type) #gen_where;
            }
        };
    }
}
