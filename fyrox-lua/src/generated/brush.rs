
#![allow(unused_variables)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for lite_ui::Brush {
    const CLASS_NAME: &'static str = "Brush";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("Solid", |lua, this, mut args: mlua::MultiValue| {
            let Some(_1) = args.pop_front() else {
                return Err(lua_error!("argument 1 (Color) missing"));
            };
            let _1 = <Traitor<lite_ui::Color> as mlua::FromLua>::from_lua(_1, lua)?;
            let _1 = _1.inner().clone().into();

            let value = lite_ui::Brush::Solid(_1);
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

            let from = value.get::<_, Traitor<lite_math::PodVector2>>("from")?;
            let from = from.inner().clone().into();

            let to = value.get::<_, Traitor<lite_math::PodVector2>>("to")?;
            let to = to.inner().clone().into();

            let stops = value.get::<_, Vec<Traitor<lite_ui::GradientPoint>>>("stops")?;
            let stops = stops
                .into_iter()
                .map(|it| it.inner().clone().into())
                .collect::<Vec<_>>();

            let value = lite_ui::Brush::LinearGradient { from, to, stops };
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

            let center = value.get::<_, Traitor<lite_math::PodVector2>>("center")?;
            let center = center.inner().clone().into();

            let stops = value.get::<_, Vec<Traitor<lite_ui::GradientPoint>>>("stops")?;
            let stops = stops
                .into_iter()
                .map(|it| it.inner().clone().into())
                .collect::<Vec<_>>();

            let value = lite_ui::Brush::RadialGradient { center, stops };
            Ok(Traitor::new(value))
        });
    }
}
