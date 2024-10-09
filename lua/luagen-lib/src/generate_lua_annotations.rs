use lite_model::{Class, Domain};

use crate::{
    annotations::{
        engine_class::generate_engine, enum_class::generate_enum, struct_class::generate_struct,
    }, by_package::classes_by_package, code_model::{HierarchicalCodeBase, ModContent, Module}, templating::strExt, writelnu
};

pub fn generate_lua_annotations(domain: &Domain) -> HierarchicalCodeBase {
    let mut mods = vec![];

    mods.push(Module::code("Script", "
			-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
			-- More about Lua annotations format: https://luals.github.io/wiki/annotations
			-- This file is auto-generated, do not edit it manually.
			
			---@diagnostic disable: missing-return, lowercase-global, missing-fields

			---@class Script
			---@field node Node
			Script = {}

			-- Used to 
			function script_class() end
		".deindent()));

    let by_package = classes_by_package(domain);
    for (package, classes) in by_package {

        let mut package_mods = vec![];

        for class in classes {
            let class = domain.get_class(&class).unwrap();
            let mut s = String::new();
            writelnu!(s, "");
            writelnu!(
                s,
                "-----------------------------------------------------------"
            );
            writelnu!(s, "------ {}", class.rust_name());
            writelnu!(
                s,
                "-----------------------------------------------------------"
            );
            match class {
                Class::Engine(class) => generate_engine(&mut s, class),
                Class::Struct(class) => generate_struct(&mut s, class),
                Class::Enum(class) => generate_enum(&mut s, class),
            }
            writelnu!(s, "");

            package_mods.push(Module::code(class.class_name(), s));
        }

        mods.push(Module::children(package, package_mods));
    }
    HierarchicalCodeBase {
        mods: vec![Module {
            name: "fyrox-lite".into(),
            content: ModContent::Children(mods),
        }],
    }
}
