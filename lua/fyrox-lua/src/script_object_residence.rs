use fyrox::core::log::Log;
use send_wrapper::SendWrapper;

use crate::{lua_lang::{LuaLang, UnpackedScriptObjectVisit}, lua_lifecycle::lua_vm, script_object::ScriptObject, typed_userdata::TypedUserData, user_data_plus::Traitor, LuaPlugin};

pub type ScriptResidence = fyrox_lite::script_object_residence::ScriptResidence<LuaLang>;

