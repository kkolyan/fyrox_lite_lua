use std::{collections::HashMap, fs, path::Path, str::FromStr};

use fyrox_lite_model::{DataType, Domain, EngineClass, Field, PodClass, PodClassName};
use fyrox_lite_parser::{extract_ty::extract_ty, visit_impl::extract_engine_class, visit_struct::{extract_pod_struct}};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, Ident, TraitBoundModifier};

fn main() {
    let mut domain = Domain::default();
    let mut aliases = Default::default();
    let dir = "fyrox-lite/src";
    for entry in fs::read_dir(dir).unwrap().flatten() {
        if entry.file_type().unwrap().is_dir() {
            continue;
        }
        if !entry.file_name().to_string_lossy().ends_with(".rs") {
            continue;
        }
        load_path(&entry.path(), &mut domain, &mut aliases);
    }
}

fn load_path(path: &Path, domain: &mut Domain, aliases: &mut HashMap<Ident, String>) {
    let text = fs::read_to_string(path).unwrap();
    let file = parse2::<syn::File>(TokenStream::from_str(&text).unwrap()).unwrap();

    for item in file.items {
        match item {
            syn::Item::Enum(it) => {},
            syn::Item::Impl(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite_engine_class", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_engine_class(attr, &it, &mut vec![], &mut vec![]) {
                        aliases.insert(rust_name.clone(), class.class_name.0.clone());
                        domain.engine_classes.push(class);
                    }
                }
            },
            syn::Item::Struct(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "fyrox_lite_pod", &mut vec![]) {
                    if let Some((rust_name, class)) = extract_pod_struct(attr, &it, &mut vec![]) {
                        aliases.insert(rust_name.clone(), class.class_name.0.clone());
                        domain.pod_classes.push(class);
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
    
    let pod = attrs
        .iter()
        .find(|it| it.path().to_token_stream().to_string() == attr_name);
    let pod = pod?;
    match pod.meta.require_list() {
        Ok(it) => Some(it.to_token_stream()),
        Err(err) => {
            errors.push(err);
            None
        },
    }
}
