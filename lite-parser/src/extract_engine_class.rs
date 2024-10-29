use std::collections::HashMap;

use lite_model::{
    ClassName, Constant, ConstantValue, DataType, EngineClass, Literal, Method, Param, RustQualifiedName, Signature
};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{parse_quote_spanned, spanned::Spanned, Expr, Ident, ImplItemFn, ItemFn, TraitBoundModifier, TypeParamBound};

use crate::{extract_ty::extract_ty, lite_api_attr::LiteApiAttr};

pub fn extract_engine_class_and_inject_assertions(
    rust_path: &str,
    (attr, _attr_span): (LiteApiAttr, Span),
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

    let mut constants: Vec<Constant> = Default::default();
    let mut methods: Vec<Method> = Default::default();

    'items: for it in item.items.iter_mut() {
        match it {
            syn::ImplItem::Fn(func) => {
                if let Some(method) = extract_fn(func, errors) {
                    let buffer_param = method.signature.params.iter().find(|it| matches!(it.ty, DataType::Buffer(_))).map(|it| it.ty.clone());
                    if buffer_param.is_some() && buffer_param != method.signature.return_ty {
                        errors.push(syn::Error::new_spanned(
                            &func.sig.output,
                            r#"Fyrox Lite: methods with Buffer parameter should return this
                            buffer parameter value. that's necessary to comply with Lite's "no borrowing" rule#)"#,
                        ));
                    }
                    methods.push(method);
                }
            }
            syn::ImplItem::Const(it) => {
                constants.push(Constant {
                    const_name: it.ident.to_string(),
                    ty: match extract_ty(&it.ty, None) {
                        Ok(it) => it,
                        Err(err) => {
                            errors.push(err);
                            continue 'items;
                        }
                    },
                    value: parse_constant_value(&it.expr),
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
    rust_name.map(|rust_name| {
        let class_name = attr.class.unwrap_or_else(|| rust_name.to_string());
        (
            rust_name.clone(),
            EngineClass {
                parent: None,
                class_name: ClassName(class_name),
                methods,
                constants,
                rust_struct_path: RustQualifiedName(format!("{}::{}", rust_path, rust_name)),
                features: attr.features,
            },
        )
    })
}

pub fn extract_fn(
    func: &mut ImplItemFn,
    errors: &mut Vec<syn::Error>,
) -> Option<Method> {

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
    let arg_count = func.sig.inputs.len();
    'params: for (i, fn_arg) in func.sig.inputs.iter_mut().enumerate() {
        let arg = match fn_arg {
            syn::FnArg::Receiver(_receiver) => {
                instance = true;
                continue 'params;
            }
            syn::FnArg::Typed(pat_type) => pat_type,
        };
        let is_class_name = if arg
            .attrs
            .iter()
            .any(|it| it.meta.path().is_ident("class_name"))
        {
            arg.attrs
                .retain(|it| !it.meta.path().is_ident("class_name"));
            true
        } else {
            false
        };
        let ty = arg.ty.as_ref();
        types.push(ty.clone());
        match extract_ty(ty, Some(&generic_params)) {
            Ok(mut it) => {
                if is_class_name {
                    if !matches!(it, DataType::String) {
                        errors.push(syn::Error::new_spanned(
                            fn_arg,
                            "Fyrox Lite: only String parameter could be a class name",
                        ));
                        return None;
                    }
                    it = DataType::ClassName;
                }
                // handle #[variadic]
                let variadic_index = arg
                    .attrs
                    .iter_mut()
                    .enumerate()
                    .find(|(_i, it)| {
                        it.path()
                            .get_ident()
                            .map(|it| it == "variadic")
                            .unwrap_or_default()
                    })
                    .map(|(i, _)| i);
                let variadic = match variadic_index {
                    Some(index) => {
                        arg.attrs.remove(index);
                        true
                    }
                    None => false,
                };
                if variadic && i != arg_count - 1 {
                    errors.push(syn::Error::new_spanned(
                        fn_arg,
                        "Fyrox Lite: only the last parameter could be variadic",
                    ));
                    return None;
                }
                params.push(Param {
                    name: match arg.pat.as_ref() {
                        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                        _ => {
                            errors.push(syn::Error::new_spanned(
                                arg,
                                "Fyrox Lite: identifier should be used as method arg name",
                            ));
                            return None;
                        }
                    },
                    ty: it,
                    variadic,
                });
            }
            Err(err) => {
                errors.push(err);
                return None;
            }
        }
    }
    let static_assertions = types
        .iter()
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
    Some(Method {
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
                            return None;
                        }
                    })
                }
            },
        },
    })
}

fn parse_constant_value(it: &Expr) -> ConstantValue {
    ConstantValue::Literal(match it {
        Expr::Lit(it) => match &it.lit {
            syn::Lit::Str(it) => Literal::String(it.value()),
            syn::Lit::Byte(it) => Literal::Byte(it.value()),
            syn::Lit::Int(it) => Literal::Number(it.base10_digits().to_string()),
            syn::Lit::Float(it) => Literal::Number(it.base10_digits().to_string()),
            syn::Lit::Bool(it) => Literal::Bool(it.value),
            it => return ConstantValue::ComplexExpression(it.to_token_stream().to_string()),
        }
        it => return ConstantValue::ComplexExpression(it.to_token_stream().to_string()),
    })
}