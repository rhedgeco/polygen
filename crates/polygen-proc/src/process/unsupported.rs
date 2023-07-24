use quote::{__private::TokenStream, quote};

pub fn unsupported(item: &syn::Item) -> TokenStream {
    #[cfg(feature = "allow-unsupported")]
    let support_assert = quote!();
    #[cfg(not(feature = "allow-unsupported"))]
    let support_assert = quote!(compile_error!("This item is currently unsupported by polygen. Please submit an issue. Or even better, submit a PR! :)"););

    quote!( #item #support_assert )
}
