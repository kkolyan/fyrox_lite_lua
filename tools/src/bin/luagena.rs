
use luagen_lib::
    generate_lua_annotations::generate_lua_annotations
;
use tools::{get_combined_domain, write_annotations};

fn main() {
    println!("parsing domain");
    let domain = get_combined_domain();

    println!("generating annotations");
    let annotations = generate_lua_annotations(&domain);
    write_annotations(annotations);
}