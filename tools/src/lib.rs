use std::fs;

use lite_model::Domain;
use lite_parser::parse_domain_metadata::parse_domain_metadata;
use luagen_lib::code_model::HierarchicalCodeBase;

pub fn write_annotations(annotations: HierarchicalCodeBase) {
    let target_path = "lua/annotations";
    println!("clearing {}", target_path);
    let _ = fs::remove_dir_all(target_path);
    println!("writing bindings to {}", target_path);
    annotations.write_lua(target_path);
}

pub fn write_bindings(bindings: HierarchicalCodeBase) {
    let target_path = "lua/fyrox-lua/src/generated";
    println!("clearing {}", target_path);
    let _ = fs::remove_dir_all(target_path);
    println!("writing bindings to {}", target_path);
    bindings.write_rust(target_path);
}

pub fn get_combined_domain() -> Domain {
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

    Domain::merge_all([fyrox, math])
}
