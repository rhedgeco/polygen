use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use rand::distributions::{Alphanumeric, DistString};
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

use super::PolyAttr;

pub fn polyimpl(_attrs: &PolyAttr, item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    // fail on generics
    if !item.generics.params.empty_or_trailing() {
        return quote_spanned! { item.generics.params.span() =>
            compile_error!("Generics are not supported by #[polygen] attribute");
        };
    }

    // get the self type for use later
    let self_ty = &item.self_ty;

    // generate random id to prevent exported name collisions
    // this can be replaced with the module name once available in proc macros
    let rand_id: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let mut exports = quote!();
    let mut polyfns = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    for implitem in &item.items {
        match implitem {
            syn::ImplItem::Fn(implfn) => {
                // fail on generics
                if !implfn.sig.generics.params.empty_or_trailing() {
                    return quote_spanned! { item.generics.params.span() =>
                        compile_error!("Generics are not supported by #[polygen] attribute");
                    };
                }

                let mut variables = proc_macro2::TokenStream::new();
                let mut into_params = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                let mut export_params = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                let mut polyfn_input = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                for input in &implfn.sig.inputs {
                    match input {
                        syn::FnArg::Receiver(rec) => match rec.reference {
                            Some(_) => {
                                export_params.push(quote_spanned! { rec.self_token.span() =>
                                    __polygen_self_ptr: *mut #self_ty
                                });
                                variables.append_all(quote_spanned! { rec.self_token.span() =>
                                    let __polygen_self_ref = unsafe { &mut *__polygen_self_ptr };
                                });
                                into_params.push(quote_spanned! { rec.self_token.span() =>
                                    __polygen_self_ref
                                });
                                polyfn_input.push(quote_spanned! { rec.self_token.span() =>
                                    ::polygen::items::FnInput {
                                        name: "self",
                                        ty: &<*mut #self_ty as ::polygen::__private::ExportedPolyStruct>::STRUCT
                                    }
                                });
                            }
                            None => {
                                export_params.push(quote_spanned! { rec.self_token.span() =>
                                    __polygen_self: <#self_ty as ::polygen::__private::ExportedPolyStruct>::ExportedType
                                });
                                into_params.push(quote_spanned! { rec.self_token.span() =>
                                    __polygen_self.into()
                                });
                                polyfn_input.push(quote_spanned! { rec.self_token.span() =>
                                    ::polygen::items::FnInput {
                                        name: "self",
                                        ty: &<#self_ty as ::polygen::__private::ExportedPolyStruct>::STRUCT
                                    }
                                });
                            }
                        },
                        syn::FnArg::Typed(typed) => {
                            let ty = &typed.ty;
                            let pat_ident = match &*typed.pat {
                                syn::Pat::Ident(ident) => &ident.ident,
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

                            export_params.push(quote_spanned! { ty.span() =>
                                #pat_ident: <#ty as ::polygen::__private::ExportedPolyStruct>::ExportedType
                            });
                            into_params.push(quote_spanned!( ty.span() => #pat_ident.into() ));
                            polyfn_input.push(quote_spanned! { ty.span() =>
                                ::polygen::items::FnInput {
                                    name: stringify!(#pat_ident),
                                    ty: &<#ty as ::polygen::__private::ExportedPolyStruct>::STRUCT,
                                }
                            });
                        }
                    }
                }

                let (output, polyout) = match &implfn.sig.output {
                    syn::ReturnType::Default => (proc_macro2::TokenStream::new(), quote!(None)),
                    syn::ReturnType::Type(_, ty) => {
                        let mut ty = ty;
                        if ty.to_token_stream().to_string() == "Self" {
                            ty = self_ty;
                        }

                        (
                            quote_spanned! { ty.span() =>
                                -> <#ty as ::polygen::__private::ExportedPolyStruct>::ExportedType
                            },
                            quote_spanned! { ty.span() =>
                                Some(<#ty as ::polygen::__private::ExportedPolyStruct>::STRUCT)
                            },
                        )
                    }
                };

                let ident = &implfn.sig.ident;
                let export_ident =
                    syn::Ident::new(&format!("__polygen_implfn_{ident}_{rand_id}"), ident.span());
                polyfns.push(quote! {
                    ::polygen::items::ImplFn {
                        name: stringify!(#ident),
                        export_name: stringify!(#export_ident),
                        params: ::polygen::items::FnParams {
                            inputs: &[#polyfn_input],
                            output: #polyout,
                        }
                    }
                });

                exports.append_all(quote! {
                    #[no_mangle]
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    #[allow(improper_ctypes_definitions)]
                    extern "C" fn #export_ident( #export_params ) #output {
                        #variables
                        #self_ty::#ident( #into_params ).into()
                    }
                });
            }
            _ => {
                return quote_spanned! { implitem.span() =>
                    compile_error!("This item is not supported by #[polygen]");
                }
            }
        }
    }

    quote! {
        #exports

        unsafe impl ::polygen::__private::ExportedPolyImpl for #self_ty {
            const IMPL: ::polygen::items::PolyImpl = ::polygen::items::PolyImpl {
                functions: &[#polyfns],
            };
        }
    }
}
