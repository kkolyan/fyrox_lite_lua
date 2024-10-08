
use luagen_lib::{generate_lua_annotations::generate_lua_annotations, generate_lua_bindings::generate_lua_bindings};
use tools::{get_combined_domain, write_annotations, write_bindings};

fn main() {

    println!("parsing domain");
    let domain = get_combined_domain();

    println!("generating bindings");
    let bindings = generate_lua_bindings(&domain);

    println!("generating annotations");
    let annotations = generate_lua_annotations(&domain);

    write_bindings(bindings);
    write_annotations(annotations);
}