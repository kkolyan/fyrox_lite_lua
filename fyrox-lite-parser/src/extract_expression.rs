use fyrox_lite_model::{
    BinaryOp, Constant, ConstantValue, DataType, EngineClass, EngineClassName, Method, Signature
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, Ident};

use crate::extract_ty::{extract_ty, extract_ty_path};



pub(crate) fn extract_expression(it: &syn::Expr) -> syn::Result<ConstantValue> {
    match &it {
        syn::Expr::Lit(it) => match &it.lit {
            syn::Lit::Str(it) => Ok(ConstantValue::String(it.value().clone())),
            syn::Lit::Int(it) => Ok(ConstantValue::Integer(match it.base10_parse() {
                Ok(it) => it,
                Err(err) => {
                    return Err(err);
                }
            })),
            syn::Lit::Float(it) => Ok(ConstantValue::Float(match it.base10_parse() {
                Ok(it) => it,
                Err(err) => {
                    return Err(err);
                }
            })),
            syn::Lit::Bool(it) => Ok(ConstantValue::Bool(it.value)),
            _ => {
                Err(syn::Error::new_spanned(
                    it,
                    "Fyrox Lite: cannot interpret value as constant",
                ))
            }
        },
        syn::Expr::Binary(it) => {
            let op = match it.op {
                syn::BinOp::Add(_it) => BinaryOp::Add,
                syn::BinOp::Sub(_it) => BinaryOp::Sub,
                syn::BinOp::Mul(_it) => BinaryOp::Mul,
                syn::BinOp::Div(_it) => BinaryOp::Div,
                syn::BinOp::Rem(_it) => BinaryOp::Rem,
                syn::BinOp::And(_it) => BinaryOp::And,
                syn::BinOp::Or(_it) => BinaryOp::Or,
                syn::BinOp::BitXor(_it) => BinaryOp::BitXor,
                syn::BinOp::BitAnd(_it) => BinaryOp::BitAnd,
                syn::BinOp::BitOr(_it) => BinaryOp::BitOr,
                syn::BinOp::Shl(_it) => BinaryOp::Shl,
                syn::BinOp::Shr(_it) => BinaryOp::Shr,
                syn::BinOp::Eq(_it) => BinaryOp::Eq,
                syn::BinOp::Lt(_it) => BinaryOp::Lt,
                syn::BinOp::Le(_it) => BinaryOp::Le,
                syn::BinOp::Ne(_it) => BinaryOp::Ne,
                syn::BinOp::Ge(_it) => BinaryOp::Ge,
                syn::BinOp::Gt(_it) => BinaryOp::Gt,
                _ => {
                    return Err(syn::Error::new_spanned(
                        it,
                        "Fyrox Lite: assign operators not allowed in constants",
                    ));
                }
            };
            Ok(ConstantValue::BinaryOp { op,
                 left: match extract_expression(&it.left) {
                    Ok(it) => Box::new(it),
                    Err(err) => {
                        return Err(err);
                    }
                 }, 
                right: match extract_expression(&it.right) {
                    Ok(it) => Box::new(it),
                    Err(err) => {
                        return Err(err);
                    }
                },
             })
            
        }
        syn::Expr::Path(expr) => {
            let mut owner = expr.path.clone();
            let name = match owner.segments.pop() {
                Some(it) => it.into_value(),
                None => panic!("WTF: no elements in path? {:?}", expr),
            };
            let owner = match extract_ty_path(expr.qself.as_ref(), &owner, expr, None) {
                Ok(it) => match it {
                    DataType::UnresolvedClass(it) => {
                        if it == "Self" {
                            return Err(syn::Error::new_spanned(
                                expr,
                                "Fyrox Lite: Self is not allowed in constant",
                            ));
                        }
                        it
                    },
                    _ => {
                        return Err(syn::Error::new_spanned(
                            expr,
                            "Fyrox Lite: only Engine Class allowed to be owner of constant",
                        ));
                    },
                },
                Err(err) => {
                    return Err(err);
                },
            };
            Ok(ConstantValue::Reference { owner: EngineClassName(owner), constant_name: name.ident.to_string() })
        }
        _ => {
            Err(syn::Error::new_spanned(
                it,
                "Fyrox Lite: only literals allowed in constants",
            ))
        }
    }
}