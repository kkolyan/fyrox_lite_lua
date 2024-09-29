use fyrox_lite_model::{DataType, EngineClass, EngineClassName, Method, Signature};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident};

use crate::extract_ty::extract_ty;

pub fn extract_engine_class(
    attr: TokenStream,
    item: &syn::ItemImpl,
    errors: &mut Vec<syn::Error>,
    types: &mut Vec<syn::Type>,
) -> Option<(Ident, EngineClass)> {
    let rust_name = match extract_ty(&item.self_ty) {
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
    };
    'methods: for it in item.items.iter() {
        match it {
            syn::ImplItem::Fn(it) => {
                let mut instance = false;
                let mut params = Vec::new();
                'params: for it in it.sig.inputs.iter() {
                    let ty = match it {
                        syn::FnArg::Receiver(_receiver) => {
                            instance = true;
                            continue 'params;
                        }
                        syn::FnArg::Typed(pat_type) => pat_type.ty.as_ref(),
                    };
                    types.push(ty.clone());
                    match extract_ty(ty) {
                        Ok(it) => {
                            params.push(it);
                        }
                        Err(err) => {
                            errors.push(err);
                            continue 'methods;
                        }
                    }
                }
                pod.methods.push(Method {
                    method_name: it.sig.ident.to_token_stream().to_string(),
                    instance,
                    signature: Signature {
                        params,
                        return_ty: match &it.sig.output {
                            syn::ReturnType::Default => None,
                            syn::ReturnType::Type(_rarrow, ty) => {
                                types.push(ty.as_ref().clone());
                                Some(match extract_ty(ty) {
                                    Ok(it) => it,
                                    Err(err) => {
                                        errors.push(err);
                                        continue 'methods;
                                    }
                                })
                            }
                        },
                    },
                });
            }
            _ => {
                errors.push(syn::Error::new_spanned(
                    it,
                    "Fyrox Lite: only functions allowed in Engine Class definitions",
                ));
            }
        }
    }
    rust_name.map(|rust_name| (rust_name, pod))
}
