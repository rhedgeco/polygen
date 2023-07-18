use quote::ToTokens;
use serde::Serialize;

use super::PolyField;

#[derive(Serialize)]
pub struct PolyStruct {
    pub vis: String,
    pub name: String,
    pub fields: Vec<PolyField>,
    pub repr: Vec<String>,
}

impl From<&syn::ItemStruct> for PolyStruct {
    fn from(value: &syn::ItemStruct) -> Self {
        let mut fields = Vec::with_capacity(value.fields.len());
        for (i, field) in value.fields.iter().enumerate() {
            fields.push(PolyField::new(i, field));
        }

        let repr = value.attrs.iter().find_map(|a| {
            let list = match &a.meta {
                syn::Meta::List(list) => list,
                _ => return None,
            };

            if list.path.to_token_stream().to_string() != "repr" {
                return None;
            }

            let repr = list
                .tokens
                .to_string()
                .replace(" ", "")
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            return Some(repr);
        });

        Self {
            fields,
            name: value.ident.to_string(),
            vis: value.vis.to_token_stream().to_string(),
            repr: Vec::new(),
        }
    }
}
