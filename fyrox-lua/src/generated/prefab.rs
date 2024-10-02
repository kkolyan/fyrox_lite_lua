
		#![allow(unused_variables)]

		use fyrox_lite::*;
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
					|lua, this, (position, orientation): (TypedUserData<Traitor<fyrox_lite_math::LiteVector3>>, TypedUserData<Traitor<fyrox_lite_math::LiteQuaternion>>)| {
			
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
		}
	