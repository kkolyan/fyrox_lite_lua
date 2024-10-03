

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
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
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_ui::TextBuilder> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract TextBuilder from {:?}. expected table.", value));
                };
    
                let foregound = value.get::<_, Option<Traitor<lite_ui::Brush>>>("foregound")?;
                let foregound = if let Some(foregound) = foregound { Some(foregound.inner().clone().into()) } else { None };
        
                let font_size = value.get::<_, Option<f32>>("font_size")?;
                let font_size = if let Some(font_size) = font_size { Some(font_size) } else { None };
        
                Ok(Traitor::new(lite_ui::TextBuilder { foregound, font_size }))
            }
        }
    