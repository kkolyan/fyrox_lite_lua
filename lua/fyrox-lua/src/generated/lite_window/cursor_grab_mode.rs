
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

impl FyroxUserData for fyrox_lite::lite_window::LiteCursorGrabMode {
    const CLASS_NAME: &'static str = "CursorGrabMode";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("None", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_window::LiteCursorGrabMode::None,
            ))
        });

        fields.add_field_method_get("Confined", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_window::LiteCursorGrabMode::Confined,
            ))
        });

        fields.add_field_method_get("Locked", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_window::LiteCursorGrabMode::Locked,
            ))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("None", |lua, this| {
            let fyrox_lite::lite_window::LiteCursorGrabMode::None = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Confined", |lua, this| {
            let fyrox_lite::lite_window::LiteCursorGrabMode::Confined = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Locked", |lua, this| {
            let fyrox_lite::lite_window::LiteCursorGrabMode::Locked = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
