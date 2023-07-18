use quote::{__private::TokenStream, quote};
use syn::Item;

pub fn unsupported(item: Item) -> TokenStream {
    quote! {
        compile_error!("Unsupported polygen item");
        #item
    }
}
