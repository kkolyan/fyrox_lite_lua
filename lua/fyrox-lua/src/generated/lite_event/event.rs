
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

impl FyroxUserData for fyrox_lite::lite_event::Event {
    const CLASS_NAME: &'static str = "Event";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut(
                    "NewEvents",
                    |lua, this, mut args: mlua::MultiValue| {
            
                        let Some(_1) = args.pop_front() else {
                            return Err(lua_error!("argument 1 (StartCause) missing"));
                        };
                        let _1 = <TypedUserData<Traitor<fyrox_lite::lite_event::StartCause>> as mlua::FromLua>::from_lua(_1, lua)?;
                        let _1 = _1.borrow()?.inner().clone().into();
            
                        let value = fyrox_lite::lite_event::Event::NewEvents( _1 );
                        Ok(Traitor::new(value))
        
                    }
                );

        methods.add_method_mut("WindowEvent", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "Event::WindowEvent-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let window_id = value.get::<_, i64>("window_id")?;
            let window_id = window_id;

            let event = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::WindowEvent>>>("event")?;
            let event = event.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::Event::WindowEvent { window_id, event };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("DeviceEvent", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "Event::DeviceEvent-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let device_id = value.get::<_, i64>("device_id")?;
            let device_id = device_id;

            let event = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::DeviceEvent>>>("event")?;
            let event = event.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::Event::DeviceEvent { device_id, event };
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("UserEvent", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::UserEvent))
        });

        fields.add_field_method_get("Suspended", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::Suspended))
        });

        fields.add_field_method_get("Resumed", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::Resumed))
        });

        fields.add_field_method_get("AboutToWait", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::AboutToWait))
        });

        fields.add_field_method_get("LoopExiting", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::LoopExiting))
        });

        fields.add_field_method_get("MemoryWarning", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::Event::MemoryWarning))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("NewEvents", |lua, this| {
            let fyrox_lite::lite_event::Event::NewEvents(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_event::StartCause::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("WindowEvent", |lua, this| {
            let fyrox_lite::lite_event::Event::WindowEvent { window_id, event } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("window_id", {
                let window_id = window_id.clone();
                window_id
            })?;

            t.set("event", {
                let event = event.clone();
                Traitor::new(fyrox_lite::lite_event::WindowEvent::from(event))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("DeviceEvent", |lua, this| {
            let fyrox_lite::lite_event::Event::DeviceEvent { device_id, event } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("device_id", {
                let device_id = device_id.clone();
                device_id
            })?;

            t.set("event", {
                let event = event.clone();
                Traitor::new(fyrox_lite::lite_event::DeviceEvent::from(event))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("UserEvent", |lua, this| {
            let fyrox_lite::lite_event::Event::UserEvent = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Suspended", |lua, this| {
            let fyrox_lite::lite_event::Event::Suspended = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Resumed", |lua, this| {
            let fyrox_lite::lite_event::Event::Resumed = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AboutToWait", |lua, this| {
            let fyrox_lite::lite_event::Event::AboutToWait = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("LoopExiting", |lua, this| {
            let fyrox_lite::lite_event::Event::LoopExiting = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MemoryWarning", |lua, this| {
            let fyrox_lite::lite_event::Event::MemoryWarning = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
