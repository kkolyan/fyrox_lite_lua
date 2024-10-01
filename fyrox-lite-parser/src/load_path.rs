use std::{collections::HashMap, fs, path::Path, str::FromStr};

use fyrox_lite_model::{ClassName, DataType, Domain, DomainItem, EngineClass, Field, StructClass};
use crate::{extract_engine_class::extract_engine_class_and_inject_assertions, extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct, extract_ty::extract_ty, RustSymbol};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, Ident, TraitBoundModifier};


pub fn load_path(path: &Path, domain: &mut Domain, aliases: &mut HashMap<String, RustSymbol>) {
    let text = fs::read_to_string(path).unwrap();
    let file = parse2::<syn::File>(TokenStream::from_str(&text).unwrap()).unwrap();

    for item in file.items {
        match item {
            syn::Item::Impl(mut it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_engine_class_and_inject_assertions(attr, &mut it, &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.items.push(DomainItem::EngineClass(class));
                    }
                }
            },
            syn::Item::Struct(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_pod_struct(attr, &it, &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.items.push(DomainItem::StructClass(class));
                    }
                }
            },
            syn::Item::Enum(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_pod_enum(attr, &it, &mut vec![], &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.items.push(DomainItem::EnumClass(class));
                    }
                }
            },
            syn::Item::Trait(it) => {},
            _ => {}
        }
    }
}



fn visit_enum(it: &syn::ItemEnum, domain: &mut Domain) {}

fn extract_attr(attrs: &[syn::Attribute], attr_name: &str, errors: &mut Vec<syn::Error>) -> Option<TokenStream> {
    
    let attr = attrs
        .iter()
        .find(|it| it.path().to_token_stream().to_string() == attr_name);
    let attr = attr?;
    match attr.meta.require_list() {
        Ok(it) => Some(it.tokens.to_token_stream()),
        Err(err) => {
            errors.push(err);
            None
        },
    }
}
