use std::collections::HashMap;

use fyrox_lite_model::DataType;
use quote::ToTokens;
use syn::Ident;

pub fn extract_ty_path(
    qself: Option<&syn::QSelf>,
    path: &syn::Path,
    span: impl ToTokens,
    generic_params: Option<&HashMap<&Ident, DataType>>,
) -> syn::Result<fyrox_lite_model::DataType> {
    if qself.is_some() {
        return Err(syn::Error::new_spanned(
            span,
            "Fyrox Lite: Self is not allowed",
        ));
    }
    if path.segments.len() > 1 {
        return Err(syn::Error::new_spanned(
            &path.segments,
            "Fyrox Lite: qualified names not allowed",
        ));
    }
    let segment = &path.segments[0];
    if segment.ident == "Vec" {
        return Ok(DataType::Vec(Box::new(extract_single_type_param(segment, generic_params)?)));
    }
    if segment.ident == "Option" {
        return Ok(DataType::Option(Box::new(extract_single_type_param(
            segment, generic_params,
        )?)));
    }
    let syn::PathArguments::None = &segment.arguments else {
        return Err(syn::Error::new_spanned(
            &path.segments,
            "Fyrox Lite: generics allowed for `Vec` and `Option` only",
        ));
    };
    Ok(match segment.ident.to_string().as_str() {
        "bool" => DataType::Bool,
        "u8" => DataType::Byte,
        "i32" => DataType::I32,
        "i64" => DataType::I64,
        "f32" => DataType::F32,
        "f64" => DataType::F64,
        "String" => DataType::String,
        _ => DataType::UnresolvedClass(segment.ident.to_string()),
    })
}

pub fn extract_ty(
    ty: &syn::Type,
    generic_params: Option<&HashMap<&Ident, DataType>>,
) -> syn::Result<fyrox_lite_model::DataType> {
    match ty {
        syn::Type::Path(it) => extract_ty_path(it.qself.as_ref(), &it.path, it, generic_params),
        _ => Err(syn::Error::new_spanned(ty, "Fyrox Lite: not allowed type")),
    }
}

fn extract_single_type_param(
    segment: &syn::PathSegment,
    generic_params: Option<&HashMap<&Ident, DataType>>,
) -> Result<DataType, syn::Error> {
    if let syn::PathArguments::AngleBracketed(it) = &segment.arguments {
        if it.args.len() == 1 {
            if let syn::GenericArgument::Type(ty) = &it.args[0] {
                return extract_ty(ty, generic_params);
            }
        }
    }
    Err(syn::Error::new_spanned(
        segment,
        "Fyrox Lite: expected to have a single type parameter",
    ))
}
