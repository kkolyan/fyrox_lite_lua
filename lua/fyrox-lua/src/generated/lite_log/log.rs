
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
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for fyrox_lite::lite_log::LiteLog {
    const CLASS_NAME: &'static str = "Log";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("info", |lua, this, (msg): (mlua::String)| {
            let msg = msg.to_str()?.to_string();

            let ret = fyrox_lite::lite_log::LiteLog::info(msg);
            let ret = ret;
            Ok(ret)
        });

        methods.add_method_mut("warn", |lua, this, (msg): (mlua::String)| {
            let msg = msg.to_str()?.to_string();

            let ret = fyrox_lite::lite_log::LiteLog::warn(msg);
            let ret = ret;
            Ok(ret)
        });

        methods.add_method_mut("err", |lua, this, (msg): (mlua::String)| {
            let msg = msg.to_str()?.to_string();

            let ret = fyrox_lite::lite_log::LiteLog::err(msg);
            let ret = ret;
            Ok(ret)
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}
}
