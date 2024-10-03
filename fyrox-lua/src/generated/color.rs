

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
        };
    
        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_ui::Color> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("r", {
                        let r = self.r;
                        r
                    })?;
        
                    t.set("g", {
                        let g = self.g;
                        g
                    })?;
        
                    t.set("b", {
                        let b = self.b;
                        b
                    })?;
        
                    t.set("a", {
                        let a = self.a;
                        a
                    })?;
        
                    t
                }))
            }
        }
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_ui::Color> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract Color from {:?}. expected table.", value));
                };
    
                let r = value.get::<_, u8>("r")?;
                let r = r;
        
                let g = value.get::<_, u8>("g")?;
                let g = g;
        
                let b = value.get::<_, u8>("b")?;
                let b = b;
        
                let a = value.get::<_, u8>("a")?;
                let a = a;
        
                Ok(Traitor::new(lite_ui::Color { r, g, b, a }))
            }
        }
    