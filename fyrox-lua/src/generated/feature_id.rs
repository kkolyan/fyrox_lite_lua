
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			lua_error,
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_physics::LiteFeatureId {
			const CLASS_NAME: &'static str = "FeatureId";
		
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"Vertex",
					|lua, this, mut args: mlua::MultiValue| {
			
						let Some(_1) = args.pop_front() else {
							return Err(lua_error!("argument 1 (i32) missing"));
						};
						let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
						let _1 = _1;
			
						let value = lite_physics::LiteFeatureId::Vertex( _1 );
						Ok(Traitor::new(value))
		
					}
				);
		
				methods.add_method_mut(
					"Edge",
					|lua, this, mut args: mlua::MultiValue| {
			
						let Some(_1) = args.pop_front() else {
							return Err(lua_error!("argument 1 (i32) missing"));
						};
						let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
						let _1 = _1;
			
						let value = lite_physics::LiteFeatureId::Edge( _1 );
						Ok(Traitor::new(value))
		
					}
				);
		
				methods.add_method_mut(
					"Face",
					|lua, this, mut args: mlua::MultiValue| {
			
						let Some(_1) = args.pop_front() else {
							return Err(lua_error!("argument 1 (i32) missing"));
						};
						let _1 = <i32 as mlua::FromLua>::from_lua(_1, lua)?;
						let _1 = _1;
			
						let value = lite_physics::LiteFeatureId::Face( _1 );
						Ok(Traitor::new(value))
		
					}
				);
		
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
					
				fields.add_field_method_get("Unknown", |lua, this| {
					Ok(Traitor::new(lite_physics::LiteFeatureId::Unknown))
				});
		
			}
	
		}
	