use core::fmt;

use convert_case::{Case, Casing};
use fyrox_lite_model::{DataType, Domain, EngineClass, RustQualifiedName, StructClass};
use std::{collections::HashMap, fmt::Write, ops::Deref};
use to_vec::ToVec;

use crate::{
    code_model::Mod, context::GenerationContext, generate_methods::generate_methods, templating::render
};

pub fn generate_engine_class_bindings(it: &EngineClass, ctx: &GenerationContext) -> Mod {
    let mut s: String = Default::default();

    render(
        &mut s,
        r#"
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for ${rust_struct_path} {
			const CLASS_NAME: &'static str = "${class_name}";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		"#,
        [
            ("rust_struct_path", &it.rust_struct_path.0),
            ("class_name", &it.class_name.0),
        ],
    );
    generate_methods(&mut s, it, ctx, true);

    render(
        &mut s,
        r#"
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	"#,
        [],
    );
    generate_methods(&mut s, it, ctx, false);

    s += "
			}
		}
	";

    Mod {
        name: it.class_name.0.to_case(Case::Snake),
        content: s,
    }
}
