mod polyengine;
mod polyitems;

use polyengine::PolyEngine;
use polyitems::{PolyBuild, PolyItem};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get the item and parse it, returning an error on failure
    let item = syn::parse_macro_input!(item as syn::Item);
    let PolyBuild {
        polyitem,
        assertions,
    } = match PolyItem::build(&item) {
        Ok(polyitem) => polyitem,
        Err(error) => {
            return quote! {
                #error
                #item
            }
            .into();
        }
    };

    // get the engine instance and return an error if it failed
    let engine = match PolyEngine::get_instance() {
        Ok(engine) => engine,
        Err(error) => {
            return quote! {
                #assertions
                #error
                #item
            }
            .into();
        }
    };

    // process the polyitem using the engine
    if let Err(error) = engine.process(&polyitem) {
        return quote! {
            #assertions
            #error
            #item
        }
        .into();
    }

    quote! {
        #assertions
        #item
    }
    .into()
}
