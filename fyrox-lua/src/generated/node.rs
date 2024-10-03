
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_node::LiteNode {
			const CLASS_NAME: &'static str = "Node";
		
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"as_rigid_body",
					|lua, this, (): ()| {
			
						let ret = this.as_rigid_body();
                        let ret = if let Some(ret) = ret { Some(Traitor::new(lite_physics::LiteRigidBody::from(ret))) } else { None };
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"name",
					|lua, this, (): ()| {
			
						let ret = this.name();
                        let ret = if let Some(ret) = ret { Some(ret) } else { None };
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"is_alive",
					|lua, this, (): ()| {
			
						let ret = this.is_alive();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"destroy",
					|lua, this, (): ()| {
			
						let ret = this.destroy();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"global_position",
					|lua, this, (): ()| {
			
						let ret = this.global_position();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"local_position",
					|lua, this, (): ()| {
			
						let ret = this.local_position();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"local_rotation",
					|lua, this, (): ()| {
			
						let ret = this.local_rotation();
                        let ret = Traitor::new(quat::LiteQuaternion::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"send_hierarchical",
					|lua, this, (routing, payload): (TypedUserData<Traitor<lite_node::LiteRoutingStrategy>>, mlua::Value)| {
			
						let routing = routing.borrow()?.inner().clone().into();
				
						let payload = Traitor::new(send_wrapper::SendWrapper::new(unsafe { std::mem::transmute::<_, mlua::Value<'static>>(payload) } ));
				
						let ret = this.send_hierarchical::<TypedUserData<ScriptObject>>(routing, payload);
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"subscribe_to",
					|lua, this, (): ()| {
			
						let _stub = Default::default();
				
						let ret = this.subscribe_to::<TypedUserData<ScriptObject>>(_stub);
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"try_get_collider",
					|lua, this, (): ()| {
			
						let ret = this.try_get_collider();
                        let ret = if let Some(ret) = ret { Some(Traitor::new(lite_node::LiteNode::from(ret))) } else { None };
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"is_valid",
					|lua, this, (): ()| {
			
						let ret = this.is_valid();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"parent",
					|lua, this, (): ()| {
			
						let ret = this.parent();
                        let ret = Traitor::new(lite_node::LiteNode::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"add_script",
					|lua, this, (state): (TypedUserData<ScriptObject>)| {
			
						let state = state;
				
						let ret = this.add_script::<TypedUserData<ScriptObject>>(state);
                        let ret = ret?;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"find_script",
					|lua, this, (class_name): (mlua::String)| {
			
						let class_name = class_name.to_str()?.to_string();
				
						let _stub = Default::default();
				
						let ret = this.find_script::<TypedUserData<ScriptObject>>(class_name, _stub);
                        let ret = if let Some(ret) = ret { Some(ret) } else { None };
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"global_rotation",
					|lua, this, (): ()| {
			
						let ret = this.global_rotation();
                        let ret = Traitor::new(quat::LiteQuaternion::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"tag_is",
					|lua, this, (tag): (mlua::String)| {
			
						let tag = tag.to_str()?.to_string();
				
						let ret = this.tag_is(tag);
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
				fields.add_field_method_set("local_position", |lua, this, value: TypedUserData<Traitor<vec::LiteVector3>>| {
					this.set_local_position(value.borrow()?.inner().clone().into());
					Ok(())
				});
		
				fields.add_field_method_set("local_rotation", |lua, this, value: TypedUserData<Traitor<quat::LiteQuaternion>>| {
					this.set_local_rotation(value.borrow()?.inner().clone().into());
					Ok(())
				});
		
				fields.add_field_method_set("tag", |lua, this, value: mlua::String| {
					this.set_tag(value.to_str()?.to_string());
					Ok(())
				});
		
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
	
			}
	
		}
	