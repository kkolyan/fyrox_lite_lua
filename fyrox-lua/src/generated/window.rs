
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_window::LiteWindow {
			const CLASS_NAME: &'static str = "Window";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"set_cursor_grab",
					|lua, this, (mode): (TypedUserData<Traitor<lite_window::LiteCursorGrabMode>>)| {
			
						let mode = mode.borrow()?.inner().clone().into();
				
						let ret = lite_window::LiteWindow::set_cursor_grab(mode);
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
		}
	