use quote::{__private::TokenStream, quote};

pub fn polyfunction(polyfunction: &syn::ItemFn) -> TokenStream {
    quote!(#polyfunction)
}
