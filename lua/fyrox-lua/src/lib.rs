pub(crate) mod debug;
pub(crate) mod external_script_proxy;
pub(crate) mod fmt_pretty;
pub(crate) mod fyrox_lua_plugin;
pub(crate) mod generated;
pub(crate) mod lua_lang;
pub(crate) mod lua_lifecycle;
pub(crate) mod lua_utils;
pub(crate) mod manual_lua_bindings;
pub(crate) mod script_class;
pub(crate) mod script_metadata;
pub(crate) mod script_object_residence;
pub(crate) mod typed_userdata;
pub(crate) mod user_data_plus;
pub(crate) mod user_script_impl;
pub(crate) mod lua_script_metadata;

pub use fyrox_lua_plugin::LuaPlugin;

pub(crate) mod script_object {
    use crate::lua_lang::LuaLang;

    pub type ScriptObject = fyrox_lite::script_object::ScriptObject<LuaLang>;
    pub type ScriptFieldValue = fyrox_lite::script_object::ScriptFieldValue<LuaLang>;
}
