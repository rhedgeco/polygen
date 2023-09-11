use quote::{quote, quote_spanned, TokenStreamExt};
use syn::spanned::Spanned;

use super::PolyAttr;

pub fn polystruct(_attrs: &PolyAttr, item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    // fail on generics
    if !item.generics.params.empty_or_trailing() {
        return quote_spanned! { item.generics.params.span() =>
            compile_error!("Generics are not supported by #[polygen] attribute");
        };
    }

    // get useful items
    let ident = &item.ident;
    let fields = &item.fields;
    let export_ident = syn::Ident::new(&format!("__polygen_struct_{ident}"), ident.span());

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
                let field_type = &field.ty;
                let export_type = make_exp(field_type);
                let Some(field_name) = &field.ident else {
                    unreachable!(); // named structs will always have an ident
                };
                from_fields.append_all(quote_spanned! { field_type.span() =>
                    #field_name: #export_type::from(value.#field_name),
                });
                into_fields.append_all(quote_spanned! { field_type.span() =>
                    #field_name: #export_type::into(self.#field_name),
                });
                export_fields
                    .append_all(quote_spanned!( field_type.span() => #field_name: #export_type, ));
                poly_fields.append_all(quote_spanned! { field_type.span() =>
                    ::polygen::items::StructField {
                        name: stringify!(#field_name),
                        ty: ::polygen::items::FieldType::Typed(
                            &<#field_type as ::polygen::__private::ExportedPolyStruct>::STRUCT
                        ),
                    },
                });
            }
            output.append_all(quote! {
                impl From<#ident> for #export_ident {
                    fn from(value: #ident) -> Self {
                        Self { #from_fields }
                    }
                }
                impl Into<#ident> for #export_ident {
                    fn into(self) -> #ident {
                        #ident { #into_fields }
                    }
                }
            });
        }
    };

    output.append_all(quote! {
        #[repr(C)]
        #[doc(hidden)]
        pub struct #export_ident {
            #export_fields
        }

        unsafe impl ::polygen::__private::ExportedPolyStruct for #ident {
            type ExportedType = #export_ident;

            const STRUCT: ::polygen::items::PolyStruct = ::polygen::items::PolyStruct {
                module: module_path!(),
                name: stringify!(#ident),
                fields: &[#poly_fields],
                generics: &[], // only manually implemented generics are currently supported
            };
        }
    });

    output
}
