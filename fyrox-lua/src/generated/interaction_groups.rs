

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
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
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_physics::LiteInteractionGroups> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract InteractionGroups from {:?}. expected table.", value));
                };
    
                let memberships = value.get::<_, i32>("memberships")?;
                let memberships = memberships;
        
                let filter = value.get::<_, i32>("filter")?;
                let filter = filter;
        
                Ok(Traitor::new(lite_physics::LiteInteractionGroups { memberships, filter }))
            }
        }
    