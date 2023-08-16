use quote::{ToTokens, __private::TokenStream};
use serde::Serialize;

use crate::items::{assert_type_exported, PolyError, PolyErrorBuilder};

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

#[derive(Serialize)]
pub struct PolyFn {
    vis: bool,
    abi: Option<String>,
    name: String,
    attrs: Vec<String>,
    inputs: Vec<FnArg>,
    output: Option<PolyType>,

    #[serde(skip)]
    stream: TokenStream,
}

impl ToTokens for PolyFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.stream.clone())
    }
}

impl PolyFn {
    pub fn build(item: &syn::ItemFn) -> PolyResult<Self> {
        // fail if function contains generics
        if !item.sig.generics.params.empty_or_trailing() {
            return Err(PolyError::build(
                &item.sig.generics.params,
                "Polygen does not support generic functions.",
            ));
        }

        // create initial stream
        let mut stream = TokenStream::new();

        // assert that output type is exported by polygen
        use syn::ReturnType::*;
        if let Type(_, ty) = &item.sig.output {
            stream.extend(assert_type_exported(ty));
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
                    stream.extend(assert_type_exported(ty));
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
            stream,
        })
    }
}
