mod builder;
mod scripting;

extern crate proc_macro;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use scripting::PolygenEngine;
use std::{fs, path::PathBuf};
use syn_serde::Syn;

const OUTPUT_DIR: &str = "./target/polygen";
static ENGINE: Lazy<PolygenEngine> = Lazy::new(|| PolygenEngine::new("./polygen"));

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse item
    let item = syn::parse_macro_input!(item as syn::Item);

    // validate item with scripting engine
    let new_engine = once_cell::sync::Lazy::<PolygenEngine>::get(&ENGINE).is_none();
    let validation = match ENGINE.validate_item(item.to_adapter()) {
        Ok(_) => quote!(),
        Err(e) => {
            let message = e.to_string();
            quote!(compile_error!(#message);)
        }
    };

    // flush logs if necesary
    if new_engine {
        fs::create_dir_all(OUTPUT_DIR).expect("Could not create log dir");
        let log_path = PathBuf::from(OUTPUT_DIR).join("polygen.log");
        ENGINE.flush_logs(log_path).expect("Could not flush logs");
    }

    // build item into final output
    let output = match item {
        syn::Item::Struct(item) => builder::polystruct(item),
        item => builder::unsupported(item),
    };

    // construct final stream using vaidation and output
    quote!(#validation #output).into()
}
