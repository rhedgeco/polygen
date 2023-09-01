use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use rand::distributions::{Alphanumeric, DistString};
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

pub fn polyfn(_attr: TokenStream, item: &syn::ItemFn) -> proc_macro2::TokenStream {
    if !item.sig.generics.params.empty_or_trailing() {
        return quote_spanned! { item.sig.generics.params.span() =>
            compile_error!("Generics are not supported by #[polygen] attribute");
        };
    }

    // generate random id to prevent exported name collisions
    let rand_id: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let ident = &item.sig.ident;
    let export_ident = syn::Ident::new(&format!("__polygen_fn_{ident}_{rand_id}"), ident.span());
    let mut into_args = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    let mut fn_args = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    let mut polyfields = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    for input in &item.sig.inputs {
        use syn::FnArg as A;
        match input {
            A::Typed(typed) => {
                let ty = &typed.ty;
                let pat_ident = match &*typed.pat {
                    syn::Pat::Ident(ident) => ident,
                    pat => {
                        let message = format!(
                            "This pattern is unsupported by #[polygen]. \
                            Please use a literal name e.g. `literal_name: {}`",
                            ty.to_token_stream().to_string()
                        );

                        return quote_spanned! { pat.span() =>
                            compile_error!(#message);
                        };
                    }
                };

                into_args.push(quote_spanned!( ty.span() => #pat_ident.into() ));
                fn_args.push(quote_spanned! { ty.span() =>
                    #pat_ident: <#ty as ::polygen::__private::ExportedPolyType>::ExportedType
                });
                polyfields.push(quote_spanned! { ty.span() =>
                    ::polygen::items::PolyField {
                        name: stringify!(#pat_ident),
                        ty: <#ty as ::polygen::__private::ExportedPolyType>::TYPE,
                    }
                });
            }
            A::Receiver(_) => {} // do nothing here. compiler will catch this
        }
    }

    let (output, polyout) = match &item.sig.output {
        syn::ReturnType::Default => (proc_macro2::TokenStream::new(), quote!(None)),
        syn::ReturnType::Type(_, ty) => (
            quote_spanned! { ty.span() =>
                -> <#ty as ::polygen::__private::ExportedPolyType>::ExportedType
            },
            quote_spanned! { ty.span() =>
                Some(<#ty as ::polygen::__private::ExportedPolyType>::TYPE)
            },
        ),
    };

    let struct_ident = syn::Ident::new(&format!("__polygen_fn_{ident}"), ident.span());

    return quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct #struct_ident {}
        unsafe impl ::polygen::__private::ExportedPolyFn for #struct_ident {
            const FUNCTION: ::polygen::items::PolyFn = ::polygen::items::PolyFn {
                ident: stringify!(#ident),
                export_ident: stringify!(#export_ident),
                inputs: &[#polyfields],
                output: #polyout,
            };
        }

        #[no_mangle]
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn #export_ident( #fn_args ) #output {
            #ident( #into_args ).into()
        }
    };
}
