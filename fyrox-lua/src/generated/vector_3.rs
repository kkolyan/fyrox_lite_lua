
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
					"get_x",
					|lua, this, (): ()| {
			
						let ret = this.get_x();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"get_y",
					|lua, this, (): ()| {
			
						let ret = this.get_y();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"get_z",
					|lua, this, (): ()| {
			
						let ret = this.get_z();
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"set_x",
					|lua, this, (value): (f32)| {
			
						let value = value;
				
						let ret = this.set_x(value);
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"set_y",
					|lua, this, (value): (f32)| {
			
						let value = value;
				
						let ret = this.set_y(value);
                        let ret = ret;
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"set_z",
					|lua, this, (value): (f32)| {
			
						let value = value;
				
						let ret = this.set_z(value);
                        let ret = ret;
						Ok(ret)
					},
				);
			
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
					"x_axis",
					|lua, this, (): ()| {
			
						let ret = vec::LiteVector3::x_axis();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"y_axis",
					|lua, this, (): ()| {
			
						let ret = vec::LiteVector3::y_axis();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
				methods.add_method_mut(
					"z_axis",
					|lua, this, (): ()| {
			
						let ret = vec::LiteVector3::z_axis();
                        let ret = Traitor::new(vec::LiteVector3::from(ret));
						Ok(ret)
					},
				);
			
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
		}
	