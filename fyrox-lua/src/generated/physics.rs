
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_physics::LitePhysics {
			const CLASS_NAME: &'static str = "Physics";
		
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"cast_ray",
					|lua, this, (opts): (TypedUserData<Traitor<lite_physics::LiteRayCastOptions>>)| {
			
						let opts = opts.borrow()?.inner().clone().into();
				
						let ret = lite_physics::LitePhysics::cast_ray(opts);
                        let ret = lua.create_table_from(ret.into_iter().map(|it| Traitor::new(lite_physics::LiteIntersection::from(it))).enumerate());
						Ok(ret)
					},
				);
			
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
					
				fields.add_field_method_get("EXCLUDE_FIXED", |lua, this| {
					let value = lite_physics::LitePhysics::EXCLUDE_FIXED;
					Ok(value)
				});
						
				fields.add_field_method_get("EXCLUDE_KINEMATIC", |lua, this| {
					let value = lite_physics::LitePhysics::EXCLUDE_KINEMATIC;
					Ok(value)
				});
						
				fields.add_field_method_get("EXCLUDE_DYNAMIC", |lua, this| {
					let value = lite_physics::LitePhysics::EXCLUDE_DYNAMIC;
					Ok(value)
				});
						
				fields.add_field_method_get("EXCLUDE_SENSORS", |lua, this| {
					let value = lite_physics::LitePhysics::EXCLUDE_SENSORS;
					Ok(value)
				});
						
				fields.add_field_method_get("EXCLUDE_SOLIDS", |lua, this| {
					let value = lite_physics::LitePhysics::EXCLUDE_SOLIDS;
					Ok(value)
				});
						
				fields.add_field_method_get("ONLY_DYNAMIC", |lua, this| {
					let value = lite_physics::LitePhysics::ONLY_DYNAMIC;
					Ok(value)
				});
						
				fields.add_field_method_get("ONLY_KINEMATIC", |lua, this| {
					let value = lite_physics::LitePhysics::ONLY_KINEMATIC;
					Ok(value)
				});
						
				fields.add_field_method_get("ONLY_FIXED", |lua, this| {
					let value = lite_physics::LitePhysics::ONLY_FIXED;
					Ok(value)
				});
		
			}
	
		}
	