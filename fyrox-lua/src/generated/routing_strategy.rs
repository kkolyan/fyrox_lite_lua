
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
	
		}
	