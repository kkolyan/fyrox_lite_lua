use core::fmt;

use convert_case::{Case, Casing};
use fyrox_lite_model::{DataType, Domain, EngineClass, RustQualifiedName, StructClass};
use std::{collections::HashMap, fmt::Write, ops::Deref};
use to_vec::ToVec;

use crate::{
    code_model::Mod,
    context::GenerationContext,
    expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua},
    generate_methods::generate_methods,
    templating::render,
};

pub fn generate_struct_class_bindings(it: &StructClass, ctx: &GenerationContext) -> Mod {
    let mut s: String = Default::default();
    render(
        &mut s,
        r#"

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

        impl <'lua> mlua::IntoLua<'lua> for Traitor<${rust_struct_path}> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    "#,
        [("rust_struct_path", &it.rust_struct_path.0)],
    );

    for field in it.fields.iter() {
        let expression = rust_expr_to_mlua(ctx, &field.name, &field.ty);
        render(
            &mut s,
            r#"
                    t.set("${field_name}", {
                        let ${field_name} = self.${field_name};
                        ${expression}
                    })?;
        "#,
            [
                ("field_name", &field.name),
                ("expression", &expression),
            ],
        );
    }

    render(
        &mut s,
        r#"
                    t
                }))
            }
        }
    "#,
        [],
    );

    Mod {
        name: it.class_name.0.to_case(Case::Snake),
        content: s,
    }
}
