use quote::{__private::TokenStream, quote};

pub fn polyfunction(item: &syn::ItemFn) -> TokenStream {
    // create initial token stream
    let mut output = quote!( #item );

    // fail if function contains generics
    if !item.sig.generics.params.empty_or_trailing() {
        return quote! {
            #output

            compile_error!("Polygen does not support generic functions as they are not FFI safe.");
        };
    }

    // fail if function is not 'extern "C"'
    match &item.sig.abi {
        Some(syn::Abi {
            extern_token: _,
            name: Some(name),
        }) if name.value() == format!("C") => (), // successfully found C abi
        _ => {
            output = quote! {
                #output
                compile_error!("Polygen functions must use 'extern \"C\"' abi");
            }
        }
    }

    // assert that output type is exported by polygen
    use syn::ReturnType::*;
    if let Type(_, ty) = &item.sig.output {
        output = quote! {
            #output

            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __assert_exported(_item: #ty) { __accept_exported(_item); }
            };
        };
    }

    // assert that all input fields are also exported by polygen
    for input in &item.sig.inputs {
        use syn::FnArg::*;
        let typed = match input {
            Typed(typed) => typed,
            Receiver(_) => {
                output = quote! {
                    #output

                    compile_error!("Polygen doesnt support the 'self' parameter");
                };
                continue;
            }
        };

        let ty = &typed.ty;
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
