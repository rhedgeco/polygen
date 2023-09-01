use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn polyfn(_attr: TokenStream, item: &syn::ItemFn) -> proc_macro2::TokenStream {
    if !item.sig.generics.params.empty_or_trailing() {
        return quote_spanned! { item.sig.generics.params.span() =>
            compile_error!("Generics are not supported by #[polygen] attribute");
        };
    }

    return quote!();
}
