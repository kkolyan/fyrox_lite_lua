

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass}, lua_error, script_object::ScriptObject, typed_userdata::TypedUserData
        };
    
        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_physics::LiteIntersection> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("collider", {
                        let collider = self.collider.clone();
                        Traitor::new(lite_node::LiteNode::from(collider))
                    })?;
        
                    t.set("normal", {
                        let normal = self.normal.clone();
                        Traitor::new(vec::LiteVector3::from(normal))
                    })?;
        
                    t.set("position", {
                        let position = self.position.clone();
                        Traitor::new(vec::LiteVector3::from(position))
                    })?;
        
                    t.set("feature", {
                        let feature = self.feature.clone();
                        Traitor::new(lite_physics::LiteFeatureId::from(feature))
                    })?;
        
                    t.set("toi", {
                        let toi = self.toi.clone();
                        toi
                    })?;
        
                    t
                }))
            }
        }
    
        impl <'lua> mlua::FromLua<'lua> for Traitor<lite_physics::LiteIntersection> {

            fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
                let mlua::Value::Table(value) = value else {
                    return Err(lua_error!("cannot extract Intersection from {:?}. expected table.", value));
                };
    
                let collider = value.get::<_, TypedUserData<Traitor<lite_node::LiteNode>>>("collider")?;
                let collider = collider.borrow()?.inner().clone().into();
        
                let normal = value.get::<_, TypedUserData<Traitor<vec::LiteVector3>>>("normal")?;
                let normal = normal.borrow()?.inner().clone().into();
        
                let position = value.get::<_, TypedUserData<Traitor<vec::LiteVector3>>>("position")?;
                let position = position.borrow()?.inner().clone().into();
        
                let feature = value.get::<_, Traitor<lite_physics::LiteFeatureId>>("feature")?;
                let feature = feature.inner().clone().into();
        
                let toi = value.get::<_, f32>("toi")?;
                let toi = toi;
        
                Ok(Traitor::new(lite_physics::LiteIntersection { collider, normal, position, feature, toi }))
            }
        }
    