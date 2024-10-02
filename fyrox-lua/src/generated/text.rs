
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use mlua;

		use crate::{
			fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
			script_object::ScriptObject,
			typed_userdata::TypedUserData,
		};

		impl FyroxUserData for lite_ui::LiteText {
			const CLASS_NAME: &'static str = "Text";
			
	    	fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
				methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
					Ok(format!("{:?}", this.inner()))
				});
		
				methods.add_method_mut(
					"set_text_async",
					|lua, this, (text): (mlua::String)| {
			
						let text = text.to_str()?.to_string();
				
						let ret = this.set_text_async(text);
                        let ret = ret;
						Ok(ret)
					},
				);
			
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"new",
					|lua, this, (state): (TypedUserData<Traitor<lite_ui::TextBuilder>>)| {
			
						let state = state.borrow()?.inner().clone().into();
				
						let ret = lite_ui::LiteText::new(state);
                        let ret = Traitor::new(lite_ui::LiteText::from(ret));
						Ok(ret)
					},
				);
			
			}
		}
	