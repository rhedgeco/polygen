mod builder;
mod types;

extern crate proc_macro;
use builder::PolygenBuilder;
use proc_macro::TokenStream;
use quote::quote;

static BUILDER: PolygenBuilder = PolygenBuilder::new("./polygen/*", "./target/polygen");

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::Item);

    if let Err(e) = BUILDER.write_item(&item) {
        let message = format!("Polygen Error: {e}");
        return quote!(compile_error!(#message);#item).into();
    }

    match item {
        syn::Item::Struct(item) => types::polystruct(item),
        item => types::unsupported(item),
    }
    .into()
}
