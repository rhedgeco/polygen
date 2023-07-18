use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::ItemStruct;

pub fn polystruct(item: ItemStruct) -> TokenStream {
    let ident = item.ident.clone();
    let (impl_gen, ty_gen, where_gen) = item.generics.split_for_impl();
    quote_spanned! {item.ident.span() =>
        #item
        const _: fn() = || {
            extern "C" {
                #[doc(hidden)]
                #[allow(dead_code)]
                #[deny(improper_ctypes)]
                #[allow(clashing_extern_declarations)]
                fn __polygen_assert_ffi_safe #impl_gen (_: #ident #ty_gen) #where_gen;
            }
        };
    }
    .into()
}
