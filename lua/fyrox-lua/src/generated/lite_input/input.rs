
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite::lite_input::Input {
    const CLASS_NAME: &'static str = "Input";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("is_mouse_button_down", |lua, this, (button): (i32)| {
            let button = button;
            let ret = fyrox_lite::lite_input::Input::is_mouse_button_down(button);
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut("is_mouse_button_up", |lua, this, (button): (i32)| {
            let button = button;
            let ret = fyrox_lite::lite_input::Input::is_mouse_button_up(button);
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut("is_mouse_button", |lua, this, (button): (i32)| {
            let button = button;
            let ret = fyrox_lite::lite_input::Input::is_mouse_button(button);
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut(
            "is_key_down",
            |lua, this, (key): (TypedUserData<Traitor<fyrox_lite::lite_input::LiteKeyCode>>)| {
                let key = key.borrow()?.inner().clone().into();
                let ret = fyrox_lite::lite_input::Input::is_key_down(key);
                let ret = ret;
                Ok(ret)
            },
        );
        methods.add_method_mut(
            "is_key_up",
            |lua, this, (key): (TypedUserData<Traitor<fyrox_lite::lite_input::LiteKeyCode>>)| {
                let key = key.borrow()?.inner().clone().into();
                let ret = fyrox_lite::lite_input::Input::is_key_up(key);
                let ret = ret;
                Ok(ret)
            },
        );
        methods.add_method_mut(
            "is_key",
            |lua, this, (key): (TypedUserData<Traitor<fyrox_lite::lite_input::LiteKeyCode>>)| {
                let key = key.borrow()?.inner().clone().into();
                let ret = fyrox_lite::lite_input::Input::is_key(key);
                let ret = ret;
                Ok(ret)
            },
        );
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("MouseLeft", |lua, this| {
            let value = fyrox_lite::lite_input::Input::MouseLeft;
            Ok(value)
        });
        fields.add_field_method_get("MouseRight", |lua, this| {
            let value = fyrox_lite::lite_input::Input::MouseRight;
            Ok(value)
        });
        fields.add_field_method_get("MouseMiddle", |lua, this| {
            let value = fyrox_lite::lite_input::Input::MouseMiddle;
            Ok(value)
        });
        fields.add_field_method_get("MouseBack", |lua, this| {
            let value = fyrox_lite::lite_input::Input::MouseBack;
            Ok(value)
        });
        fields.add_field_method_get("MouseForward", |lua, this| {
            let value = fyrox_lite::lite_input::Input::MouseForward;
            Ok(value)
        });
        fields.add_field_method_get("mouse_move", |lua, this| {
            let value = fyrox_lite::lite_input::Input::get_mouse_move();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(
                value,
            )))
        });
        fields.add_field_method_get("mouse_scroll", |lua, this| {
            let value = fyrox_lite::lite_input::Input::get_mouse_scroll();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(
                value,
            )))
        });
    }
}
