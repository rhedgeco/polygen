mod builder;
mod engine;

extern crate proc_macro;
use engine::PolygenEngine;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use std::{env, fs, path::PathBuf};
use syn_serde::Syn;

const LOG_DIR: &str = "./target/polygen";
const BINDING_DIR: &str = "./target/polygen/bindings";
static ENGINE: Lazy<PolygenEngine> = Lazy::new(|| PolygenEngine::new("./polygen"));

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse item
    let item = syn::parse_macro_input!(item as syn::Item);

    // validate item with scripting engine
    let new_engine = once_cell::sync::Lazy::<PolygenEngine>::get(&ENGINE).is_none();
    let process_error = match ENGINE.process_item(item.to_adapter()) {
        Ok(_) => quote!(),
        Err(e) => {
            let message = e.to_string();
            quote!(compile_error!(#message);)
        }
    };

    // flush logs if necesary
    if new_engine {
        fs::create_dir_all(LOG_DIR).expect("Could not create log dir");
        let log_path = PathBuf::from(LOG_DIR).join("polygen.log");
        ENGINE.flush_logs(log_path).expect("Could not flush logs");
    }

    // flush bindings if necessary
    let binding_error = if env::var("NO_POLYGEN_BINDINGS") == Ok("1".to_string()) {
        quote!()
    } else {
        fs::create_dir_all(BINDING_DIR).expect("Could not create binding dir");
        match ENGINE.flush_bindings(BINDING_DIR) {
            Ok(_) => quote!(),
            Err(e) => {
                let message = e.to_string();
                quote!(compile_error!(#message);)
            }
        }
    };

    // build item into final output
    let output = match item {
        syn::Item::Struct(item) => builder::polystruct(item),
        item => builder::unsupported(item),
    };

    // construct final stream using vaidation and output
    quote!(#process_error #binding_error #output).into()
}
