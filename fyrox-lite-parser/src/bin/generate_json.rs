use std::{collections::HashMap, fs, path::Path, str::FromStr};

use fyrox_lite_model::{DataType, Domain, EngineClass, Field, PodClass, PodClassName};
use fyrox_lite_parser::{extract_engine_class::extract_engine_class_and_inject_assertions, extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct, extract_ty::extract_ty, load_path::load_path, resolve_classes::resolve_classes};
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

    resolve_classes(&mut domain, &mut aliases);
    let s = serde_json::to_string_pretty(&domain).unwrap();
    fs::write("fyrox-lite/fyrox-lite.json", s).unwrap();
}
