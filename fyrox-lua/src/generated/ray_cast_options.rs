

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_physics::LiteRayCastOptions> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("ray_origin", {
                        let ray_origin = self.ray_origin;
                        Traitor::new(vec::LiteVector3::from(ray_origin))
                    })?;
        
                    t.set("ray_direction", {
                        let ray_direction = self.ray_direction;
                        Traitor::new(vec::LiteVector3::from(ray_direction))
                    })?;
        
                    t.set("max_len", {
                        let max_len = self.max_len;
                        max_len
                    })?;
        
                    t.set("groups", {
                        let groups = self.groups;
                        Traitor::new(lite_physics::LiteInteractionGroups::from(groups))
                    })?;
        
                    t.set("sort_results", {
                        let sort_results = self.sort_results;
                        sort_results
                    })?;
        
                    t
                }))
            }
        }
    