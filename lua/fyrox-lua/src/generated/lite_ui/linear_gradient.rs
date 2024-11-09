
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
impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_ui::LinearGradient> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;
            t.set("from", {
                let from = self.from.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(from))
            })?;
            t.set("to", {
                let to = self.to.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(to))
            })?;
            t.set("stops", {
                let stops = self.stops.clone();
                lua.create_table_from(
                    stops
                        .into_iter()
                        .map(|it| Traitor::new(fyrox_lite::lite_ui::GradientPoint::from(it)))
                        .enumerate()
                        .map(|(i, v)| (i + 1, v)),
                )?
            })?;
            t
        }))
    }
}
impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_ui::LinearGradient> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract LinearGradient from {:?}. expected table.",
                value
            ));
        };
        let from = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2>>>("from")?;
        let from = from.borrow()?.inner().clone().into();
        let to = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2>>>("to")?;
        let to = to.borrow()?.inner().clone().into();
        let stops = value.get::<_, Vec<Traitor<fyrox_lite::lite_ui::GradientPoint>>>("stops")?;
        let stops = stops
            .into_iter()
            .map(|it| it.inner().clone().into())
            .collect::<Vec<_>>();
        Ok(Traitor::new(fyrox_lite::lite_ui::LinearGradient {
            from,
            to,
            stops,
        }))
    }
}
