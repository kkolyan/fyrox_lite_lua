use std::{fmt::Display, fs, str::FromStr};
use std::collections::HashSet;
use gen_common::templating::render;
use lite_model::Domain;
use proc_macro2::TokenStream;
use syn::{parse2, File};
use to_vec::ToVec;
use gen_common::code_model::HierarchicalCodeBase;
use crate::lite_cgen::CBindingsLite;

pub mod rust_decl_to_cs;
pub mod lite_cgen;
pub mod lite_csgen;

pub fn generate_manual_bindings_cs() -> HierarchicalCodeBase {
    let s = fs::read_to_string("cs/fyrox-c/src/bindings_manual.rs").unwrap();
    let file = parse2::<File>(TokenStream::from_str(&s).unwrap()).unwrap();

    rust_decl_to_cs::rust_decl_to_c(&file, &HashSet::new())
}
