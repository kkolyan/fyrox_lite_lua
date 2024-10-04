
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

impl FyroxUserData for fyrox_lite::lite_physics::LiteFeatureId {
    const CLASS_NAME: &'static str = "FeatureId";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Vertex", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (i32) missing"));
            };
            let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_physics::LiteFeatureId::Vertex(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Edge", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (i32) missing"));
            };
            let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_physics::LiteFeatureId::Edge(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Face", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (i32) missing"));
            };
            let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_physics::LiteFeatureId::Face(_1);
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Unknown", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_physics::LiteFeatureId::Unknown,
            ))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Vertex", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureId::Vertex(_1) = this.inner() else {
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

        fields.add_field_method_get("Edge", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureId::Edge(_1) = this.inner() else {
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

        fields.add_field_method_get("Face", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureId::Face(_1) = this.inner() else {
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

        fields.add_field_method_get("Unknown", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureId::Unknown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
