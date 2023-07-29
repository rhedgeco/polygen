mod polyfield;
mod polyfunction;
mod polystruct;
mod polytype;

pub use polyfield::*;
pub use polyfunction::*;
pub use polystruct::*;
pub use polytype::*;
use quote::{__private::TokenStream, quote};
use serde::Serialize;

pub type PolyResult<T> = Result<T, quote::__private::TokenStream>;

#[derive(Serialize)]
pub enum PolyItem {
    Struct(PolyStruct),
    Fn(PolyFn),
    Unsupported,
}

impl PolyItem {
    pub fn build(item: syn::Item) -> (Self, TokenStream) {
        use syn::Item::*;
        match item {
            Struct(item) => {
                let (item, stream) = PolyStruct::build(item);
                (Self::Struct(item), stream)
            }
            Fn(item) => {
                let (item, stream) = PolyFn::build(item);
                (Self::Fn(item), stream)
            }
            _ => (Self::Unsupported, quote!(#item)),
        }
    }
}
