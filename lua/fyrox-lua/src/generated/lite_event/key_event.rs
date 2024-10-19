
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_event::KeyEvent> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("physical_key", {
                let physical_key = self.physical_key.clone();
                Traitor::new(fyrox_lite::lite_event::PhysicalKey::from(physical_key))
            })?;

            t.set("state", {
                let state = self.state.clone();
                Traitor::new(fyrox_lite::lite_event::ElementState::from(state))
            })?;

            t.set("repeat", {
                let repeat = self.repeat.clone();
                repeat
            })?;

            t
        }))
    }
}

impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_event::KeyEvent> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract KeyEvent from {:?}. expected table.",
                value
            ));
        };

        let physical_key = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::PhysicalKey>>>(
                "physical_key",
            )?;
        let physical_key = physical_key.borrow()?.inner().clone().into();

        let state = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::ElementState>>>("state")?;
        let state = state.borrow()?.inner().clone().into();

        let repeat = value.get::<_, bool>("repeat")?;
        let repeat = repeat;

        Ok(Traitor::new(fyrox_lite::lite_event::KeyEvent {
            physical_key,
            state,
            repeat,
        }))
    }
}
