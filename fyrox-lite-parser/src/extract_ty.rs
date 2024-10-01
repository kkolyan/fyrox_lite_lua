use std::collections::HashMap;

use fyrox_lite_model::DataType;
use quote::ToTokens;
use syn::{GenericArgument, Ident};

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
    if let Some(ident) = extract_user_script_associated_type(qself, path, generic_params) {
        match ident {
            UserScriptBasedType::Named(ident) => {
                if ident == "UserScriptMessage" {
                    return Ok(DataType::UserScriptMessage);
                }
                if ident == "UserScriptGenericStub" {
                    return Ok(DataType::UserScriptGenericStub);
                }
            }
            UserScriptBasedType::Itself => {
                return Ok(DataType::UserScript);
            },
        }
    }
    if path.segments.len() > 1 {
        return Err(syn::Error::new_spanned(
            &path.segments,
            "Fyrox Lite: qualified names not allowed",
        ));
    }
    let segment = &path.segments[0];
    if segment.ident == "Vec" {
        return Ok(DataType::Vec(Box::new(extract_single_type_param(
            segment,
            generic_params,
        )?)));
    }
    if segment.ident == "Option" {
        return Ok(DataType::Option(Box::new(extract_single_type_param(
            segment,
            generic_params,
        )?)));
    }
    if segment.ident == "Result" {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
            if args.args.len() == 2 {
                let result_ok = &args.args[0];
                let result_err = &args.args[1];
                if let GenericArgument::Type(result_type) = result_ok {
                    #[allow(clippy::collapsible_match)]
                    if let GenericArgument::Type(syn::Type::Path(it)) = result_err {
                        if let Some(ident) = extract_user_script_associated_type(
                            it.qself.as_ref(),
                            &it.path,
                            generic_params,
                        ) {
                            if let UserScriptBasedType::Named(ident) = ident {
                                if ident == "LangSpecificError" {
                                    return Ok(DataType::Result {
                                        ok: match extract_ty(result_type, generic_params) {
                                            Ok(it) => Box::new(it),
                                            Err(err) => return Err(err),
                                        },
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let syn::PathArguments::None = &segment.arguments else {
        return Err(syn::Error::new_spanned(
            &path.segments,
            "Fyrox Lite: unexpected generics",
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

enum UserScriptBasedType<'a> {
    Itself,
    Named(&'a Ident),
}

fn extract_user_script_associated_type<'a>(
    qself: Option<&'a syn::QSelf>,
    path: &'a syn::Path,
    generic_params: Option<&HashMap<&Ident, DataType>>,
) -> Option<UserScriptBasedType<'a>> {
    if let Some(generic_params) = generic_params {
        if let Some((user_script_type, _data_type)) = generic_params
            .iter()
            .find(|(_ident, data_type)| **data_type == DataType::UserScript)
        {
            if qself.is_none()
                && !path.segments.is_empty()
                && path.segments[0].ident == **user_script_type
            {
                if path.segments.len() == 1 {
                    return Some(UserScriptBasedType::Itself);
                }
                if path.segments.len() == 2 {
                    return Some(UserScriptBasedType::Named(&path.segments[1].ident));
                }
            }
        }
    }
    None
}

pub fn extract_ty(
    ty: &syn::Type,
    generic_params: Option<&HashMap<&Ident, DataType>>,
) -> syn::Result<fyrox_lite_model::DataType> {
    match ty {
        syn::Type::Path(it) => extract_ty_path(it.qself.as_ref(), &it.path, it, generic_params),
        syn::Type::Tuple(it) => {
            if it.elems.is_empty() {
                Ok(DataType::Unit)
            } else {
                Err(syn::Error::new_spanned(
                    ty,
                    "Fyrox Lite: tuples not allowed",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            ty,
            format!("Fyrox Lite: not allowed type: {:?}", ty),
        )),
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
