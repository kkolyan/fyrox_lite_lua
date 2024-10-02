use std::{collections::HashMap, fs, path::Path, str::FromStr};

use fyrox_lite_model::{Domain, Class};
use crate::{extract_engine_class::extract_engine_class_and_inject_assertions, extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct, RustSymbol};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;


pub fn load_path(path: &Path, domain: &mut Domain, aliases: &mut HashMap<String, RustSymbol>) {
    let text = fs::read_to_string(path).unwrap();
    let file = parse2::<syn::File>(TokenStream::from_str(&text).unwrap()).unwrap();

    let mod_name = path.file_name().unwrap().to_str().unwrap().replace(".rs", "");

    for item in file.items {
        match item {
            syn::Item::Impl(mut it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_engine_class_and_inject_assertions(&mod_name, attr, &mut it, &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.classes.push(Class::Engine(class));
                    }
                }
            },
            syn::Item::Struct(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_pod_struct(&mod_name,attr, &it, &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.classes.push(Class::Struct(class));
                    }
                }
            },
            syn::Item::Enum(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_pod_enum(&mod_name,attr, &it, &mut vec![], &mut vec![]) {
                        aliases.insert(class.class_name.0.clone(), RustSymbol(rust_name.to_string().clone()));
                        domain.classes.push(Class::Enum(class));
                    }
                }
            },
            _ => {}
        }
    }
}

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
