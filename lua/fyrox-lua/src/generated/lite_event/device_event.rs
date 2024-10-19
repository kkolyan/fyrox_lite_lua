
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

impl FyroxUserData for fyrox_lite::lite_event::DeviceEvent {
    const CLASS_NAME: &'static str = "DeviceEvent";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("MouseMotion", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "DeviceEvent::MouseMotion-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let delta = value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2>>("delta")?;
            let delta = delta.inner().clone().into();

            let value = fyrox_lite::lite_event::DeviceEvent::MouseMotion { delta };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("MouseWheel", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "DeviceEvent::MouseWheel-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let delta = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::MouseScrollDelta>>>(
                    "delta",
                )?;
            let delta = delta.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::DeviceEvent::MouseWheel { delta };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Motion", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "DeviceEvent::Motion-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let axis = value.get::<_, i32>("axis")?;
            let axis = axis;

            let value = value.get::<_, f64>("value")?;
            let value = value;

            let value = fyrox_lite::lite_event::DeviceEvent::Motion { axis, value };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Button", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "DeviceEvent::Button-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let button = value.get::<_, i32>("button")?;
            let button = button;

            let state = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::ElementState>>>("state")?;
            let state = state.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::DeviceEvent::Button { button, state };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Key", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (RawKeyEvent) missing"));
            };
            let _1 =
                <Traitor<fyrox_lite::lite_event::RawKeyEvent> as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.inner().clone().into();

            let value = fyrox_lite::lite_event::DeviceEvent::Key(_1);
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Added", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::DeviceEvent::Added))
        });

        fields.add_field_method_get("Removed", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::DeviceEvent::Removed))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Added", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::Added = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Removed", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::Removed = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MouseMotion", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::MouseMotion { delta } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("delta", {
                let delta = delta.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2::from(delta))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("MouseWheel", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::MouseWheel { delta } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("delta", {
                let delta = delta.clone();
                Traitor::new(fyrox_lite::lite_event::MouseScrollDelta::from(delta))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Motion", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::Motion { axis, value } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("axis", {
                let axis = axis.clone();
                axis
            })?;

            t.set("value", {
                let value = value.clone();
                value
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Button", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::Button { button, state } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("button", {
                let button = button.clone();
                button
            })?;

            t.set("state", {
                let state = state.clone();
                Traitor::new(fyrox_lite::lite_event::ElementState::from(state))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Key", |lua, this| {
            let fyrox_lite::lite_event::DeviceEvent::Key(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_event::RawKeyEvent::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });
    }
}
