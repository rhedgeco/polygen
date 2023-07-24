use quote::{__private::TokenStream, quote};

pub fn other(item: &syn::Item) -> TokenStream {
    quote!(#item)
}
