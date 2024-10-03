

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
        };
    
        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_ui::GradientPoint> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("stop", {
                        let stop = self.stop.clone();
                        stop
                    })?;
        
                    t.set("color", {
                        let color = self.color.clone();
                        Traitor::new(lite_ui::Color::from(color))
                    })?;
        
                    t
                }))
            }
        }
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_ui::GradientPoint> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract GradientPoint from {:?}. expected table.", value));
                };
    
                let stop = value.get::<_, f32>("stop")?;
                let stop = stop;
        
                let color = value.get::<_, Traitor<lite_ui::Color>>("color")?;
                let color = color.inner().clone().into();
        
                Ok(Traitor::new(lite_ui::GradientPoint { stop, color }))
            }
        }
    