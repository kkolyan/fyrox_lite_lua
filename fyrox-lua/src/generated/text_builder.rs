

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_ui::TextBuilder> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("foregound", {
                        let foregound = self.foregound;
                        if let Some(foregound) = foregound { Some(Traitor::new(lite_ui::Brush::from(foregound))) } else { None }
                    })?;
        
                    t.set("font_size", {
                        let font_size = self.font_size;
                        if let Some(font_size) = font_size { Some(font_size) } else { None }
                    })?;
        
                    t
                }))
            }
        }
    