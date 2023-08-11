use quote::{__private::TokenStream, quote_spanned, ToTokens};
use serde::Serialize;
use syn::spanned::Spanned;

use crate::items::{PolyError, PolyErrorBuilder};

use super::{PolyResult, PolyType};

#[derive(Serialize)]
struct FnArg {
    name: String,
    r#type: PolyType,
}

impl FnArg {
    pub fn new(item: &syn::FnArg) -> PolyResult<Self> {
        use syn::FnArg::*;
        let typed = match item {
            Typed(typed) => typed,
            Receiver(rec) => {
                return Err(PolyError::build(
                    rec,
                    "Polygen functions don't support the 'self' parameter.",
                ));
            }
        };

        let r#type = PolyType::new(&typed.ty)?;

        use syn::Pat::*;
        let name = match &*typed.pat {
            Ident(ident) => ident.ident.to_string(),
            pat => {
                return Err(PolyError::build(
                    pat,
                    "This pattern is not supported by polygen.",
                ));
            }
        };

        Ok(Self { name, r#type })
    }
}

// #[derive(Serialize)]
// pub struct PolyFn {
//     vis: bool,
//     name: String,
//     inputs: Vec<FnArg>,
//     output: Option<PolyType>,
// }

// impl PolyFn {
//     pub fn build(mut item: syn::ItemFn) -> (Self, TokenStream) {
//         // create initial stream
//         let mut stream = quote!();

//         // add error if the function defines an abi
//         match &item.sig.abi {
//             Some(_) => stream.extend(quote_spanned! { item.sig.abi.span() =>
//                 compile_error!("Polygen function should not declare an 'extern' abi. \
//                     Polygen functions will always be 'extern \"C\"'");
//             }),
//             _ => (),
//         }

//         // force ABI to be 'extern C'
//         item.sig.abi = Some(syn::parse_quote!(extern "C"));

//         // check for #[no_mangle] attribute
//         use syn::Meta::*;
//         if !item.attrs.iter().any(|attr| match &attr.meta {
//             Path(path) if path.into_token_stream().to_string() == "no_mangle" => true,
//             _ => false,
//         }) {
//             // add a #[no_mangle] hint
//             item.attrs.push(syn::parse_quote!(#[no_mangle]));
//         }

//         // fail if function contains generics
//         if !item.sig.generics.params.empty_or_trailing() {
//             stream.extend(quote_spanned! { item.sig.generics.params.span() =>
//                 compile_error!("Polygen does not support generic functions.");
//                 #item
//             });
//         }

//         // assert that output type is exported by polygen
//         use syn::ReturnType::*;
//         if let Type(_, ty) = &item.sig.output {
//             stream.extend(quote_spanned! { ty.span() =>
//                 const _: fn() = || {
//                     fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
//                     fn __accept_exported(_item: #ty) { __assert_exported(_item); }
//                 };
//             });
//         }

//         // assert that all input fields are also exported by polygen
//         let mut inputs = Vec::new();
//         for input in &item.sig.inputs {
//             match FnArg::new(input) {
//                 Ok(arg) => inputs.push(arg),
//                 Err(error) => stream.extend(error),
//             }

//             use syn::FnArg::*;
//             match input {
//                 Typed(typed) => {
//                     let ty = &typed.ty;
//                     stream.extend(quote_spanned! { ty.span() =>
//                     const _: fn() = || {
//                         fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
//                         fn __accept_exported(_item: #ty) { __assert_exported(_item); }
//                     };
//                 });
//                 }
//                 _ => (),
//             }
//         }

//         // build the output type
//         let output = match &item.sig.output {
//             Default => None,
//             Type(_, ty) => match PolyType::new(ty) {
//                 Ok(ty) => Some(ty),
//                 Err(error) => {
//                     stream.extend(error);
//                     None
//                 }
//             },
//         };

//         // append item to the end
//         stream.extend(quote!( #item ));

//         (
//             Self {
//                 vis: item.vis.to_token_stream().to_string() == "pub",
//                 name: item.sig.ident.to_string(),
//                 inputs,
//                 output,
//             },
//             stream,
//         )
//     }
// }

#[derive(Serialize)]
pub struct PolyFn {
    vis: bool,
    abi: Option<String>,
    name: String,
    attrs: Vec<String>,
    inputs: Vec<FnArg>,
    output: Option<PolyType>,

    #[serde(skip)]
    assertions: TokenStream,
}

impl PolyFn {
    pub fn assertions(&self) -> &TokenStream {
        &self.assertions
    }

    pub fn build(item: &syn::ItemFn) -> PolyResult<Self> {
        // fail if function contains generics
        if !item.sig.generics.params.empty_or_trailing() {
            return Err(PolyError::build(
                &item.sig.generics.params,
                "Polygen does not support generic functions.",
            ));
        }

        // create initial stream
        let mut assertions = TokenStream::new();

        // assert that output type is exported by polygen
        use syn::ReturnType::*;
        if let Type(_, ty) = &item.sig.output {
            assertions.extend(quote_spanned! { ty.span() =>
                const _: fn() = || {
                    fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                    fn __accept_exported(_item: #ty) { __assert_exported(_item); }
                };
            });
        }

        // create error builder
        let mut errors = PolyErrorBuilder::new();

        // build the output type
        let output = match &item.sig.output {
            Default => None,
            Type(_, ty) => match PolyType::new(ty) {
                Ok(ty) => Some(ty),
                Err(error) => {
                    errors.merge(error);
                    None
                }
            },
        };

        // assert that all input fields are also exported by polygen
        let mut inputs = Vec::new();
        for input in &item.sig.inputs {
            match FnArg::new(input) {
                Ok(arg) => inputs.push(arg),
                Err(error) => errors.merge(error),
            }

            use syn::FnArg::*;
            match input {
                Typed(typed) => {
                    let ty = &typed.ty;
                    assertions.extend(quote_spanned! { ty.span() =>
                        const _: fn() = || {
                            fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                            fn __accept_exported(_item: #ty) { __assert_exported(_item); }
                        };
                    });
                }
                _ => (),
            }
        }

        errors.fork()?;

        let abi = match &item.sig.abi {
            Some(abi) => Some(match &abi.name {
                Some(name) => name.to_token_stream().to_string(),
                None => "".to_string(),
            }),
            None => None,
        };

        // collect the attributes into a string list
        let mut attrs = Vec::new();
        for attr in &item.attrs {
            attrs.push(attr.meta.to_token_stream().to_string());
        }

        Ok(Self {
            vis: item.vis.to_token_stream().to_string() == "pub",
            abi,
            name: item.sig.ident.to_string(),
            attrs,
            inputs,
            output,
            assertions,
        })
    }
}
