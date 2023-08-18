use quote::{quote_spanned, ToTokens};
use serde::Serialize;
use syn::spanned::Spanned;

use super::{BuildResult, PolyBuild, PolyError};

#[derive(Serialize, Clone)]
pub enum PolyType {
    #[serde(rename = "named")]
    Named(String),
    #[serde(rename = "ref")]
    Ref(Box<PolyType>),
    #[serde(rename = "mutref")]
    RefMut(Box<PolyType>),
    #[serde(rename = "mutptr")]
    PtrMut(Box<PolyType>),
    #[serde(rename = "constptr")]
    PtrConst(Box<PolyType>),
}

impl PolyType {
    pub fn build(item: &syn::Type) -> BuildResult<Self> {
        static UNSUPPORTED_MESSAGE: &str = "This type is not supported by polygen.";

        use syn::Type::*;
        match item {
            Path(path) => match path.path.segments.last() {
                Some(name) => {
                    let name = name.to_token_stream().to_string();
                    let assertions = quote_spanned! { item.span() =>
                        const _: fn() = || {
                            fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                            fn __accept_exported(_item: #item) { __assert_exported(_item); }
                        };
                    };

                    PolyBuild::build(Self::Named(name), assertions)
                }
                None => Err(PolyError::spanned(item, UNSUPPORTED_MESSAGE)),
            },
            Reference(reference) => Ok(Self::build(&reference.elem)?.map(|inner| match reference
                .mutability
            {
                Some(_) => Self::RefMut(Box::new(inner)),
                None => Self::Ref(Box::new(inner)),
            })),
            Ptr(ptr) => Ok(Self::build(&ptr.elem)?.map(|inner| match ptr.const_token {
                Some(_) => Self::PtrConst(Box::new(inner)),
                None => Self::PtrMut(Box::new(inner)),
            })),
            _ => Err(PolyError::spanned(item, UNSUPPORTED_MESSAGE)),
        }
    }
}

#[derive(Serialize)]
pub struct PolyField {
    vis: bool,
    name: String,
    r#type: PolyType,
}

impl PolyField {
    pub fn build(index: usize, field: &syn::Field) -> BuildResult<Self> {
        let vis = field.vis.to_token_stream().to_string() == "pub";
        let name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => format!("x{index}"),
        };
        Ok(PolyType::build(&field.ty)?.map(|r#type| Self { vis, name, r#type }))
    }
}
