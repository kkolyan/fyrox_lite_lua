
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

		impl FyroxUserData for lite_node::LiteRoutingStrategy {
			const CLASS_NAME: &'static str = "RoutingStrategy";
		
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
					
				fields.add_field_method_get("Up", |lua, this| {
					Ok(Traitor::new(lite_node::LiteRoutingStrategy::Up))
				});
						
				fields.add_field_method_get("Down", |lua, this| {
					Ok(Traitor::new(lite_node::LiteRoutingStrategy::Down))
				});
		
			}
	
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
				fields.add_field_method_get("Up", |lua, this| {
		
					let lite_node::LiteRoutingStrategy::Up = this.inner() else {
						return Ok(mlua::Value::Nil);
					};
					Ok(mlua::Value::Boolean(true))
    
				});
		
				fields.add_field_method_get("Down", |lua, this| {
		
					let lite_node::LiteRoutingStrategy::Down = this.inner() else {
						return Ok(mlua::Value::Nil);
					};
					Ok(mlua::Value::Boolean(true))
    
				});
		
			}
	
		}
	