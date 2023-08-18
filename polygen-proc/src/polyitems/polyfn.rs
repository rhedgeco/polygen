use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::Serialize;

use crate::polyitems::{PolyBuild, PolyError};

use super::{BuildResult, PolyType};

#[derive(Serialize)]
struct PolyArg {
    name: String,
    r#type: PolyType,
}

impl PolyArg {
    pub fn build(item: &syn::FnArg, self_override: Option<&str>) -> BuildResult<Self> {
        use syn::FnArg::*;
        let typed = match item {
            Typed(typed) => typed,
            Receiver(rec) => match self_override {
                None => {
                    return Err(PolyError::spanned(
                        rec,
                        "This function doesn't support the 'self' parameter.",
                    ))
                }
                Some(self_override) => {
                    if rec.colon_token.is_some() {
                        return Err(PolyError::spanned(
                            rec,
                            "Polygen functions don't support explicit 'self' parameters.",
                        ));
                    }

                    return Ok(
                        PolyType::build(&rec.ty, Some(self_override))?.map(|ty| Self {
                            name: "_self".to_string(),
                            r#type: ty,
                        }),
                    );
                }
            },
        };

        use syn::Pat::*;
        let name = match &*typed.pat {
            Ident(ident) => ident.ident.to_string(),
            pat => {
                return Err(PolyError::spanned(
                    pat,
                    "This pattern is not supported by polygen.",
                ));
            }
        };

        Ok(PolyType::build(&typed.ty, self_override)?.map(|r#type| Self { name, r#type }))
    }
}

#[derive(Serialize)]
pub struct PolyFnSig {
    abi: Option<String>,
    name: String,
    inputs: Vec<PolyArg>,
    output: Option<PolyType>,
}

impl PolyFnSig {
    pub fn build(item: &syn::Signature, self_override: Option<&str>) -> BuildResult<Self> {
        // fail if function contains generics
        if !item.generics.params.empty_or_trailing() {
            return Err(PolyError::spanned(
                &item.generics.params,
                "Polygen does not support generic functions.",
            ));
        }

        // create errors
        let mut errors = PolyError::empty();

        // create assertion stream
        let mut assertions = TokenStream::new();

        // build the output type
        use syn::ReturnType::*;
        let output = match &item.output {
            Default => None,
            Type(_, ty) => match PolyType::build(ty, self_override) {
                Ok(build) => {
                    assertions.extend(build.assertions);
                    Some(build.polyitem)
                }
                Err(error) => {
                    errors.merge(error);
                    None
                }
            },
        };

        // assert that all input fields are also exported by polygen
        let mut inputs = Vec::new();
        for input in &item.inputs {
            match PolyArg::build(input, self_override) {
                Ok(build) => {
                    assertions.extend(build.assertions);
                    inputs.push(build.polyitem);
                }
                Err(error) => errors.merge(error),
            }
        }

        errors.fork()?;

        let abi = match &item.abi {
            Some(abi) => Some(match &abi.name {
                Some(name) => name.to_token_stream().to_string(),
                None => "".to_string(),
            }),
            None => None,
        };

        PolyBuild::build(
            Self {
                abi,
                name: item.ident.to_string(),
                inputs,
                output,
            },
            assertions,
        )
    }
}

#[derive(Serialize)]
pub struct PolyFn {
    vis: bool,
    attrs: Vec<String>,
    #[serde(flatten)]
    sig: PolyFnSig,
}

impl PolyFn {
    pub fn build(item: &syn::ItemFn) -> BuildResult<Self> {
        // get a boolean for public visibility
        let vis = item.vis.to_token_stream().to_string() == "pub";

        // collect the attributes into a string list
        let mut attrs = Vec::new();
        for attr in &item.attrs {
            attrs.push(attr.meta.to_token_stream().to_string());
        }

        Ok(PolyFnSig::build(&item.sig, None)?.map(|sig| Self { vis, attrs, sig }))
    }
}
