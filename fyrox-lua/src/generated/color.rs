

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
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
    