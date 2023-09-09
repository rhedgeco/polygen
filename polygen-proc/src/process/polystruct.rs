use std::collections::BTreeSet;

use quote::{quote, quote_spanned, TokenStreamExt};
use syn::spanned::Spanned;

use super::PolyAttr;

pub fn polystruct(_attrs: &PolyAttr, item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let mut generics = BTreeSet::new();
    let mut export_generics = item.generics.clone();
    for param in &mut export_generics.params {
        use syn::GenericParam as G;
        match param {
            G::Const(c) => {
                return quote_spanned! { c.ident.span() =>
                    compile_error!("Constant generics are not supported by #[polygen] attribute");
                }
            }
            G::Lifetime(l) => {
                return quote_spanned! { l.lifetime.span() =>
                    compile_error!("Lifetimes are not supported by #[polygen] attribute");
                }
            }
            G::Type(ty) => {
                generics.insert(ty.ident.clone());
                ty.bounds.push(syn::parse_quote! {
                    ::polygen::__private::ExportedPolyStruct
                });
            }
        }
    }

    // get useful items
    let ident = &item.ident;
    let fields = &item.fields;
    let export_ident = syn::Ident::new(&format!("__polygen_struct_{ident}"), ident.span());
    let (implgen, typegen, wheregen) = export_generics.split_for_impl();

    // create initial output stream
    let mut output = quote!();

    // create helper type for forcing exported type
    let make_exp = |t: &syn::Type| -> syn::Type {
        syn::parse_quote_spanned! { t.span() =>
            <#t as ::polygen::__private::ExportedPolyStruct>::ExportedType
        }
    };

    // collect field data for construction
    let mut export_fields = quote!();
    let mut poly_fields = quote!();
    use syn::Fields as F;
    match fields {
        F::Unit => {
            return quote_spanned! { ident.span() =>
                compile_error!("Unit structs are not FFI safe.");
            }
        }
        F::Named(f) if f.named.len() == 0 => {
            return quote_spanned! { ident.span() =>
                compile_error!("Empty structs are not FFI safe.");
            }
        }
        F::Unnamed(_) => {
            return quote_spanned! { ident.span() =>
                compile_error!("Tuple structs are currently unsupported by #[polygen]");
            }
        }
        F::Named(f) => {
            let mut from_fields = quote!();
            let mut into_fields = quote!();
            for field in f.named.iter() {
                let ty = &field.ty;
                let exp_ty = make_exp(ty);
                let Some(ident) = &field.ident else {
                    unreachable!(); // named structs will always have an ident
                };
                from_fields.append_all(quote_spanned! { ty.span() =>
                    #ident: #exp_ty::from(value.#ident),
                });
                into_fields.append_all(quote_spanned! { ty.span() =>
                    #ident: #exp_ty::into(self.#ident),
                });
                export_fields.append_all(quote_spanned!( ty.span() => #ident: #exp_ty, ));
                poly_fields.append_all(quote_spanned! { ty.span() =>
                    ::polygen::items::PolyField {
                        name: stringify!(#ident),
                        ty_name: stringify!(#ty),
                        ty: &<#ty as ::polygen::__private::ExportedPolyStruct>::STRUCT,
                    },
                });
            }
            output.append_all(quote! {
                impl #implgen From<#ident #typegen> for #export_ident #typegen #wheregen {
                    fn from(value: #ident #typegen) -> Self {
                        Self { #from_fields }
                    }
                }
                impl #implgen Into<#ident #typegen> for #export_ident #typegen #wheregen {
                    fn into(self) -> #ident #typegen {
                        #ident { #into_fields }
                    }
                }
            });
        }
    };

    output.append_all(quote! {
        #[repr(C)]
        #[doc(hidden)]
        pub struct #export_ident #implgen {
            #export_fields
        }

        unsafe impl #implgen ::polygen::__private::ExportedPolyStruct for #ident #typegen #wheregen {
            type ExportedType = #export_ident #typegen;
            
            const STRUCT: ::polygen::items::PolyStruct = ::polygen::items::PolyStruct {
                module: module_path!(),
                name: stringify!(#ident),
                fields: &[#poly_fields],
                generics: &[#(stringify!(#generics),)*],
            };
        }
    });

    output
}
