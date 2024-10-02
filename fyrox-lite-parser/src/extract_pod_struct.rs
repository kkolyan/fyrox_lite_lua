use fyrox_lite_model::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::extract_ty::extract_ty;

pub fn extract_pod_struct(
    rust_path: &str,
    attr: TokenStream,
    item: &syn::ItemStruct,
    errors: &mut Vec<syn::Error>,
) -> Option<(Ident, StructClass)> {
    let class_name = attr.to_string();
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
