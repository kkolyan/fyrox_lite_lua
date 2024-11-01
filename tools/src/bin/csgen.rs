use std::fs;

fn main() {
    csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_fyrox_lite_domain();

    let (codebase, rust) = csgen_lib::lite_csgen::generate_cs_facade(&domain);
    csgen_lib::lite_csgen::write_cs::write_cs(codebase);
    fs::write("cs/fyrox-c/src/bindings_lite_2.rs", rust.code).unwrap();

    // csgen_lib::generate_lite_bindings_cs(&domain);
}