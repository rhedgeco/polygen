use quote::{__private::TokenStream, quote};

pub fn unsupported(item: &syn::Item) -> TokenStream {
    quote! {
        #item
        compile_error!("This item is unsupported by polygen");
    }
}
