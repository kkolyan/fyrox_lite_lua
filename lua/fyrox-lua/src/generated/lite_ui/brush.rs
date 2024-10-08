
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

impl FyroxUserData for fyrox_lite::lite_ui::Brush {
    const CLASS_NAME: &'static str = "Brush";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Solid", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Color) missing"));
            };
            let _1 =
                <TypedUserData<Traitor<fyrox_lite::lite_ui::Color>> as mlua::FromLua>::from_lua(
                    _1, lua,
                )?;
            let _1 = _1.borrow()?.inner().clone().into();

            let value = fyrox_lite::lite_ui::Brush::Solid(_1);
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("LinearGradient", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "Brush::LinearGradient-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let from = value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2>>("from")?;
            let from = from.inner().clone().into();

            let to = value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2>>("to")?;
            let to = to.inner().clone().into();

            let stops =
                value.get::<_, Vec<Traitor<fyrox_lite::lite_ui::GradientPoint>>>("stops")?;
            let stops = stops
                .into_iter()
                .map(|it| it.inner().clone().into())
                .collect::<Vec<_>>();

            let value = fyrox_lite::lite_ui::Brush::LinearGradient { from, to, stops };
            Ok(Traitor::new(value))
        });

        methods.add_method_mut("RadialGradient", |lua, this, mut args: mlua::MultiValue| {
            if args.len() != 1 {
                return Err(lua_error!(
                    "exactly one argument expected, but {} supplied",
                    args.len()
                ));
            }
            let value = args.pop_front().expect("WTF, just checket the size");
            let mlua::Value::Table(value) = value else {
                return Err(lua_error!(
                    "Brush::RadialGradient-typed table expected, but {:?} supplied",
                    value
                ));
            };

            let center = value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2>>("center")?;
            let center = center.inner().clone().into();

            let stops =
                value.get::<_, Vec<Traitor<fyrox_lite::lite_ui::GradientPoint>>>("stops")?;
            let stops = stops
                .into_iter()
                .map(|it| it.inner().clone().into())
                .collect::<Vec<_>>();

            let value = fyrox_lite::lite_ui::Brush::RadialGradient { center, stops };
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
        fields.add_field_method_get("Solid", |lua, this| {
            let fyrox_lite::lite_ui::Brush::Solid(_1) = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            // Lua annotations is based on assumption that indexed table is homogenous array, so use string keys to allow heterogenous typing here.
            t.set("_1", {
                let _1 = _1.clone();
                Traitor::new(fyrox_lite::lite_ui::Color::from(_1))
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("LinearGradient", |lua, this| {
            let fyrox_lite::lite_ui::Brush::LinearGradient { from, to, stops } = this.inner()
            else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("from", {
                let from = from.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2::from(from))
            })?;

            t.set("to", {
                let to = to.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2::from(to))
            })?;

            t.set("stops", {
                let stops = stops.clone();
                lua.create_table_from(
                    stops
                        .into_iter()
                        .map(|it| Traitor::new(fyrox_lite::lite_ui::GradientPoint::from(it)))
                        .enumerate()
                        .map(|(i, v)| (i + 1, v)),
                )?
            })?;

            Ok(mlua::Value::Table(t))
        });

        fields.add_field_method_get("RadialGradient", |lua, this| {
            let fyrox_lite::lite_ui::Brush::RadialGradient { center, stops } = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            let t = lua.create_table()?;

            t.set("center", {
                let center = center.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2::from(center))
            })?;

            t.set("stops", {
                let stops = stops.clone();
                lua.create_table_from(
                    stops
                        .into_iter()
                        .map(|it| Traitor::new(fyrox_lite::lite_ui::GradientPoint::from(it)))
                        .enumerate()
                        .map(|(i, v)| (i + 1, v)),
                )?
            })?;

            Ok(mlua::Value::Table(t))
        });
    }
}
