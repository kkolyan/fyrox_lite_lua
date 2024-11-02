
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
impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_ui::Brush> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;
            t.set("solid_color", {
                let solid_color = self.solid_color.clone();
                if let Some(solid_color) = solid_color {
                    Some(Traitor::new(fyrox_lite_color::lite_color::Color::from(
                        solid_color,
                    )))
                } else {
                    None
                }
            })?;
            t.set("linear_gradient", {
                let linear_gradient = self.linear_gradient.clone();
                if let Some(linear_gradient) = linear_gradient {
                    Some(Traitor::new(fyrox_lite::lite_ui::LinearGradient::from(
                        linear_gradient,
                    )))
                } else {
                    None
                }
            })?;
            t.set("radial_gradient", {
                let radial_gradient = self.radial_gradient.clone();
                if let Some(radial_gradient) = radial_gradient {
                    Some(Traitor::new(fyrox_lite::lite_ui::RadialGradient::from(
                        radial_gradient,
                    )))
                } else {
                    None
                }
            })?;
            t
        }))
    }
}
impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_ui::Brush> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract Brush from {:?}. expected table.",
                value
            ));
        };
        let solid_color = value.get::<_, Option<
            TypedUserData<Traitor<fyrox_lite_color::lite_color::Color>>,
        >>("solid_color")?;
        let solid_color = if let Some(solid_color) = solid_color {
            Some(solid_color.borrow()?.inner().clone().into())
        } else {
            None
        };
        let linear_gradient = value
            .get::<_, Option<Traitor<fyrox_lite::lite_ui::LinearGradient>>>("linear_gradient")?;
        let linear_gradient = if let Some(linear_gradient) = linear_gradient {
            Some(linear_gradient.inner().clone().into())
        } else {
            None
        };
        let radial_gradient = value
            .get::<_, Option<Traitor<fyrox_lite::lite_ui::RadialGradient>>>("radial_gradient")?;
        let radial_gradient = if let Some(radial_gradient) = radial_gradient {
            Some(radial_gradient.inner().clone().into())
        } else {
            None
        };
        Ok(Traitor::new(fyrox_lite::lite_ui::Brush {
            solid_color,
            linear_gradient,
            radial_gradient,
        }))
    }
}
