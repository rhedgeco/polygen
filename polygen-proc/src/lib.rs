mod engine;
mod items;

use engine::PolyEngine;
use items::PolyItem;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn polygen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get the item and parse it, returning an error on failure
    let item = syn::parse_macro_input!(item as syn::Item);
    let polyitem = match PolyItem::build(&item) {
        Ok(polyitem) => polyitem,
        Err(error) => {
            let stream = error.stream();
            return quote! {
                #stream
                #item
            }
            .into();
        }
    };

    // get the processed assertions
    let assertions = polyitem.assertions();

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
        let error = error.stream();
        return quote! {
            #assertions
            #error
            #item
        }
        .into();
    }

    // return the original item with the processed assertions
    let assertions = polyitem.assertions();
    quote! {
        #assertions
        #item
    }
    .into()
}
