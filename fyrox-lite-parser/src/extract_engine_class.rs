use std::collections::HashMap;

use fyrox_lite_model::{
    BinaryOp, Constant, ConstantValue, DataType, EngineClass, EngineClassName, Method, Signature,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Ident, TraitBoundModifier, TypeParamBound
};

use crate::{
    extract_expression::extract_expression,
    extract_ty::{extract_ty, extract_ty_path},
};

pub fn extract_engine_class_and_inject_assertions(
    attr: TokenStream,
    item: &mut syn::ItemImpl,
    errors: &mut Vec<syn::Error>,
) -> Option<(Ident, EngineClass)> {
    let rust_name = match extract_ty(&item.self_ty, None) {
        Ok(it) => {
            if let DataType::UnresolvedClass(it) = it {
                Some(Ident::new(it.as_str(), item.self_ty.span()))
            } else {
                errors.push(syn::Error::new_spanned(
                    &item.self_ty,
                    "Fyrox Lite: engine class cannot be another Lite type",
                ));
                None
            }
        }
        Err(err) => {
            errors.push(err);
            None
        }
    };

    let class_name = attr.to_string();
    let mut pod = EngineClass {
        parent: None,
        class_name: EngineClassName(class_name),
        methods: Default::default(),
        constants: Default::default(),
    };
    'items: for it in item.items.iter_mut() {
        match it {
            syn::ImplItem::Fn(func) => {
                let mut types: Vec<syn::Type> = Default::default();
                let mut generic_params: HashMap<&Ident, DataType> = Default::default();
                for gp in func.sig.generics.params.iter() {
                    match gp {
                        syn::GenericParam::Type(param) => {
                            if param.bounds.len() == 1 {
                                let bound =
                                    param.bounds.iter().next().expect("WTF, just checked size");
                                if let TypeParamBound::Trait(it) = bound {
                                    if it.lifetimes.is_none() {
                                        if let TraitBoundModifier::None = it.modifier {
                                            if it.path.to_token_stream().to_string() == "UserScript"
                                                && !generic_params.contains_key(&param.ident)
                                            {
                                                generic_params
                                                    .insert(&param.ident, DataType::UserScript);
                                                continue;
                                            }
                                            if it.path.to_token_stream().to_string()
                                                == "UserScriptMessage"
                                                && !generic_params.contains_key(&param.ident)
                                            {
                                                generic_params.insert(
                                                    &param.ident,
                                                    DataType::UserScriptMessage,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }

                            errors.push(syn::Error::new_spanned(
                                gp,
                                "Fyrox Lite: unexpected generic param",
                            ));
                        }
                        _ => {
                            errors.push(syn::Error::new_spanned(
                                gp,
                                "Fyrox Lite: lifetime and const generic params are not allowed",
                            ));
                        }
                    }
                }
                let mut instance = false;
                let mut params = Vec::new();
                'params: for it in func.sig.inputs.iter() {
                    let ty = match it {
                        syn::FnArg::Receiver(_receiver) => {
                            instance = true;
                            continue 'params;
                        }
                        syn::FnArg::Typed(pat_type) => pat_type.ty.as_ref(),
                    };
                    types.push(ty.clone());
                    match extract_ty(ty, Some(&generic_params)) {
                        Ok(it) => {
                            params.push(it);
                        }
                        Err(err) => {
                            errors.push(err);
                            continue 'items;
                        }
                    }
                }
                pod.methods.push(Method {
                    method_name: func.sig.ident.to_token_stream().to_string(),
                    instance,
                    signature: Signature {
                        params,
                        return_ty: match &func.sig.output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_rarrow, ty) => {
                                types.push(ty.as_ref().clone());
                                Some(match extract_ty(ty, Some(&generic_params)) {
                                    Ok(it) => it,
                                    Err(err) => {
                                        errors.push(err);
                                        continue 'items;
                                    }
                                })
                            }
                        },
                    },
                });
                let static_assertions = types
                    .into_iter()
                    .map::<syn::Stmt, _>(|ty| {
						let span = ty.span();
                        parse_quote_spanned! {span =>
                            {
								#[allow(unused_imports)]
                                use crate::LiteDataType;
                                <#ty>::compiles_if_type_is_allowed();
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                func.block.stmts.splice(0..0, static_assertions);
            }
            syn::ImplItem::Const(it) => {
                pod.constants.push(Constant {
                    const_name: it.ident.to_string(),
                    ty: match extract_ty(&it.ty, None) {
                        Ok(it) => it,
                        Err(err) => {
                            errors.push(err);
                            continue 'items;
                        }
                    },
                    value: match extract_expression(&it.expr) {
                        Ok(it) => it,
                        Err(err) => {
                            errors.push(err);
                            continue 'items;
                        }
                    },
                });
            }
            _ => {
                errors.push(syn::Error::new_spanned(
                    it,
                    "Fyrox Lite: only functions and constants allowed in Engine Class definitions",
                ));
            }
        }
    }
    rust_name.map(|rust_name| (rust_name, pod))
}

fn extract_ty_generic_param(
    ty: &syn::Type,
    generic_params: &HashMap<&Ident, DataType>,
) -> Option<DataType> {
    if generic_params.is_empty() {
        return None;
    }
    let syn::Type::Path(it) = ty else {
        return None;
    };
    if it.qself.is_some() {
        return None;
    }
    let it = it.path.get_ident()?;
    generic_params.get(it).cloned()
}
