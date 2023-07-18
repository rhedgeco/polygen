use proc_macro::TokenStream;
use quote::quote;
use syn::Item;

pub fn unsupported(item: Item) -> TokenStream {
    quote! {
        compile_error!("Unsupported polygen item");
        #item
    }
    .into()
}
