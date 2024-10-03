
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for vec::LiteVector3 {
			const CLASS_NAME: &'static str = "Vector3";
		
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"mul",
					|lua, this, (o): (f32)| {
			
						let o = o;
				
						let ret = this.mul(o);
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"add",
					|lua, this, (o): (TypedUserData<Traitor<vec::LiteVector3>>)| {
			
						let o = o.borrow()?.inner().clone().into();
				
						let ret = this.add(o);
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"normalize",
					|lua, this, (): ()| {
			
						let ret = this.normalize();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"sub",
					|lua, this, (o): (TypedUserData<Traitor<vec::LiteVector3>>)| {
			
						let o = o.borrow()?.inner().clone().into();
				
						let ret = this.sub(o);
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"magnitude",
					|lua, this, (): ()| {
			
						let ret = this.magnitude();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"normalize_inplace",
					|lua, this, (): ()| {
			
						let ret = this.normalize_inplace();
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"zero",
					|lua, this, (): ()| {
			
						let ret = vec::LiteVector3::zero();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"new",
					|lua, this, (x, y, z): (f32, f32, f32)| {
			
						let x = x;
				
						let y = y;
				
						let z = z;
				
						let ret = vec::LiteVector3::new(x, y, z);
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
				fields.add_field_method_get("x", |lua, this| {
					let value = this.get_x();
					Ok(value)
				});
		
				fields.add_field_method_get("y", |lua, this| {
					let value = this.get_y();
					Ok(value)
				});
		
				fields.add_field_method_get("z", |lua, this| {
					let value = this.get_z();
					Ok(value)
				});
		
				fields.add_field_method_set("x", |lua, this, value: f32| {
					this.set_x(value);
					Ok(())
				});
		
				fields.add_field_method_set("y", |lua, this, value: f32| {
					this.set_y(value);
					Ok(())
				});
		
				fields.add_field_method_set("z", |lua, this, value: f32| {
					this.set_z(value);
					Ok(())
				});
		
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
					
				fields.add_field_method_get("X", |lua, this| {
					let value = vec::LiteVector3::get_X();
					Ok(Traitor::new(vec::LiteVector3::from(value)))
				});
						
				fields.add_field_method_get("Y", |lua, this| {
					let value = vec::LiteVector3::get_Y();
					Ok(Traitor::new(vec::LiteVector3::from(value)))
				});
						
				fields.add_field_method_get("Z", |lua, this| {
					let value = vec::LiteVector3::get_Z();
					Ok(Traitor::new(vec::LiteVector3::from(value)))
				});
		
			}
	
		}
	