
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

impl FyroxUserData for fyrox_lite::lite_event::Force {
    const CLASS_NAME: &'static str = "Force";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Calibrated", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "Force::Calibrated-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let force = value.get::<_, f64>("force")?;
            let force = force;

            let max_possible_force = value.get::<_, f64>("max_possible_force")?;
            let max_possible_force = max_possible_force;

            let altitude_angle = value.get::<_, Option<f64>>("altitude_angle")?;
            let altitude_angle = if let Some(altitude_angle) = altitude_angle {
                Some(altitude_angle)
            } else {
                None
            };

            let value = fyrox_lite::lite_event::Force::Calibrated {
                force,
                max_possible_force,
                altitude_angle,
            };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("Normalized", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (f64) missing"));
            };
            let _1 = <f64 as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1;

            let value = fyrox_lite::lite_event::Force::Normalized(_1);
            Ok(Traitor::new(value))
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Calibrated", |lua, this| {
            let fyrox_lite::lite_event::Force::Calibrated {
                force,
                max_possible_force,
                altitude_angle,
            } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("force", {
                let force = force.clone();
                force
            })?;

            t.set("max_possible_force", {
                let max_possible_force = max_possible_force.clone();
                max_possible_force
            })?;

            t.set("altitude_angle", {
                let altitude_angle = altitude_angle.clone();
                if let Some(altitude_angle) = altitude_angle {
                    Some(altitude_angle)
                } else {
                    None
                }
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("Normalized", |lua, this| {
            let fyrox_lite::lite_event::Force::Normalized(_1) = this.inner() else {
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
