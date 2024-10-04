
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for fyrox_lite::lite_event::MouseButton {
    const CLASS_NAME: &'static str = "MouseButton";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Other", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (i32) missing"));
            };
            let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_event::MouseButton::Other(_1);
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Left", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::MouseButton::Left))
        });

        fields.add_field_method_get("Right", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::MouseButton::Right))
        });

        fields.add_field_method_get("Middle", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::MouseButton::Middle))
        });

        fields.add_field_method_get("Back", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::MouseButton::Back))
        });

        fields.add_field_method_get("Forward", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::MouseButton::Forward))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Left", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Left = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Right", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Right = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Middle", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Middle = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Back", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Back = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Forward", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Forward = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Other", |lua, this| {
            let fyrox_lite::lite_event::MouseButton::Other(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                _1
            })?;

            Ok(mlua::Value::Table(t))
        });
    }
}
