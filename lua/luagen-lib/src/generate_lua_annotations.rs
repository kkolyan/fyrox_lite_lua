use lite_model::{Class, Domain};

use crate::{
    annotations::{
        engine_class::generate_engine, enum_class::generate_enum, struct_class::generate_struct,
    },
    code_model::{HierarchicalCodeBase, ModContent, Module},
    templating::strExt,
    writelnu,
};

pub fn generate_lua_annotations(domain: &Domain) -> HierarchicalCodeBase {
    let mut s = String::new();
    s += "
			-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
			-- More about Lua annotations format: https://luals.github.io/wiki/annotations
			-- This file is auto-generated, do not edit it manually.
			
			---@diagnostic disable: missing-return, lowercase-global, missing-fields

			---@class Script
			---@field node Node
			Script = {}

			-- Used to 
			function script_class() end
		".deindent().as_str();
    for class in domain.classes.iter() {
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
        writelnu!(s, "do");
        match class {
            Class::Engine(class) => generate_engine(&mut s, class),
            Class::Struct(class) => generate_struct(&mut s, class),
            Class::Enum(class) => generate_enum(&mut s, class),
        }
        writelnu!(s, "");
        writelnu!(s, "end");
    }
    HierarchicalCodeBase {
        mods: vec![Module {
            name: "fyrox-lite".into(),
            content: ModContent::Code(s),
        }],
    }
}
