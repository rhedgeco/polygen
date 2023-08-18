use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::Serialize;
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

use crate::polyitems::PolyFn;

use super::{BuildResult, PolyBuild, PolyError, PolyType};

#[derive(Serialize)]
pub struct FnImpl {
    vis: bool,
    alias: String,
    function: PolyFn,
}

#[derive(Serialize)]
pub struct PolyImpl {
    name: String,
    functions: Vec<FnImpl>,
}

impl PolyImpl {
    pub fn build(item: &syn::ItemImpl) -> BuildResult<Self> {
        // create stream for assertions
        let mut assertions = TokenStream::new();

        // create the type for this impl
        let self_ty = &item.self_ty;
        let type_build = PolyType::build(&self_ty, None)?;
        let polytype = type_build.polyitem;
        assertions.extend(type_build.assertions);

        // extract the type into a name
        let name = match &polytype {
            PolyType::Named(name) => name.to_owned(),
            PolyType::Ref(_) | PolyType::RefMut(_) => {
                return Err(PolyError::spanned(
                    &self_ty,
                    "Polygen cannot export impl blocks over a reference type.",
                ));
            }
            PolyType::PtrMut(_) | PolyType::PtrConst(_) => {
                return Err(PolyError::spanned(
                    &self_ty,
                    "Polygen cannot export impl blocks over a pointer type.",
                ));
            }
        };

        // create an empty error to append to
        let mut errors = PolyError::empty();

        let mut functions = Vec::new();
        for impl_item in &item.items {
            use syn::ImplItem::*;
            match impl_item {
                Fn(syn::ImplItemFn { vis, sig, .. }) => {
                    // build inputs
                    let mut sig_inputs = Punctuated::<syn::FnArg, Token![,]>::new();
                    let mut pass_inputs = Punctuated::<syn::Ident, Token![,]>::new();
                    for sig_input in &sig.inputs {
                        use syn::FnArg::*;
                        let (new_input, pass_input) = match sig_input {
                            Receiver(rec) => {
                                let reference = match rec.reference {
                                    Some((and, _)) => Some(and),
                                    _ => None,
                                };
                                let mutability = rec.mutability.clone();
                                (
                                    syn::parse_quote!(__polygen_impl_self: #reference #mutability #self_ty),
                                    syn::Ident::new("__polygen_impl_self", rec.span()),
                                )
                            }
                            i @ Typed(pat) => {
                                let pass_input = match &*pat.pat {
                                    syn::Pat::Ident(ident) => ident.ident.clone(),
                                    _ => panic!("Polygen internal error"),
                                };

                                (i.clone(), pass_input)
                            }
                        };

                        sig_inputs.push(new_input);
                        pass_inputs.push(pass_input);
                    }

                    // build output
                    let output = match &sig.output {
                        out @ syn::ReturnType::Type(_, ty) => match ty.deref() {
                            syn::Type::Path(path) => match path.path.segments.last() {
                                Some(name) if name.into_token_stream().to_string() == "Self" => {
                                    syn::parse_quote!(-> #self_ty)
                                }
                                _ => out.clone(),
                            },
                            _ => out.clone(),
                        },
                        out => out.clone(),
                    };

                    let fn_name = sig.ident.to_string();
                    let export_name = format!("__polygen_impl_{name}_{fn_name}");
                    let export_ident = syn::Ident::new(&export_name, sig.ident.span());
                    let sig_ident = &sig.ident;
                    let unsafety = &sig.unsafety;
                    let item_fn: syn::ItemFn = syn::parse_quote! {
                        #[no_mangle]
                        #[doc(hidden)]
                        #unsafety extern "C" fn #export_ident(#sig_inputs) #output {
                            #self_ty::#sig_ident(#pass_inputs)
                        }
                    };
                    assertions.extend(quote!(#item_fn));

                    let function = match PolyFn::build(&item_fn) {
                        Ok(build) => {
                            assertions.extend(build.assertions);
                            build.polyitem
                        }
                        Err(error) => {
                            errors.merge(error);
                            continue;
                        }
                    };

                    functions.push(FnImpl {
                        vis: vis.to_token_stream().to_string() == "pub",
                        alias: fn_name,
                        function,
                    })
                }
                _ => errors.push_spanned(impl_item, "This impl item is not supported by polygen"),
            }
        }

        // fork if there are any errors
        errors.fork()?;

        PolyBuild::build(Self { name, functions }, assertions)
    }
}
