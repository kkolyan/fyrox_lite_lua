use std::fs;
use csgen_lib::lite_csgen::gen_rs::RustEmitter;
use gen_common::code_model::Module;
use gen_common::fmt::fmt_file;

fn main() {
    let manuals = csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_fyrox_lite_domain();

    let (mut facade_cs, facade_rs) = csgen_lib::lite_csgen::generate_cs_facade(&domain);
    let (mut internal_cs, internal_rs) = csgen_lib::lite_csgen::generate_base();

    facade_cs.mods.extend(internal_cs.mods);
    facade_cs.mods.extend(manuals.mods);

    csgen_lib::lite_csgen::write_cs::write_cs("cs/FyroxLite/FyroxLite/Auto", facade_cs);
    write_rs_to_file("cs/fyrox-c-lib/src/bindings_lite_2.rs", facade_rs);
    write_rs_to_file("cs/fyrox-c-lib/src/internal_auto.rs", internal_rs);

    // csgen_lib::generate_lite_bindings_cs(&domain);
}

fn write_rs_to_file(path: &str, rust: RustEmitter) {
    let s = format!("
            #![allow(non_camel_case_types)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #![allow(clippy::redundant_locals)]
            #![allow(clippy::useless_conversion)]
            #![allow(clippy::unused_unit)]
            #![allow(clippy::let_unit_value)]
            #![allow(unused)]
            use crate::*;
            use fyrox_lite::externalizable::Externalizable;
            {}
    ", rust.code);

    fs::write(path, s).unwrap();
    fmt_file(path);
}