use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use rand::distributions::{Alphanumeric, DistString};
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

pub fn polyimpl(_attr: TokenStream, item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    if !item.generics.params.empty_or_trailing() {
        return quote_spanned! { item.generics.params.span() =>
            compile_error!("Impl generics are not supported by #[polygen] attribute");
        };
    }

    // generate random id to prevent exported name collisions
    // this can be replaced with the module name once available in proc macros
    let rand_id: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 6);

    let self_ty = &item.self_ty;
    let mut export_functions = proc_macro2::TokenStream::new();
    let mut functions = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    for impl_item in &item.items {
        use syn::ImplItem as I;
        match impl_item {
            I::Fn(itemfn) => {
                let ident = &itemfn.sig.ident;
                let export_ident =
                    syn::Ident::new(&format!("__polygen_implfn_{ident}_{rand_id}"), ident.span());
                let mut into_args = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                let mut fn_args = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                let mut polyfields = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
                for input in &itemfn.sig.inputs {
                    use syn::FnArg as A;
                    match input {
                        A::Receiver(rec) => {
                            let reference = match &rec.reference {
                                None => quote!(),
                                Some((and, _)) => and.to_token_stream(),
                            };
                            let mutability = rec.mutability;
                            fn_args.push(quote_spanned! { rec.span() =>
                                __polygen_self: <#reference #mutability #self_ty as ::polygen::__private::ExportedPolyType>::ExportedType
                            });
                            into_args.push(quote_spanned! { rec.span() =>
                                __polygen_self.into()
                            });
                            polyfields.push(quote_spanned! { rec.span() =>
                                ::polygen::items::PolyField {
                                    name: "self",
                                    ty: <#reference #mutability #self_ty as ::polygen::__private::ExportedPolyType>::TYPE,
                                }
                            });
                        }
                        A::Typed(pat_ty) => {
                            let ty = &pat_ty.ty;
                            let ty = match parse_type(ty, self_ty) {
                                Ok(ty) => ty,
                                Err(msg) => {
                                    return quote_spanned!(ty.span() => compile_error!(#msg);)
                                }
                            };

                            let pat_ident = match &*pat_ty.pat {
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
                    }
                }

                let (output, polyout) = match &itemfn.sig.output {
                    syn::ReturnType::Default => (proc_macro2::TokenStream::new(), quote!(None)),
                    syn::ReturnType::Type(_, ty) => {
                        let ty = match parse_type(ty, self_ty) {
                            Ok(ty) => ty,
                            Err(msg) => return quote_spanned!(ty.span() => compile_error!(#msg);),
                        };

                        (
                            quote_spanned! { ty.span() =>
                                -> <#ty as ::polygen::__private::ExportedPolyType>::ExportedType
                            },
                            quote_spanned! { ty.span() =>
                                Some(<#ty as ::polygen::__private::ExportedPolyType>::TYPE)
                            },
                        )
                    }
                };

                export_functions.append_all(quote_spanned! { itemfn.sig.span() =>
                    #[no_mangle]
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    extern "C" fn #export_ident(#fn_args) #output {
                        #self_ty::#ident(#into_args).into()
                    }
                });
                functions.push(quote! {
                    ::polygen::items::ImplFn {
                        name: stringify!(#ident),
                        export_name: stringify!(#export_ident),
                        inputs: &[#polyfields],
                        output: #polyout,
                    }
                });
            }
            _ => {
                return quote_spanned! { impl_item.span() =>
                    compile_error!("This impl item is not supported by #[polygen]");
                }
            }
        }
    }

    quote! {
        #export_functions
        unsafe impl ::polygen::__private::ExportedPolyImpl for #self_ty {
            const IMPL: ::polygen::items::PolyImpl = ::polygen::items::PolyImpl {
                functions: &[#functions],
            };
        }
    }
}

fn parse_type(
    ty: &Box<syn::Type>,
    self_ty: &Box<syn::Type>,
) -> Result<Box<syn::Type>, &'static str> {
    use syn::Type as T;
    match ty.as_ref() {
        T::Path(path) => {
            if path.to_token_stream().to_string() == "Self" {
                Ok(self_ty.clone())
            } else {
                Ok(ty.clone())
            }
        }
        T::Ptr(ptr) => {
            let inner = parse_type(&ptr.elem, self_ty)?;
            let mut ptr = ptr.clone();
            ptr.elem = inner;
            Ok(Box::new(T::Ptr(ptr)))
        }
        T::Reference(reference) => {
            let inner = parse_type(&reference.elem, self_ty)?;
            let mut reference = reference.clone();
            reference.elem = inner;
            Ok(Box::new(T::Reference(reference)))
        }
        _ => Err("This type is not supported by #[polygen]"),
    }
}
