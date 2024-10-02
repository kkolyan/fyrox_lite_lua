
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_physics::LiteRigidBody {
			const CLASS_NAME: &'static str = "RigidBody";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"apply_force",
					|lua, this, (force): (TypedUserData<Traitor<fyrox_lite_math::LiteVector3>>)| {
			
						let force = force.borrow()?.inner().clone().into();
				
						let ret = this.apply_force(force);
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
			}
		}
	