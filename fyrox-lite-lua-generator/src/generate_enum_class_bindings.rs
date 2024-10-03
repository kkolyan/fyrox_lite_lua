use core::fmt;

use convert_case::{Case, Casing};
use fyrox_lite_model::{
    DataType, Domain, EngineClass, EnumClass, EnumValue, Field, Method, RustQualifiedName, StructClass
};
use std::{borrow::Cow, collections::HashMap, fmt::Write, ops::Deref};
use to_vec::ToVec;

use crate::{
    code_model::Mod,
    context::GenerationContext,
    expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua},
    generate_methods::{generate_methods, is_getter, is_setter},
    templating::render,
};

pub fn generate_enum_class_bindings(class: &EnumClass, ctx: &GenerationContext) -> Mod {
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
			lua_error,
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

    // class methods: one for each struct/tuple variant to create instance
    // class fields: one for each unit variant
    // instance methods: __tostring metamethod
    // instance fields: one for each variant. for struct it returns table with fields,
    // 		for tuple the table again, with fields like "_1", "_2", "_3", for unit - `true` boolean value`.

    generate_class_methods(&mut s, class, ctx);
    generate_class_fields(&mut s, class, ctx);
    // generate_instance_methods(&mut s, class, ctx);
    // generate_instance_fields(&mut s, class, ctx);

    s += "
		}
	";

    Mod {
        name: class.class_name.0.to_case(Case::Snake),
        content: s,
    }
}

fn generate_class_fields(s: &mut String, class: &EnumClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
	"#,
        [],
    );

	unit_accessors(s, class, ctx);


    *s += "
			}
	";
}

fn unit_accessors(s: &mut String, class: &EnumClass, ctx: &GenerationContext) {
	for variant in class.variants.iter() {
		let EnumValue::Unit = &variant.value else {
			continue;
		};
		
        render(
            s,
            r#"				
				fields.add_field_method_get("${field_name}", |lua, this| {
					Ok(Traitor::new(${rust_struct_path}::${variant_name}))
				});
		"#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &variant.tag),
                ("variant_name", &variant.tag),
            ],
        );
	}
}

fn generate_class_methods(s: &mut String, class: &EnumClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	"#,
        [],
    );
    for variant in class.variants.iter() {
		if let EnumValue::Unit = &variant.value {
			continue;
		}
        render(
            s,
            r#"
				methods.add_method_mut(
					"${method_name}",
					|lua, this, mut args: mlua::MultiValue| {
			"#,
            [("method_name", &variant.tag)],
        );
        match &variant.value {
            EnumValue::Unit => {}
            EnumValue::Tuple { fields } => tuple_construction(s, class, &variant.tag, fields, ctx),
            EnumValue::Struct { fields } => struct_construction(s, class, &variant.tag, fields, ctx),
        }

        *s += "
					}
				);
		";
    }

    render(
        s,
        r#"
			}
	"#,
        [],
    );
}

fn tuple_construction(
    s: &mut String,
    class: &EnumClass,
    tag: &str,
    fields: &[DataType],
    ctx: &GenerationContext,
) {
    let mut output_names = Vec::new();
    for (i, ty) in fields.iter().enumerate() {
        let var = format!("_{}", i + 1);
        let expression = mlua_to_rust_expr(&var, ty, ctx);
        render(
            s,
            r#"
						let Some(${var}) = args.pop_front() else {
							return Err(lua_error!("argument ${index} (${display_type_name}) missing"));
						};
						let ${var} = <${field_type} as mlua::FromLua>::from_lua(${var}, lua)?;
						let ${var} = ${expression};
			"#,
            [
                ("index", &(i + 1)),
                ("var", &var),
                ("expression", &expression),
                ("display_type_name", &format!("{}", ty)),
                ("field_type", &type_to_mlua(ty, ctx)),
            ],
        );
        output_names.push(var);
    }

    render(
        s,
        r#"
						let value = ${rust_struct_path}::${variant_name}( ${output_names} );
						Ok(Traitor::new(value))
		"#,
        [
            ("rust_struct_path", &class.rust_struct_path.0),
            ("variant_name", &tag),
            ("output_names", &output_names.join(", ")),
        ],
    );
}

fn struct_construction(
    s: &mut String,
    class: &EnumClass,
    tag: &str,
    fields: &[Field],
    ctx: &GenerationContext,
) {
    let mut output_names = Vec::new();
	render(
		s,
		r#"
						if args.len() != 1 {
							return Err(lua_error!("exactly one argument expected, but {} supplied", args.len()));
						}
						let value = args.pop_front().expect("WTF, just checket the size");
						let mlua::Value::Table(value) = value else {
							return Err(lua_error!("${display_type_name}-typed table expected, but {:?} supplied", value));
						};
		"#,
		[
			("display_type_name", &format!("{}::{}", class.class_name.0, tag)),
		],
	);
    for field in fields.iter() {
        render(
            s,
            r#"
						let ${field_name} = value.get::<_, ${field_type}>("${field_name}")?;
						let ${field_name} = ${expression};
			"#,
            [
                ("field_name", &field.name),
                ("expression", & mlua_to_rust_expr(&field.name, &field.ty, ctx)),
                ("field_type", &type_to_mlua(&field.ty, ctx)),
            ],
        );
        output_names.push(field.name.as_str());
    }

    render(
        s,
        r#"
						let value = ${rust_struct_path}::${variant_name} { ${output_names} };
						Ok(Traitor::new(value))
		"#,
        [
            ("rust_struct_path", &class.rust_struct_path.0),
            ("variant_name", &tag),
            ("output_names", &output_names.join(", ")),
        ],
    );
}
