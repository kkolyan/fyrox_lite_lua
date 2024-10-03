use core::fmt;

use convert_case::{Case, Casing};
use fyrox_lite_model::{DataType, Domain, EngineClass, Method, RustQualifiedName, StructClass};
use std::{collections::HashMap, fmt::Write, ops::Deref};
use to_vec::ToVec;

use crate::{
    code_model::Mod,
    context::GenerationContext,
    expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua},
    generate_methods::{generate_methods, is_getter, is_setter},
    templating::render,
};

pub fn generate_engine_class_bindings(class: &EngineClass, ctx: &GenerationContext) -> Mod {
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
		"#,
        [
            ("class_name", &class.class_name.0),
            ("rust_struct_path", &class.rust_struct_path.0),
        ],
    );

    generate_instance_methods(&mut s, class, ctx);
    generate_class_methods(&mut s, class, ctx);
    generate_instance_fields(&mut s, class, ctx);
    generate_class_fields(&mut s, class, ctx);

    s += "
		}
	";

    Mod {
        name: class.class_name.0.to_case(Case::Snake),
        content: s,
    }
}

fn generate_class_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	"#,
        [],
    );
    generate_methods(s, class, ctx, false);

    render(
        s,
        r#"
			}
	"#,
        [],
    );
}

fn generate_instance_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		"#,
        [],
    );
    generate_methods(s, class, ctx, true);

    render(
        s,
        r#"
			}
	"#,
        [],
    );
}

fn generate_instance_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	"#,
        [],
    );

    generate_instance_getters(s, class, ctx);

    generate_instance_setters(s, class, ctx);

    render(
        s,
        r#"
			}
	"#,
        [],
    );
}

fn generate_instance_setters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    let writable_props = class
        .methods
        .iter()
        .filter(|it| is_setter(it))
        .map(|method| (method.method_name.strip_prefix("set_").unwrap(), method))
        .to_vec();

    for (prop, setter) in writable_props {
        render(
            s,
            r#"
				fields.add_field_method_set("${field_name}", |lua, this, value: ${input_type}| {
					this.set_${field_name}(${expression});
					Ok(())
				});
		"#,
            [
                ("field_name", &prop),
                (
                    "input_type",
                    &type_to_mlua(&setter.signature.params[0].ty, ctx),
                ),
                (
                    "expression",
                    &mlua_to_rust_expr("value", &setter.signature.params[0].ty, ctx),
                ),
            ],
        );
    }
}

fn generate_instance_getters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    let readable_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_props {
        render(
            s,
            r#"
				fields.add_field_method_get("${field_name}", |lua, this| {
					let value = this.get_${field_name}();
					Ok(${expression})
				});
		"#,
            [
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }
}

fn generate_class_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
	"#,
        [],
    );

    for constant in class.constants.iter() {
        render(
            s,
            r#"				
				fields.add_field_method_get("${field_name}", |lua, this| {
					let value = ${rust_struct_path}::${field_name};
					Ok(${expression})
				});
		"#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &constant.const_name),
                ("expression", &rust_expr_to_mlua(ctx, "value", &constant.ty)),
            ],
        );
    }

    let readable_static_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| !it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_static_props {
        render(
            s,
            r#"				
				fields.add_field_method_get("${field_name}", |lua, this| {
					let value = ${rust_struct_path}::get_${field_name}();
					Ok(${expression})
				});
		"#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }

    *s += "
			}
	";
}
