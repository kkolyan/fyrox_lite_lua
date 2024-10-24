use std::{fmt::Display, fs, str::FromStr};

use gen_common::templating::render;
use lite_model::Domain;
use proc_macro2::TokenStream;
use syn::{parse2, File};
use to_vec::ToVec;

pub mod rust_decl_to_cs;
pub mod lite_cgen;

fn write_bindings_cs(class: &str, s: &str) {
    let file = parse2::<File>(TokenStream::from_str(s).unwrap()).unwrap();

    let code = rust_decl_to_cs::rust_decl_to_c(&file);

    let mut s = String::new();
    render(
        &mut s,
        r#"
            using System.Runtime.InteropServices;

            public partial class ${class} {
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

    write_bindings_cs("FyroxManualBindings", &s);
}

pub fn generate_lite_bindings_cs(domain: &Domain) {
    let s = lite_cgen::generate_c_bindings_lite(domain);
    fs::write("cs/fyrox-c/src/bindings_lite.rs", &s).unwrap();

    write_bindings_cs("FyroxLiteBindings", &s);

}
