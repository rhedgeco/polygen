use quote::ToTokens;
use serde::Serialize;
use syn::spanned::Spanned;

use super::{PolyError, PolyResult};

#[derive(Serialize)]
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
    pub fn new(item: &syn::Type) -> PolyResult<Self> {
        Self::new_spanned(item, item)
    }

    fn new_spanned(item: &syn::Type, span: &impl Spanned) -> PolyResult<Self> {
        use syn::Type::*;
        match item {
            Path(path) => match path.path.segments.last() {
                Some(name) => Ok(Self::Named(name.to_token_stream().to_string())),
                None => Err(bad_type(span)),
            },
            Reference(reference) => {
                let inner_type = Box::new(PolyType::new_spanned(&reference.elem, span)?);
                match reference.mutability {
                    Some(_) => Ok(Self::RefMut(inner_type)),
                    None => Ok(Self::Ref(inner_type)),
                }
            }
            Ptr(ptr) => {
                let inner_type = Box::new(PolyType::new_spanned(&ptr.elem, span)?);
                match ptr.const_token {
                    Some(_) => Ok(Self::PtrConst(inner_type)),
                    None => Ok(Self::PtrMut(inner_type)),
                }
            }
            _ => Err(bad_type(span)),
        }
    }
}

fn bad_type(span: &impl Spanned) -> PolyError {
    PolyError::build(span, "This type is not supported by polygen.")
}
