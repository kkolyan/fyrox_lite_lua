
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_scene::LiteScene {
			const CLASS_NAME: &'static str = "Scene";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"load_async",
					|lua, this, (scene_path): (mlua::String)| {
			
						let scene_path = scene_path.to_str()?.to_string();
				
						let ret = lite_scene::LiteScene::load_async(scene_path);
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
		}
	