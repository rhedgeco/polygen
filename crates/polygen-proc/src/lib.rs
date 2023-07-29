mod engine;
mod items;

extern crate proc_macro;

use std::{fs, path::PathBuf};

use engine::{EngineError, PolyEngine};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use rhai::serde::to_dynamic;

use crate::items::PolyItem;

const SCRIPT_DIR: &str = "./polygen";
const BUILD_DIR: &str = "./target/polygen";
static ENGINE: Lazy<Result<PolyEngine, EngineError>> = Lazy::new(|| PolyEngine::load(SCRIPT_DIR));

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the item
    let item = syn::parse_macro_input!(item as syn::Item);

    // ensure engine is successfully loaded
    let engine = match ENGINE.as_ref() {
        Ok(engine) => engine,
        Err(error) => {
            let message = format!("Polygen Load Error: {error}");
            return quote! {
                compile_error!(#message);
                #item
            }
            .into();
        }
    };

    // build the item
    let (item, mut stream) = PolyItem::build(item);

    // early exit if the item is unsupported
    if let PolyItem::Unsupported = item {
        return quote! {
            compile_error!("This item is unsupported by polygen.");
            #stream
        }
        .into();
    }

    // convert item into dynamic for use in scripts
    let dynamic = match to_dynamic(item) {
        Ok(dynamic) => dynamic,
        Err(error) => {
            let message = format!("Polygen Dynamic Conversion Error: {error}");
            return quote! {
                compile_error!(#message);
                #stream
            }
            .into();
        }
    };

    // loop over all scripts
    for script in engine.scripts() {
        let script_name = script.name();

        // process the item
        if let Err(mut error) = script.process(dynamic.clone()) {
            // unwrap error to get to root error
            use rhai::EvalAltResult::*;
            while let ErrorInFunctionCall(_, _, inner, _) = *error {
                error = inner;
            }

            // process runtime errors to make them prettier
            let error = match *error {
                ErrorRuntime(e, _) => format!("{e}"),
                error => format!("{error}"),
            };

            // combine output with new error message
            let message = format!("'{script_name}' - {error}");
            stream.extend(quote!(compile_error!(#message);))
        }

        // render all the items
        match script.render() {
            Ok(contents) => {
                if let Err(error) = fs::create_dir_all(BUILD_DIR) {
                    let message = format!("'{script_name}' - {error}");
                    stream.extend(quote!(compile_error!(#message);))
                }

                let file_path = PathBuf::from(BUILD_DIR).join(script_name);
                if let Err(error) = fs::write(file_path, contents) {
                    let message = format!("'{script_name}' - {error}");
                    stream.extend(quote!(compile_error!(#message);))
                }
            }
            Err(error) => {
                let message = format!("'{script_name}' - {error}");
                stream.extend(quote!(compile_error!(#message);))
            }
        }
    }

    // return the token stream
    stream.into()
}
