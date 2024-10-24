fn main() {
    csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_combined_domain();
    csgen_lib::generate_lite_bindings_cs(&domain);
}