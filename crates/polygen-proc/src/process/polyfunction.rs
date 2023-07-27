use quote::{__private::TokenStream, quote, quote_spanned};
use syn::spanned::Spanned;

pub fn polyfunction(item: &syn::ItemFn) -> TokenStream {
    // create initial token stream
    let mut output = quote!();

    // fail if function contains generics
    if !item.sig.generics.params.empty_or_trailing() {
        output.extend(quote_spanned! { item.sig.generics.params.span() =>
            compile_error!("Polygen does not support generic functions.");
        });
    }

    // fail if function is not 'extern "C"'
    match &item.sig.abi {
        Some(syn::Abi {
            extern_token: _,
            name: Some(name),
        }) if name.value() == format!("C") => (), // successfully found C abi
        _ => {
            output.extend(quote! {
                compile_error!("Polygen functions must use 'extern \"C\"' abi.");
            });
        }
    }

    // assert that output type is exported by polygen
    use syn::ReturnType::*;
    if let Type(_, ty) = &item.sig.output {
        output.extend(quote_spanned! { ty.span() =>
            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __assert_exported(_item: #ty) { __accept_exported(_item); }
            };
        });
    }

    // assert that all input fields are also exported by polygen
    for input in &item.sig.inputs {
        use syn::FnArg::*;
        let typed = match input {
            Typed(typed) => typed,
            Receiver(reciever) => {
                output.extend(quote_spanned! { reciever.span() =>
                    compile_error!("Polygen doesnt support the 'self' parameter.");
                });
                continue;
            }
        };

        let ty = &typed.ty;
        output.extend(quote_spanned! { ty.span() =>
            const _: fn() = || {
                fn __accept_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __assert_exported(_item: #ty) { __accept_exported(_item); }
            };
        });
    }

    output.extend(quote!(#item));
    output
}
