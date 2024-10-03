

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
        };
    
        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_math::PodVector2> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("x", {
                        let x = self.x.clone();
                        x
                    })?;
        
                    t.set("y", {
                        let y = self.y.clone();
                        y
                    })?;
        
                    t
                }))
            }
        }
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_math::PodVector2> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract Vector2 from {:?}. expected table.", value));
                };
    
                let x = value.get::<_, f32>("x")?;
                let x = x;
        
                let y = value.get::<_, f32>("y")?;
                let y = y;
        
                Ok(Traitor::new(lite_math::PodVector2 { x, y }))
            }
        }
    