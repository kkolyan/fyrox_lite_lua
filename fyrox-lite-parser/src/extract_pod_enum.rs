use std::cell::Ref;

use fyrox_lite_model::{EnumClass, EnumClassName, EnumVariant, Field};
use proc_macro2::TokenStream;
use syn::Ident;

use crate::extract_ty::extract_ty;

pub fn extract_pod_enum(
    attr: TokenStream,
    item: &syn::ItemEnum,
    errors: &mut Vec<syn::Error>,
    types: &mut Vec<syn::Type>,
) -> Option<(Ident, EnumClass)> {
    let class_name = attr.to_string();
    let mut class = EnumClass {
        class_name: EnumClassName(class_name),
        variants: Default::default(),
    };
	'variants:
    for variant in item.variants.iter() {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(syn_fields) => {
                let mut fields = Vec::new();
                for field in syn_fields.named.iter() {
					types.push(field.ty.clone());
                    fields.push(Field {
                        field_name: field.ident.as_ref().expect("WTF, we are in the 'named' clause").to_string(),
                        ty: match extract_ty(&field.ty) {
							Ok(it) => it,
							Err(err) => {
								errors.push(err);
								continue 'variants;
							},
						},
                    });
                }
                class
                    .variants
                    .push((variant_name.to_string(), EnumVariant::Struct { fields }));
            }
            syn::Fields::Unnamed(syn_fields) => {
                let mut fields = Vec::new();
                for field in syn_fields.unnamed.iter() {
					types.push(field.ty.clone());
                    fields.push(match extract_ty(&field.ty) {
						Ok(it) => it,
						Err(err) => {
							errors.push(err);
							continue 'variants;
						},
					});
                }
                class
                    .variants
                    .push((variant_name.to_string(), EnumVariant::Tuple { fields }));
			},
            syn::Fields::Unit => {
				class.variants.push((variant_name.to_string(), EnumVariant::Unit));
			},
        }
    }
    Some((item.ident.clone(), class))
}
