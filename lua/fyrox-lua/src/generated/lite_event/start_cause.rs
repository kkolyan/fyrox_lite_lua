
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

impl FyroxUserData for fyrox_lite::lite_event::StartCause {
    const CLASS_NAME: &'static str = "StartCause";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut(
            "ResumeTimeReached",
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
                        "StartCause::ResumeTimeReached-typed table expected, but {:?} supplied",
                        value
                    ));
                };

                let start = value.get::<_, i64>("start")?;
                let start = start;

                let requested_resume = value.get::<_, i64>("requested_resume")?;
                let requested_resume = requested_resume;

                let value = fyrox_lite::lite_event::StartCause::ResumeTimeReached {
                    start,
                    requested_resume,
                };
                Ok(Traitor::new(value))
            },
        );

        methods.add_method_mut("WaitCancelled", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "StartCause::WaitCancelled-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let start = value.get::<_, i64>("start")?;
            let start = start;

            let requested_resume = value.get::<_, Option<i64>>("requested_resume")?;
            let requested_resume = if let Some(requested_resume) = requested_resume {
                Some(requested_resume)
            } else {
                None
            };

            let value = fyrox_lite::lite_event::StartCause::WaitCancelled {
                start,
                requested_resume,
            };
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Poll", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::StartCause::Poll))
        });

        fields.add_field_method_get("Init", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::StartCause::Init))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("ResumeTimeReached", |lua, this| {
            let fyrox_lite::lite_event::StartCause::ResumeTimeReached {
                start,
                requested_resume,
            } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("start", {
                let start = start.clone();
                start
            })?;

            t.set("requested_resume", {
                let requested_resume = requested_resume.clone();
                requested_resume
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("WaitCancelled", |lua, this| {
            let fyrox_lite::lite_event::StartCause::WaitCancelled {
                start,
                requested_resume,
            } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("start", {
                let start = start.clone();
                start
            })?;

            t.set("requested_resume", {
                let requested_resume = requested_resume.clone();
                if let Some(requested_resume) = requested_resume {
                    Some(requested_resume)
                } else {
                    None
                }
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Poll", |lua, this| {
            let fyrox_lite::lite_event::StartCause::Poll = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Init", |lua, this| {
            let fyrox_lite::lite_event::StartCause::Init = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
