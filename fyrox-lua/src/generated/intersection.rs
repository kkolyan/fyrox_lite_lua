

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

        impl <'lua> mlua::IntoLua<'lua> for Traitor<lite_physics::LiteIntersection> {
            fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table({
                    let t = lua.create_table()?;
    
                    t.set("collider", {
                        let collider = self.collider;
                        Traitor::new(lite_node::LiteNode::from(collider))
                    })?;
        
                    t.set("normal", {
                        let normal = self.normal;
                        Traitor::new(vec::LiteVector3::from(normal))
                    })?;
        
                    t.set("position", {
                        let position = self.position;
                        Traitor::new(vec::LiteVector3::from(position))
                    })?;
        
                    t.set("feature", {
                        let feature = self.feature;
                        Traitor::new(lite_physics::LiteFeatureId::from(feature))
                    })?;
        
                    t.set("toi", {
                        let toi = self.toi;
                        toi
                    })?;
        
                    t
                }))
            }
        }
    