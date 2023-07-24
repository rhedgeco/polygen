use quote::{__private::TokenStream, quote};

pub fn unsupported(item: &syn::Item) -> TokenStream {
    quote! {
        #item
        compile_error!("This item is currently unsupported by polygen. Please submit an issue. Or even better, submit a PR! :)");
    }
}
