
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_event::KeyEvent> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("physical_key", {
                let physical_key = self.physical_key.clone();
                Traitor::new(fyrox_lite::lite_event::PhysicalKey::from(physical_key))
            })?;

            t.set("text", {
                let text = self.text.clone();
                if let Some(text) = text {
                    Some(text)
                } else {
                    None
                }
            })?;

            t.set("location", {
                let location = self.location.clone();
                Traitor::new(fyrox_lite::lite_event::KeyLocation::from(location))
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

        let text = value.get::<_, Option<mlua::String>>("text")?;
        let text = if let Some(text) = text {
            Some(text.to_str()?.to_string())
        } else {
            None
        };

        let location = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::KeyLocation>>>("location")?;
        let location = location.borrow()?.inner().clone().into();

        let state = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::ElementState>>>("state")?;
        let state = state.borrow()?.inner().clone().into();

        let repeat = value.get::<_, bool>("repeat")?;
        let repeat = repeat;

        Ok(Traitor::new(fyrox_lite::lite_event::KeyEvent {
            physical_key,
            text,
            location,
            state,
            repeat,
        }))
    }
}
