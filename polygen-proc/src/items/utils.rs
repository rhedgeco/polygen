use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

pub fn assert_type_exported(ty: &syn::Type) -> TokenStream {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let next_id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let ident = syn::Ident::new(&format!("__ASSERT_ID{next_id}"), Span::call_site());
    quote_spanned! { ty.span() =>
        const #ident : fn() = || {
            fn __assert_exported<T: polygen::__private::exported_by_polygen>(_item: T) {}
            fn __accept_exported(_item: #ty) { __assert_exported(_item); }
        };
    }
}
