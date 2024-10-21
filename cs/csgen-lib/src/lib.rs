use std::{fs, str::FromStr};

use gen_common::templating::render;
use proc_macro2::TokenStream;
use syn::{parse2, File};
use to_vec::ToVec;

pub mod rust_decl_to_cs;

pub fn generate_manual_bindings_cs() {
    let s = fs::read_to_string("cs/fyrox-c/src/manual_bindings.rs").unwrap();
    let file = parse2::<File>(TokenStream::from_str(s.as_str()).unwrap()).unwrap();

    let code = rust_decl_to_cs::rust_decl_to_c(&file);

    let mut s = String::new();
    render(
        &mut s,
        r#"
            using System.Runtime.InteropServices;

            public partial class FyroxManualBindings {
                ${code}
            }
    "#,
        [("code", &code)],
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

    fs::write("cs/App01/FyroxManualBindings.cs", s).unwrap()
}
