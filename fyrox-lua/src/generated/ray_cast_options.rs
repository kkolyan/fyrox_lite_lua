

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
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
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_physics::LiteRayCastOptions> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract RayCastOptions from {:?}. expected table.", value));
                };
    
                let ray_origin = value.get::<_, TypedUserData<Traitor<vec::LiteVector3>>>("ray_origin")?;
                let ray_origin = ray_origin.borrow()?.inner().clone().into();
        
                let ray_direction = value.get::<_, TypedUserData<Traitor<vec::LiteVector3>>>("ray_direction")?;
                let ray_direction = ray_direction.borrow()?.inner().clone().into();
        
                let max_len = value.get::<_, f32>("max_len")?;
                let max_len = max_len;
        
                let groups = value.get::<_, Traitor<lite_physics::LiteInteractionGroups>>("groups")?;
                let groups = groups.inner().clone().into();
        
                let sort_results = value.get::<_, bool>("sort_results")?;
                let sort_results = sort_results;
        
                Ok(Traitor::new(lite_physics::LiteRayCastOptions { ray_origin, ray_direction, max_len, groups, sort_results }))
            }
        }
    