fn main() {
    csgen_lib::generate_manual_bindings_cs();
    
    println!("parsing domain");
    let domain = tools::get_fyrox_lite_domain();
    csgen_lib::generate_lite_bindings_cs(&domain);
}