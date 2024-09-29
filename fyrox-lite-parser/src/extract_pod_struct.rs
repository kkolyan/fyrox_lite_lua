
use fyrox_lite_model::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use crate::extract_ty::extract_ty;


pub fn extract_pod_struct(
    attr: TokenStream,
    item: &syn::ItemStruct,
    errors: &mut Vec<syn::Error>,
) -> Option<(Ident, PodClass)> {
    let class_name = attr.to_string();
    let rust_name = item.ident.clone();
    let mut pod = PodClass {
        parent: None,
        class_name: PodClassName(class_name),
        fields: Default::default(),
    };
    for field in item.fields.iter() {
        pod.fields.push(Field {
            field_name: field.ident.to_token_stream().to_string(),
            ty: match extract_ty(&field.ty, None) {
                Ok(it) => it,
                Err(err) => {
                    errors.push(err);
                    continue;
                }
            },
        });
    }
    Some((rust_name, pod))
}