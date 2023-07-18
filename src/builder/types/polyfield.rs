use quote::ToTokens;
use serde::Serialize;

#[derive(Serialize)]
pub struct PolyField {
    pub vis: String,
    pub name: String,
    pub r#type: String,
}

impl PolyField {
    pub fn new(index: usize, field: &syn::Field) -> Self {
        let name = match &field.ident {
            Some(name) => name.to_string(),
            None => format!("f{index}"),
        };

        Self {
            name,
            r#type: field.ty.to_token_stream().to_string(),
            vis: field.vis.to_token_stream().to_string(),
        }
    }
}
