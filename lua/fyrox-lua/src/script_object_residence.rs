use fyrox::core::log::Log;
use send_wrapper::SendWrapper;

use crate::{lua_lang::{LuaLang, UnpackedScriptObjectVisit}, lua_lifecycle::lua_vm, script_object::ScriptObject, typed_userdata::TypedUserData, user_data_plus::Traitor, LuaPlugin};

pub type ScriptResidence = fyrox_lite::script_object_residence::ScriptResidence<LuaLang>;

pub fn inner_unpacked(residence: &ScriptResidence) -> Option<TypedUserData<'static, Traitor<ScriptObject>>> {
    match residence {
        ScriptResidence::Packed(_it) => None,
        ScriptResidence::Unpacked(it) => Some(TypedUserData::clone(&it.0)),
    }
}


pub fn ensure_unpacked(residence: &mut ScriptResidence, plugin: &mut LuaPlugin) {
    if plugin.failed {
        // don't spam logs, though, plugin is completely broken at this point
        return;
    }
    if residence.is_packed() {
        // script was just loaded from the scene file or safe game. unpack it!
        let data = match residence {
            ScriptResidence::Packed(it) => {
                let so = lua_vm()
                    .create_userdata(Traitor::new(it.clone()))
                    .map(TypedUserData::<Traitor<ScriptObject>>::new)
                    .map(SendWrapper::new)
                    .map(UnpackedScriptObjectVisit)
                    .map(ScriptResidence::Unpacked);
                match so {
                    Ok(it) => it,
                    Err(err) => {
                        Log::err(format!("failed to unpack LuaScript: {:?}", err));
                        plugin.failed = true;
                        return;
                    }
                }
            }
            ScriptResidence::Unpacked(_) => panic!("WTF?"),
        };
        *residence = data;
    }
}
