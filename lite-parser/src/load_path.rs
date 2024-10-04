use std::{collections::HashMap, fs, path::Path, str::FromStr};

use crate::{
    extract_engine_class::extract_engine_class_and_inject_assertions,
    extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct, RustSymbol,
};
use lite_model::{Class, Domain};
use proc_macro2::TokenStream;
use syn::parse2;

pub fn load_path(crate_name: &str, path: &Path, domain: &mut Domain, aliases: &mut HashMap<String, RustSymbol>) {
    let text = fs::read_to_string(path).unwrap();
    let file = parse2::<syn::File>(TokenStream::from_str(&text).unwrap()).unwrap();

    let mod_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".rs", "");
    let root_mod_name = crate_name.replace("-", "_");
    let mod_name = if mod_name == "lib" {
        root_mod_name
    } else {
        format!("{}::{}", root_mod_name, mod_name)
    };
    println!("mod name: {}", mod_name);

    for item in file.items {
        match item {
            syn::Item::Impl(mut it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) = extract_engine_class_and_inject_assertions(
                        &mod_name,
                        attr,
                        &mut it,
                        &mut vec![],
                    ) {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Engine(class));
                    }
                }
            }
            syn::Item::Struct(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) =
                        extract_pod_struct(&mod_name, attr, &it, &mut vec![])
                    {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Struct(class));
                    }
                }
            }
            syn::Item::Enum(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) =
                        extract_pod_enum(&mod_name, attr, &it, &mut vec![], &mut vec![])
                    {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Enum(class));
                    }
                }
            }
            _ => {}
        }
    }
}

fn extract_attr(
    attrs: &[syn::Attribute],
    attr_name: &str
) -> Option<Option<TokenStream>> {
    let attr = attrs.iter().find(|it| {
        it.path()
            .get_ident()
            .map(|it| it == attr_name)
            .unwrap_or_default()
    });
    let attr = attr?;
    match &attr.meta {
        syn::Meta::Path(_it) => Some(None),
        syn::Meta::List(it) =>  Some(Some(it.tokens.clone())),
        syn::Meta::NameValue(_it) => panic!("usage: #[lite_api] or #[lite_api(MyClass)]"),
    }
}
