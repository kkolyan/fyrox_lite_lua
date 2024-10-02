
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for quat::LiteQuaternion {
			const CLASS_NAME: &'static str = "Quaternion";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"mul__LiteVector",
					|lua, this, (o): (TypedUserData<Traitor<vec::LiteVector3>>)| {
			
						let o = o.borrow()?.inner().clone().into();
				
						let ret = this.mul__LiteVector(o);
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"mul__LiteQuaternion",
					|lua, this, (rot_delta): (TypedUserData<Traitor<quat::LiteQuaternion>>)| {
			
						let rot_delta = rot_delta.borrow()?.inner().clone().into();
				
						let ret = this.mul__LiteQuaternion(rot_delta);
                        let ret = Traitor::new(quat::LiteQuaternion::from(ret));
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"face_towards",
					|lua, this, (dir, up): (TypedUserData<Traitor<vec::LiteVector3>>, TypedUserData<Traitor<vec::LiteVector3>>)| {
			
						let dir = dir.borrow()?.inner().clone().into();
				
						let up = up.borrow()?.inner().clone().into();
				
						let ret = quat::LiteQuaternion::face_towards(dir, up);
                        let ret = Traitor::new(quat::LiteQuaternion::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"from_axis_angle",
					|lua, this, (axis, angle): (TypedUserData<Traitor<vec::LiteVector3>>, f32)| {
			
						let axis = axis.borrow()?.inner().clone().into();
				
						let angle = angle;
				
						let ret = quat::LiteQuaternion::from_axis_angle(axis, angle);
                        let ret = Traitor::new(quat::LiteQuaternion::from(ret));
						Ok(ret)
					},
				);
			
			}
		}
	