use quote::{__private::TokenStream, quote};

pub fn unsupported(item: &syn::Item) -> TokenStream {
    let name = match item {
        syn::Item::Const(_) => "'const'",
        syn::Item::Enum(_) => "'enum'",
        syn::Item::ExternCrate(_) => "'extern crate'",
        syn::Item::Fn(_) => "'fn'",
        syn::Item::ForeignMod(_) => "'foreign mod'",
        syn::Item::Impl(_) => "'impl'",
        syn::Item::Macro(_) => "'macro'",
        syn::Item::Mod(_) => "'mod'",
        syn::Item::Static(_) => "'static'",
        syn::Item::Struct(_) => "'struct'",
        syn::Item::Trait(_) => "'trait'",
        syn::Item::TraitAlias(_) => "'trait alias'",
        syn::Item::Type(_) => "'type'",
        syn::Item::Union(_) => "'union'",
        syn::Item::Use(_) => "'use'",
        _ => "these",
    };

    let message = format!(
        "{name} items are not supported by polygen. \
        Please file an issue with your use case, or better yet submit a PR. :)"
    );
    quote! {
        compile_error!(#message);
        #item
    }
}
