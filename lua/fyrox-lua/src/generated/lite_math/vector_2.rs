
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_math::PodVector2> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("x", {
                let x = self.x.clone();
                x
            })?;

            t.set("y", {
                let y = self.y.clone();
                y
            })?;

            t
        }))
    }
}

impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_math::PodVector2> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract Vector2 from {:?}. expected table.",
                value
            ));
        };

        let x = value.get::<_, f32>("x")?;
        let x = x;

        let y = value.get::<_, f32>("y")?;
        let y = y;

        Ok(Traitor::new(fyrox_lite::lite_math::PodVector2 { x, y }))
    }
}
