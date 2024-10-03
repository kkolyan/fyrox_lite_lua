
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_prefab::LitePrefab {
			const CLASS_NAME: &'static str = "Prefab";
		
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"instantiate_at",
					|lua, this, (position, orientation): (TypedUserData<Traitor<vec::LiteVector3>>, TypedUserData<Traitor<quat::LiteQuaternion>>)| {
			
						let position = position.borrow()?.inner().clone().into();
				
						let orientation = orientation.borrow()?.inner().clone().into();
				
						let ret = this.instantiate_at(position, orientation);
                        let ret = Traitor::new(lite_node::LiteNode::from(ret));
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
	
			}
	
		}
	