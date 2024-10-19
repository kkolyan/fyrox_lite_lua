
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
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite::lite_event::KeyLocation {
    const CLASS_NAME: &'static str = "KeyLocation";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Standard", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyLocation::Standard))
        });

        fields.add_field_method_get("Left", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyLocation::Left))
        });

        fields.add_field_method_get("Right", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyLocation::Right))
        });

        fields.add_field_method_get("Numpad", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyLocation::Numpad))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Standard", |lua, this| {
            let fyrox_lite::lite_event::KeyLocation::Standard = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Left", |lua, this| {
            let fyrox_lite::lite_event::KeyLocation::Left = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Right", |lua, this| {
            let fyrox_lite::lite_event::KeyLocation::Right = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad", |lua, this| {
            let fyrox_lite::lite_event::KeyLocation::Numpad = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
