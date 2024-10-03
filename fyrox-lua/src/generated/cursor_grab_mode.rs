
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

impl FyroxUserData for lite_window::LiteCursorGrabMode {
    const CLASS_NAME: &'static str = "CursorGrabMode";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("None", |lua, this, mut args: mlua::MultiValue| {});

        methods.add_method_mut("Confined", |lua, this, mut args: mlua::MultiValue| {});

        methods.add_method_mut("Locked", |lua, this, mut args: mlua::MultiValue| {});
    }
}
