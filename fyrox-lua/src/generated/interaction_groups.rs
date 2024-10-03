

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_physics::LiteInteractionGroups> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("memberships", {
                        let memberships = self.memberships;
                        memberships
                    })?;
        
                    t.set("filter", {
                        let filter = self.filter;
                        filter
                    })?;
        
                    t
                }))
            }
        }
    