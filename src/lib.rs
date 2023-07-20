mod engine;
mod functions;

extern crate proc_macro;
use engine::PolygenEngine;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use std::{fs, path::PathBuf};
use syn_serde::Syn;

const SCRIPT_DIR: &str = "./polygen";
const BUILD_DIR: &str = "./target/polygen";
static ENGINE: Lazy<PolygenEngine> = Lazy::new(|| PolygenEngine::new(SCRIPT_DIR, BUILD_DIR));

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse item
    let item = syn::parse_macro_input!(item as syn::Item);

    // validate item with scripting engine
    let new_engine = once_cell::sync::Lazy::<PolygenEngine>::get(&ENGINE).is_none();
    let build_error = match ENGINE.build_item(item.to_adapter()) {
        Ok(_) => quote!(),
        Err(e) => {
            let message = e.to_string();
            quote!(compile_error!(#message);)
        }
    };

    // flush logs if necesary
    if new_engine {
        fs::create_dir_all(BUILD_DIR).expect("Could not create log dir");
        let log_path = PathBuf::from(BUILD_DIR).join("polygen.log");
        ENGINE.flush_logs(log_path).expect("Could not flush logs");
    }

    // merge errors together with the original item
    quote!(#build_error #item).into()
}
