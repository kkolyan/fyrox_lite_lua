use std::{fmt::Display, fs, str::FromStr};

use gen_common::templating::render;
use lite_model::Domain;
use proc_macro2::TokenStream;
use syn::{parse2, File};
use to_vec::ToVec;
use crate::lite_cgen::CBindingsLite;

pub mod rust_decl_to_cs;
pub mod lite_cgen;
pub mod lite_csgen;

fn generate_and_write_bindings_cs(class: &str, s: CBindingsLite) {
    let file = parse2::<File>(TokenStream::from_str(&s.code_rs).unwrap()).unwrap();

    let code = rust_decl_to_cs::rust_decl_to_c(&file, &s.generated_structs);

    let mut s = String::new();
    render(
        &mut s,
        r#"
            using System.Runtime.InteropServices;
            using FyroxLite;
            using FyroxLite.LiteMath;

            internal partial class ${class} {
                ${code}
            }
    "#,
        [
            ("class", &class),
            ("code", &code),
        ],
    );

    s = s
        .lines()
        .filter(|it| !it.is_empty())
        .map(|it| {
            it.strip_prefix("            ")
                .expect("12 space indent expected")
        })
        .to_vec()
        .join("\n");

    fs::write(format!("cs/App01/{}.cs", class), s).unwrap()
}

pub fn generate_manual_bindings_cs() {
    let s = fs::read_to_string("cs/fyrox-c/src/bindings_manual.rs").unwrap();

    generate_and_write_bindings_cs("FyroxManualBindings", CBindingsLite {
        code_rs: s,
        generated_structs: Default::default(),
    });
}

pub fn generate_lite_bindings_cs(domain: &Domain) {
    let s = lite_cgen::generate_c_bindings_lite(domain);
    let file = "cs/fyrox-c/src/bindings_lite.rs";
    fs::write(file, &s.code_rs).unwrap();
    gen_common::fmt::fmt_file(file);

    generate_and_write_bindings_cs("FyroxLiteBindings", s);
}
