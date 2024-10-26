
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
impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteRayCastOptions> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;
            t.set("ray_origin", {
                let ray_origin = self.ray_origin.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(ray_origin))
            })?;
            t.set("ray_direction", {
                let ray_direction = self.ray_direction.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(ray_direction))
            })?;
            t.set("max_len", {
                let max_len = self.max_len.clone();
                max_len
            })?;
            t.set("groups", {
                let groups = self.groups.clone();
                if let Some(groups) = groups {
                    Some(Traitor::new(
                        fyrox_lite::lite_physics::LiteInteractionGroups::from(groups),
                    ))
                } else {
                    None
                }
            })?;
            t.set("sort_results", {
                let sort_results = self.sort_results.clone();
                sort_results
            })?;
            t
        }))
    }
}
impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteRayCastOptions> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract RayCastOptions from {:?}. expected table.",
                value
            ));
        };
        let ray_origin = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>>(
                "ray_origin",
            )?;
        let ray_origin = ray_origin.borrow()?.inner().clone().into();
        let ray_direction = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>>(
                "ray_direction",
            )?;
        let ray_direction = ray_direction.borrow()?.inner().clone().into();
        let max_len = value.get::<_, f32>("max_len")?;
        let max_len = max_len;
        let groups = value
            .get::<_, Option<Traitor<fyrox_lite::lite_physics::LiteInteractionGroups>>>("groups")?;
        let groups = if let Some(groups) = groups {
            Some(groups.inner().clone().into())
        } else {
            None
        };
        let sort_results = value.get::<_, bool>("sort_results")?;
        let sort_results = sort_results;
        Ok(Traitor::new(fyrox_lite::lite_physics::LiteRayCastOptions {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }))
    }
}
