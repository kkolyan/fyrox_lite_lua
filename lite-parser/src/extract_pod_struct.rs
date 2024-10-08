use lite_model::*;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::Ident;

use crate::{extract_ty::extract_ty, lite_api_attr::LiteApiAttr};

pub fn extract_pod_struct(
    rust_path: &str,
    (attr, attr_span): (LiteApiAttr, Span),
    item: &syn::ItemStruct,
    errors: &mut Vec<syn::Error>,
) -> Option<(Ident, StructClass)> {
    let rust_name = item.ident.clone();
    let mut fields = Vec::new();
    for field in item.fields.iter() {
        fields.push(Field {
            name: field.ident.to_token_stream().to_string(),
            ty: match extract_ty(&field.ty, None) {
                Ok(it) => it,
                Err(err) => {
                    errors.push(err);
                    continue;
                }
            },
        });
    }
    let class_name = attr.class.unwrap_or_else(|| rust_name.to_string());
    for feature in attr.features {
        match feature {
            Feature::Eq => {
                errors.push(syn::Error::new( attr_span, "`eq` option not allowed for struct classes"));
            },
        }
    }
    Some((
        rust_name.clone(),
        StructClass {
            parent: None,
            class_name: ClassName(class_name),
            fields,
            rust_struct_path: RustQualifiedName(format!("{}::{}", rust_path, rust_name)),
        },
    ))
}
