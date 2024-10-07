use lite_model::{ClassName, EnumClass, EnumValue, EnumVariant, Field, RustQualifiedName};
use proc_macro2::TokenStream;
use syn::Ident;

use crate::extract_ty::extract_ty;

pub fn extract_pod_enum(
    rust_path: &str,
    attr: Option<TokenStream>,
    item: &syn::ItemEnum,
    errors: &mut Vec<syn::Error>,
    types: &mut Vec<syn::Type>,
) -> Option<(Ident, EnumClass)> {
    let mut variants = Vec::new();
    'variants: for variant in item.variants.iter() {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(syn_fields) => {
                let mut fields = Vec::new();
                for field in syn_fields.named.iter() {
                    types.push(field.ty.clone());
                    fields.push(Field {
                        name: field
                            .ident
                            .as_ref()
                            .expect("WTF, we are in the 'named' clause")
                            .to_string(),
                        ty: match extract_ty(&field.ty, None) {
                            Ok(it) => it,
                            Err(err) => {
                                errors.push(err);
                                continue 'variants;
                            }
                        },
                    });
                }
                variants.push(EnumVariant {
                    tag: variant_name.to_string(),
                    value: EnumValue::Struct { fields },
                });
            }
            syn::Fields::Unnamed(syn_fields) => {
                let mut fields = Vec::new();
                for field in syn_fields.unnamed.iter() {
                    types.push(field.ty.clone());
                    fields.push(match extract_ty(&field.ty, None) {
                        Ok(it) => it,
                        Err(err) => {
                            errors.push(err);
                            continue 'variants;
                        }
                    });
                }
                variants.push(EnumVariant {
                    tag: variant_name.to_string(),
                    value: EnumValue::Tuple { fields },
                });
            }
            syn::Fields::Unit => {
                variants.push(EnumVariant {
                    tag: variant_name.to_string(),
                    value: EnumValue::Unit,
                });
            }
        }
    }
    let class_name = attr.map(|it| it.to_string()).unwrap_or_else(|| item.ident.to_string());
    Some((
        item.ident.clone(),
        EnumClass {
            class_name: ClassName(class_name),
            variants,
            rust_struct_path: RustQualifiedName(format!("{}::{}", rust_path, item.ident)),
        },
    ))
}
