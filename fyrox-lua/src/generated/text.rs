
		#![allow(unused_variables)]

		use fyrox_lite::*;
		use fyrox_lite_math::*;
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
		
			}
	
			fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
	
				methods.add_method_mut(
					"new",
					|lua, this, (state): (Traitor<lite_ui::TextBuilder>)| {
			
						let state = state.inner().clone().into();
				
						let ret = lite_ui::LiteText::new(state);
                        let ret = Traitor::new(lite_ui::LiteText::from(ret));
						Ok(ret)
					},
				);
			
			}
	
			fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
	
				fields.add_field_method_set("text_async", |lua, this, value: mlua::String| {
					this.set_text_async(value.to_str()?.to_string());
					Ok(())
				});
		
			}
	
			fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
	
			}
	
		}
	