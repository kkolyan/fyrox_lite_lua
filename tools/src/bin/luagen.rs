use std::fs;

use luagen_lib::generate_lua_bindings::generate_lua_bindings;
use lite_model::Domain;
use lite_parser::parse_domain_metadata::parse_domain_metadata;

fn main() {
    let mut fyrox: Domain = parse_domain_metadata("fyrox-lite");
    let math: Domain = parse_domain_metadata("fyrox-lite-math");

    // math "overrides" classes in fyrox by name
    fyrox.classes.retain_mut(|fyrox_class| {
        let override_class = math.get_class(fyrox_class.class_name());
        if let Some(override_class) = override_class {
            println!(
                "overriding {} by {}",
                fyrox_class.rust_name(),
                override_class.rust_name()
            );
        }
        override_class.is_none()
    });

    let domain = Domain::merge_all([fyrox, math]);

    println!("generating bindings");

    let code_base = generate_lua_bindings(domain);

    let target_path = "lua/fyrox-lua/src/generated";
    println!("clearing {}", target_path);
    fs::remove_dir_all(target_path).unwrap();
    println!("writing bindings to {}", target_path);
    code_base.write(target_path);
}