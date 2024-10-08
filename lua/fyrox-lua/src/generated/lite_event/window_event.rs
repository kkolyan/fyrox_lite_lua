
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
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for fyrox_lite::lite_event::WindowEvent {
    const CLASS_NAME: &'static str = "WindowEvent";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Resized", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Vector2i) missing"));
            };
            let _1 =
                <Traitor<fyrox_lite::lite_math::PodVector2i> as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::Resized(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Moved", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Vector2i) missing"));
            };
            let _1 =
                <Traitor<fyrox_lite::lite_math::PodVector2i> as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::Moved(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("DroppedFile", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (String) missing"));
            };
            let _1 = <mlua::String as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.to_str()?.to_string();

            let value = fyrox_lite::lite_event::WindowEvent::DroppedFile(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("HoveredFile", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (String) missing"));
            };
            let _1 = <mlua::String as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.to_str()?.to_string();

            let value = fyrox_lite::lite_event::WindowEvent::HoveredFile(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Focused", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Bool) missing"));
            };
            let _1 = <bool as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_event::WindowEvent::Focused(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("KeyboardInput", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "WindowEvent::KeyboardInput-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let event = value.get::<_, Traitor<fyrox_lite::lite_event::KeyEvent>>("event")?;
            let event = event.inner().clone().into();

            let is_synthetic = value.get::<_, bool>("is_synthetic")?;
            let is_synthetic = is_synthetic;

            let value = fyrox_lite::lite_event::WindowEvent::KeyboardInput {
                event,
                is_synthetic,
            };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("CursorMoved", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "WindowEvent::CursorMoved-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let position =
                value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2i>>("position")?;
            let position = position.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::CursorMoved { position };
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
                    "WindowEvent::MouseWheel-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let delta = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::MouseScrollDelta>>>(
                    "delta",
                )?;
            let delta = delta.borrow()?.inner().clone().into();

            let phase = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::TouchPhase>>>("phase")?;
            let phase = phase.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::MouseWheel { delta, phase };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("MouseInput", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "WindowEvent::MouseInput-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let state = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::ElementState>>>("state")?;
            let state = state.borrow()?.inner().clone().into();

            let button = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::MouseButton>>>("button")?;
            let button = button.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::MouseInput { state, button };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut(
            "TouchpadMagnify",
            |lua, this, mut args: mlua::MultiValue| {
                if args.len() != 1 {
                    return Err(lua_error!(
                        "exactly one argument expected, but {} supplied",
                        args.len()
                    ));
                }
                let value = args.pop_front().expect("WTF, just checket the size");
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!(
                        "WindowEvent::TouchpadMagnify-typed table expected, but {:?} supplied",
                        value
                    ));
                };

                let delta = value.get::<_, f64>("delta")?;
                let delta = delta;

                let phase = value
                    .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::TouchPhase>>>(
                        "phase",
                    )?;
                let phase = phase.borrow()?.inner().clone().into();

                let value = fyrox_lite::lite_event::WindowEvent::TouchpadMagnify { delta, phase };
                Ok(Traitor::new(value))
            },
        );

        methods.add_method_mut("TouchpadRotate", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "WindowEvent::TouchpadRotate-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let delta = value.get::<_, f32>("delta")?;
            let delta = delta;

            let phase = value
                .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::TouchPhase>>>("phase")?;
            let phase = phase.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::TouchpadRotate { delta, phase };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut(
            "TouchpadPressure",
            |lua, this, mut args: mlua::MultiValue| {
                if args.len() != 1 {
                    return Err(lua_error!(
                        "exactly one argument expected, but {} supplied",
                        args.len()
                    ));
                }
                let value = args.pop_front().expect("WTF, just checket the size");
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!(
                        "WindowEvent::TouchpadPressure-typed table expected, but {:?} supplied",
                        value
                    ));
                };

                let pressure = value.get::<_, f32>("pressure")?;
                let pressure = pressure;

                let stage = value.get::<_, i64>("stage")?;
                let stage = stage;

                let value =
                    fyrox_lite::lite_event::WindowEvent::TouchpadPressure { pressure, stage };
                Ok(Traitor::new(value))
            },
        );

        methods.add_method_mut("AxisMotion", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "WindowEvent::AxisMotion-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let axis = value.get::<_, i32>("axis")?;
            let axis = axis;

            let value = value.get::<_, f64>("value")?;
            let value = value;

            let value = fyrox_lite::lite_event::WindowEvent::AxisMotion { axis, value };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Touch", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Touch) missing"));
            };
            let _1 = <Traitor<fyrox_lite::lite_event::Touch> as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.inner().clone().into();

            let value = fyrox_lite::lite_event::WindowEvent::Touch(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut(
            "ScaleFactorChanged",
            |lua, this, mut args: mlua::MultiValue| {
                if args.len() != 1 {
                    return Err(lua_error!(
                        "exactly one argument expected, but {} supplied",
                        args.len()
                    ));
                }
                let value = args.pop_front().expect("WTF, just checket the size");
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!(
                        "WindowEvent::ScaleFactorChanged-typed table expected, but {:?} supplied",
                        value
                    ));
                };

                let scale_factor = value.get::<_, f64>("scale_factor")?;
                let scale_factor = scale_factor;

                let inner_size_writer = value.get::<_, TypedUserData<
                    Traitor<fyrox_lite::lite_event::InnerSizeWriter>,
                >>("inner_size_writer")?;
                let inner_size_writer = inner_size_writer.borrow()?.inner().clone().into();

                let value = fyrox_lite::lite_event::WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    inner_size_writer,
                };
                Ok(Traitor::new(value))
            },
        );

        methods.add_method_mut("Occluded", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Bool) missing"));
            };
            let _1 = <bool as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_event::WindowEvent::Occluded(_1);
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("ActivationTokenDone", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::ActivationTokenDone,
            ))
        });

        fields.add_field_method_get("CloseRequested", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::CloseRequested,
            ))
        });

        fields.add_field_method_get("Destroyed", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::WindowEvent::Destroyed))
        });

        fields.add_field_method_get("HoveredFileCancelled", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::HoveredFileCancelled,
            ))
        });

        fields.add_field_method_get("ModifiersChanged", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::ModifiersChanged,
            ))
        });

        fields.add_field_method_get("Ime", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::WindowEvent::Ime))
        });

        fields.add_field_method_get("CursorEntered", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::CursorEntered,
            ))
        });

        fields.add_field_method_get("CursorLeft", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::CursorLeft,
            ))
        });

        fields.add_field_method_get("SmartMagnify", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::SmartMagnify,
            ))
        });

        fields.add_field_method_get("ThemeChanged", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::ThemeChanged,
            ))
        });

        fields.add_field_method_get("RedrawRequested", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::WindowEvent::RedrawRequested,
            ))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("ActivationTokenDone", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::ActivationTokenDone = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Resized", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Resized(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2i::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Moved", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Moved(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2i::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("CloseRequested", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::CloseRequested = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Destroyed", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Destroyed = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("DroppedFile", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::DroppedFile(_1) = this.inner() else {
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

        fields.add_field_method_get("HoveredFile", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::HoveredFile(_1) = this.inner() else {
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

        fields.add_field_method_get("HoveredFileCancelled", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::HoveredFileCancelled = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Focused", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Focused(_1) = this.inner() else {
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

        fields.add_field_method_get("KeyboardInput", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::KeyboardInput {
                event,
                is_synthetic,
            } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("event", {
                let event = event.clone();
                Traitor::new(fyrox_lite::lite_event::KeyEvent::from(event))
            })?;

            t.set("is_synthetic", {
                let is_synthetic = is_synthetic.clone();
                is_synthetic
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("ModifiersChanged", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::ModifiersChanged = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Ime", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Ime = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("CursorMoved", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::CursorMoved { position } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("position", {
                let position = position.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2i::from(position))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("CursorEntered", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::CursorEntered = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("CursorLeft", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::CursorLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MouseWheel", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::MouseWheel { delta, phase } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("delta", {
                let delta = delta.clone();
                Traitor::new(fyrox_lite::lite_event::MouseScrollDelta::from(delta))
            })?;

            t.set("phase", {
                let phase = phase.clone();
                Traitor::new(fyrox_lite::lite_event::TouchPhase::from(phase))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("MouseInput", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::MouseInput { state, button } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("state", {
                let state = state.clone();
                Traitor::new(fyrox_lite::lite_event::ElementState::from(state))
            })?;

            t.set("button", {
                let button = button.clone();
                Traitor::new(fyrox_lite::lite_event::MouseButton::from(button))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("TouchpadMagnify", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::TouchpadMagnify { delta, phase } =
                this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("delta", {
                let delta = delta.clone();
                delta
            })?;

            t.set("phase", {
                let phase = phase.clone();
                Traitor::new(fyrox_lite::lite_event::TouchPhase::from(phase))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("SmartMagnify", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::SmartMagnify = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("TouchpadRotate", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::TouchpadRotate { delta, phase } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("delta", {
                let delta = delta.clone();
                delta
            })?;

            t.set("phase", {
                let phase = phase.clone();
                Traitor::new(fyrox_lite::lite_event::TouchPhase::from(phase))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("TouchpadPressure", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::TouchpadPressure { pressure, stage } =
                this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("pressure", {
                let pressure = pressure.clone();
                pressure
            })?;

            t.set("stage", {
                let stage = stage.clone();
                stage
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("AxisMotion", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::AxisMotion { axis, value } = this.inner()
            else {
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

        fields.add_field_method_get("Touch", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Touch(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_event::Touch::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("ScaleFactorChanged", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("scale_factor", {
                let scale_factor = scale_factor.clone();
                scale_factor
            })?;

            t.set("inner_size_writer", {
                let inner_size_writer = inner_size_writer.clone();
                Traitor::new(fyrox_lite::lite_event::InnerSizeWriter::from(
                    inner_size_writer,
                ))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("ThemeChanged", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::ThemeChanged = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Occluded", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::Occluded(_1) = this.inner() else {
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

        fields.add_field_method_get("RedrawRequested", |lua, this| {
            let fyrox_lite::lite_event::WindowEvent::RedrawRequested = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
