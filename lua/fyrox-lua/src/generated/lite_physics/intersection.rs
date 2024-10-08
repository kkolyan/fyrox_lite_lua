
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteIntersection> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("collider", {
                let collider = self.collider.clone();
                Traitor::new(fyrox_lite::lite_node::LiteNode::from(collider))
            })?;

            t.set("normal", {
                let normal = self.normal.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(normal))
            })?;

            t.set("position", {
                let position = self.position.clone();
                Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(position))
            })?;

            t.set("feature", {
                let feature = self.feature.clone();
                Traitor::new(fyrox_lite::lite_physics::LiteFeatureId::from(feature))
            })?;

            t.set("toi", {
                let toi = self.toi.clone();
                toi
            })?;

            t
        }))
    }
}

impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_physics::LiteIntersection> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract Intersection from {:?}. expected table.",
                value
            ));
        };

        let collider =
            value.get::<_, TypedUserData<Traitor<fyrox_lite::lite_node::LiteNode>>>("collider")?;
        let collider = collider.borrow()?.inner().clone().into();

        let normal = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>>("normal")?;
        let normal = normal.borrow()?.inner().clone().into();

        let position = value
            .get::<_, TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>>(
                "position",
            )?;
        let position = position.borrow()?.inner().clone().into();

        let feature = value
            .get::<_, TypedUserData<Traitor<fyrox_lite::lite_physics::LiteFeatureId>>>("feature")?;
        let feature = feature.borrow()?.inner().clone().into();

        let toi = value.get::<_, f32>("toi")?;
        let toi = toi;

        Ok(Traitor::new(fyrox_lite::lite_physics::LiteIntersection {
            collider,
            normal,
            position,
            feature,
            toi,
        }))
    }
}
