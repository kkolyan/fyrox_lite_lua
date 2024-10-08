
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

impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_ui::TextBuilder> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;

            t.set("foregound", {
                let foregound = self.foregound.clone();
                if let Some(foregound) = foregound {
                    Some(Traitor::new(fyrox_lite::lite_ui::Brush::from(foregound)))
                } else {
                    None
                }
            })?;

            t.set("font_size", {
                let font_size = self.font_size.clone();
                if let Some(font_size) = font_size {
                    Some(font_size)
                } else {
                    None
                }
            })?;

            t
        }))
    }
}

impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_ui::TextBuilder> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract TextBuilder from {:?}. expected table.",
                value
            ));
        };

        let foregound = value
            .get::<_, Option<TypedUserData<Traitor<fyrox_lite::lite_ui::Brush>>>>("foregound")?;
        let foregound = if let Some(foregound) = foregound {
            Some(foregound.borrow()?.inner().clone().into())
        } else {
            None
        };

        let font_size = value.get::<_, Option<f32>>("font_size")?;
        let font_size = if let Some(font_size) = font_size {
            Some(font_size)
        } else {
            None
        };

        Ok(Traitor::new(fyrox_lite::lite_ui::TextBuilder {
            foregound,
            font_size,
        }))
    }
}
