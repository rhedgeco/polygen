use quote::ToTokens;
use serde::Serialize;

use super::{PolyResult, PolyType};

#[derive(Serialize)]
pub struct PolyField {
    vis: bool,
    name: String,
    r#type: PolyType,
}

impl PolyField {
    pub fn new(index: usize, field: &syn::Field) -> PolyResult<Self> {
        let vis = field.vis.to_token_stream().to_string() == "pub";
        let name = match &field.ident {
            Some(ident) => ident.to_string(),
            None => format!("x{index}"),
        };
        let r#type = PolyType::new(&field.ty)?;
        Ok(Self { vis, name, r#type })
    }
}
