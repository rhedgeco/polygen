use quote::{quote_spanned, ToTokens};
use serde::Serialize;
use syn::spanned::Spanned;

use super::{BuildResult, PolyBuild, PolyError, PolyResult};

#[derive(Serialize)]
enum PolyTypePath {
    #[serde(rename = "named")]
    Named(String),
    #[serde(rename = "ref")]
    Ref(Box<PolyTypePath>),
    #[serde(rename = "mutref")]
    RefMut(Box<PolyTypePath>),
    #[serde(rename = "mutptr")]
    PtrMut(Box<PolyTypePath>),
    #[serde(rename = "constptr")]
    PtrConst(Box<PolyTypePath>),
}

impl PolyTypePath {
    fn build(item: &syn::Type) -> PolyResult<Self> {
        static UNSUPPORTED_MESSAGE: &str = "This type is not supported by polygen.";
        use syn::Type::*;
        match item {
            Path(path) => match path.path.segments.last() {
                Some(name) => Ok(Self::Named(name.to_token_stream().to_string())),
                None => Err(PolyError::spanned(item, UNSUPPORTED_MESSAGE)),
            },
            Reference(reference) => {
                let inner_type = Box::new(Self::build(&reference.elem)?);
                match reference.mutability {
                    Some(_) => Ok(Self::RefMut(inner_type)),
                    None => Ok(Self::Ref(inner_type)),
                }
            }
            Ptr(ptr) => {
                let inner_type = Box::new(Self::build(&ptr.elem)?);
                match ptr.const_token {
                    Some(_) => Ok(Self::PtrConst(inner_type)),
                    None => Ok(Self::PtrMut(inner_type)),
                }
            }
            _ => Err(PolyError::spanned(item, UNSUPPORTED_MESSAGE)),
        }
    }
}

#[derive(Serialize)]
pub struct PolyType {
    #[serde(flatten)]
    path: PolyTypePath,
}

impl PolyType {
    pub fn build(item: &syn::Type) -> BuildResult<Self> {
        PolyBuild::build(
            Self {
                path: PolyTypePath::build(item)?,
            },
            quote_spanned! { item.span() =>
                const _: fn() = || {
                    fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
                    fn __accept_exported(_item: #item) { __assert_exported(_item); }
                };
            },
        )
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
        PolyType::build(&field.ty)?.map(|r#type| Self { vis, name, r#type })
    }
}
