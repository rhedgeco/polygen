use quote::{__private::TokenStream, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

pub fn polyfunction(item: &mut syn::ItemFn) -> TokenStream {
    // create initial token stream
    let mut output = quote!();

    // fail if function contains generics
    if !item.sig.generics.params.empty_or_trailing() {
        return quote_spanned! { item.sig.generics.params.span() =>
            compile_error!("Polygen does not support generic functions.");
            #item
        };
    }

    // check for #[no_mangle] attribute
    use syn::Meta::*;
    if !item.attrs.iter().any(|attr| match &attr.meta {
        Path(path) if path.into_token_stream().to_string() == "no_mangle" => true,
        _ => false,
    }) {
        // add a #[no_mangle] hint
        item.attrs.push(syn::parse_quote!(#[no_mangle]));
    }

    // check for 'extern "C"' abi
    match &item.sig.abi {
        Some(syn::Abi {
            extern_token: _,
            name: Some(name),
        }) if name.value() == format!("C") => (), // successfully found C abi
        _ => {
            // add extern C if it was not there already
            item.sig.abi = syn::parse_quote!(extern "C");
        }
    }

    // assert that output type is exported by polygen
    use syn::ReturnType::*;
    if let Type(_, ty) = &item.sig.output {
        output.extend(quote_spanned! { ty.span() =>
            const _: fn() = || {
                fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __accept_exported(_item: #ty) { __assert_exported(_item); }
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
                fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                fn __accept_exported(_item: #ty) { __assert_exported(_item); }
            };
        });
    }

    // add the function and return
    output.extend(quote!(#item));
    output
}
