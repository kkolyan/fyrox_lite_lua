
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
impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteFeatureId> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;
            t.set("kind", {
                let kind = self.kind.clone();
                Traitor::new(fyrox_lite::lite_physics::LiteFeatureKind::from(kind))
            })?;
            t.set("id", {
                let id = self.id.clone();
                id
            })?;
            t
        }))
    }
}
impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteFeatureId> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract FeatureId from {:?}. expected table.",
                value
            ));
        };
        let kind = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_physics::LiteFeatureKind>>>("kind")?;
        let kind = kind.borrow()?.inner().clone().into();
        let id = value.get::<_, i32>("id")?;
        let id = id;
        Ok(Traitor::new(fyrox_lite::lite_physics::LiteFeatureId {
            kind,
            id,
        }))
    }
}
