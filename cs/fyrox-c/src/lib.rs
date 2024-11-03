pub(crate) mod arena;
pub(crate) mod bindings_lite_2;
pub(crate) mod bindings_manual;
pub(crate) mod c_lang;
pub(crate) mod c_script_metadata;
pub(crate) mod external_script_proxy;
pub(crate) mod fyrox_c_plugin;
pub(crate) mod hello;
pub(crate) mod invoke_callback;
pub(crate) mod native_utils;
pub(crate) mod packed_script;
pub(crate) mod scripted_app;
pub(crate) mod user_script_impl;
pub(crate) mod buffer;
mod string;
pub(crate) mod executor_cs;

pub(crate) use arena::Arena;
use crate::bindings_manual::UserScriptMessage;
use crate::c_lang::UnpackedObject;

pub(crate) type LangSpecificError = String;
pub(crate) type UserScriptMessageImpl = UserScriptMessage;
pub(crate) type UserScriptImpl = UnpackedObject;
