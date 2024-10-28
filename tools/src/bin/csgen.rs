fn main() {
    csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_fyrox_lite_domain();

    let codebase = csgen_lib::lite_csgen::generate_cs_facade(&domain);
    csgen_lib::lite_csgen::write_cs::write_cs(codebase);

    csgen_lib::generate_lite_bindings_cs(&domain);
}