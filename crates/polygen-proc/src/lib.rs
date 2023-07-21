mod engine;
mod functions;
mod process;

extern crate proc_macro;
use engine::PolygenEngine;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;

static ENGINE: Lazy<PolygenEngine> =
    Lazy::new(|| PolygenEngine::new("./polygen", "./target/polygen"));

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::Item);
    ENGINE.process(&item)
}
