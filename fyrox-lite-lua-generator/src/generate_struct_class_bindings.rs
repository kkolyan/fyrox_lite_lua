use core::fmt;

use convert_case::{Case, Casing};
use fyrox_lite_model::{DataType, Domain, EngineClass, RustQualifiedName, StructClass};
use std::{collections::HashMap, fmt::Write, ops::Deref};
use to_vec::ToVec;

use crate::{
    code_model::Mod,
    context::GenerationContext,
    expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua},
    templating::render,
};

pub fn generate_struct_class_bindings(class: &StructClass, ctx: &GenerationContext) -> Mod {
    let mut s: String = Default::default();
    render(
        &mut s,
        r#"

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
        };
    "#,
        [],
    );

    generate_into_lua(&mut s, class, ctx);
    generate_from_lua(&mut s, class, ctx);

    Mod {
        name: class.class_name.0.to_case(Case::Snake),
        content: s,
    }
}

fn generate_into_lua(s: &mut String, class: &StructClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
        impl <'lua> mlua::IntoLua<'lua> for Traitor<${rust_struct_path}> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    "#,
        [("rust_struct_path", &class.rust_struct_path.0)],
    );

    for field in class.fields.iter() {
        let expression = rust_expr_to_mlua(ctx, &field.name, &field.ty);
        render(
            s,
            r#"
                    t.set("${field_name}", {
                        let ${field_name} = self.${field_name}.clone();
                        ${expression}
                    })?;
        "#,
            [("field_name", &field.name), ("expression", &expression)],
        );
    }

    render(
        s,
        r#"
                    t
                }))
            }
        }
    "#,
        [],
    );
}

fn generate_from_lua(s: &mut String, class: &StructClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
        impl <'lua> mlua::FromLua<'lua> for Traitor<${rust_struct_path}> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract ${class_name} from {:?}. expected table.", value));
                };
    "#,
        [
            ("rust_struct_path", &class.rust_struct_path.0),
            ("class_name", &class.class_name.0),
        ],
    );

    for field in class.fields.iter() {
        render(
            s,
            r#"
                let ${field_name} = value.get::<_, ${field_type}>("${field_name}")?;
                let ${field_name} = ${expression};
        "#,
            [
                ("field_name", &field.name),
                ("field_type", &type_to_mlua(&field.ty, ctx)),
                ("expression", &mlua_to_rust_expr(&field.name, &field.ty, ctx)),
            ],
        );
    }

    render(
        s,
        r#"
                Ok(Traitor::new(${rust_struct_path} { ${output_values} }))
            }
        }
    "#,
        [
            (
                "output_values",
                &class
                    .fields
                    .iter()
                    .map(|it| it.name.as_str())
                    .to_vec()
                    .join(", "),
            ),
            ("rust_struct_path", &class.rust_struct_path.0),
        ],
    );
}

