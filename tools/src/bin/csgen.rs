use std::fs;
use gen_common::fmt::fmt_file;

fn main() {
    csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_fyrox_lite_domain();

    let (codebase, rust) = csgen_lib::lite_csgen::generate_cs_facade(&domain);
    csgen_lib::lite_csgen::write_cs::write_cs(codebase);
    let s = format!("
            #![allow(non_camel_case_types)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #![allow(clippy::redundant_locals)]
            use crate::bindings_manual::*;
            use fyrox_lite::externalizable::Externalizable;
            {}
    ", rust.code);

    let path = "cs/fyrox-c/src/bindings_lite_2.rs";
    fs::write(path, s).unwrap();
    fmt_file(path);

    // csgen_lib::generate_lite_bindings_cs(&domain);
}