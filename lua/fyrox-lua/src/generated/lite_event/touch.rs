
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_event::Touch> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("phase", {
                let phase = self.phase.clone();
                Traitor::new(fyrox_lite::lite_event::TouchPhase::from(phase))
            })?;

            t.set("location", {
                let location = self.location.clone();
                Traitor::new(fyrox_lite::lite_math::PodVector2::from(location))
            })?;

            t.set("force", {
                let force = self.force.clone();
                if let Some(force) = force {
                    Some(Traitor::new(fyrox_lite::lite_event::Force::from(force)))
                } else {
                    None
                }
            })?;

            t.set("id", {
                let id = self.id.clone();
                id
            })?;

            t
        }))
    }
}

impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_event::Touch> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract Touch from {:?}. expected table.",
                value
            ));
        };

        let phase =
            value.get::<_, TypedUserData<Traitor<fyrox_lite::lite_event::TouchPhase>>>("phase")?;
        let phase = phase.borrow()?.inner().clone().into();

        let location = value.get::<_, Traitor<fyrox_lite::lite_math::PodVector2>>("location")?;
        let location = location.inner().clone().into();

        let force = value
            .get::<_, Option<TypedUserData<Traitor<fyrox_lite::lite_event::Force>>>>("force")?;
        let force = if let Some(force) = force {
            Some(force.borrow()?.inner().clone().into())
        } else {
            None
        };

        let id = value.get::<_, i64>("id")?;
        let id = id;

        Ok(Traitor::new(fyrox_lite::lite_event::Touch {
            phase,
            location,
            force,
            id,
        }))
    }
}
