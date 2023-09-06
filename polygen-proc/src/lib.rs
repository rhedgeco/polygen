use proc_macro::TokenStream;
use quote::quote;

mod process;

#[proc_macro_attribute]
pub fn polygen(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::Item);

    use syn::Item as I;
    let processed = match &item {
        I::Struct(item) => process::polystruct(attr, item),
        I::Fn(item) => process::polyfn(attr, item),
        I::Impl(item) => process::polyimpl(attr, item),
        _ => quote!(compile_error!("This item is unsupported by polygen");),
    };

    quote!( #processed #item ).into()
}
