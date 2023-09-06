use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, Token};

pub fn polystruct(_attr: TokenStream, item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    // fail on generics
    if !item.generics.params.empty_or_trailing() {
        return quote_spanned! { item.generics.params.span() =>
            compile_error!("Generics are not supported by #[polygen] attribute");
        };
    }

    // get useful items
    let ident = &item.ident;
    let fields = &item.fields;
    let semi_token = &item.semi_token;
    let export_ident = syn::Ident::new(&format!("__polygen_struct_{ident}"), ident.span());

    // create fields needed for construction
    let from_convert;
    let into_convert;
    let mut polyfields = Punctuated::<proc_macro2::TokenStream, Token![,]>::new();
    match fields {
        syn::Fields::Named(fields) if fields.named.len() > 0 => {
            let mut from_fields = Punctuated::<syn::FieldValue, Token![,]>::new();
            let mut into_fields = Punctuated::<syn::FieldValue, Token![,]>::new();
            for field in &fields.named {
                let ty = &field.ty;
                let ident = match &field.ident {
                    Some(ident) => format!("{ident}"),
                    None => format!("_"),
                };

                let syn_ident = syn::Ident::new(&ident, field.ident.span());
                from_fields.push(syn::parse_quote!( #syn_ident: value.#syn_ident ));
                into_fields.push(syn::parse_quote!( #syn_ident: self.#syn_ident ));
                polyfields.push(quote_spanned! { field.ident.span() =>
                    ::polygen::items::PolyField {
                        name: #ident,
                        ty: <#ty as ::polygen::__private::ExportedPolyType>::TYPE,
                    }
                })
            }
            from_convert = quote!(Self { #from_fields } );
            into_convert = quote!( #ident { #into_fields } );
        }
        syn::Fields::Unnamed(fields) if fields.unnamed.len() > 0 => {
            let mut from_fields = Punctuated::<syn::Expr, Token![,]>::new();
            let mut into_fields = Punctuated::<syn::Expr, Token![,]>::new();
            for (i, field) in fields.unnamed.iter().enumerate() {
                let ty = &field.ty;
                let syn_index = syn::Index::from(i);
                from_fields.push(syn::parse_quote!( value.#syn_index ));
                into_fields.push(syn::parse_quote!( self.#syn_index ));
                polyfields.push(quote_spanned! { field.ident.span() =>
                    ::polygen::items::PolyField {
                        name: "_",
                        ty: <#ty as ::polygen::__private::ExportedPolyType>::TYPE,
                    }
                })
            }
            from_convert = quote!(Self ( #from_fields ) );
            into_convert = quote!( #ident ( #into_fields ) );
        }
        _ => {
            return quote_spanned! { ident.span() =>
                compile_error!("Unit structs are not FFI safe.");
            };
        }
    }

    quote! {
        // create proxy type that will be exported
        #[repr(C)]
        #[doc(hidden)]
        pub struct #export_ident #fields #semi_token

        // implement from and into for easy conversions
        impl From<#ident> for #export_ident {
            fn from(value: #ident) -> Self {
                #from_convert
            }
        }
        impl Into<#ident> for #export_ident {
            fn into(self) -> #ident {
                #into_convert
            }
        }

        // implement the export type to be used by compiler
        unsafe impl ::polygen::__private::ExportedPolyType for #ident {
            type ExportedType = #export_ident;
            const TYPE: ::polygen::items::PolyType = ::polygen::items::PolyType::Struct(
                ::polygen::items::PolyStruct {
                    ident: ::polygen::items::PolyIdent {
                        module: module_path!(),
                        name: stringify!(#ident),
                        export_name: stringify!(#ident),
                    },
                    fields: &[#polyfields],
                }
            );
        }
    }
}
