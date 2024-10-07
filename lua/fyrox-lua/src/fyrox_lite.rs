use std::mem;

use crate::{
    fyrox_lite_class::Traitor,
    script::LuaScript,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};
use fyrox_lite::{
    spi::UserScript,
    LiteDataType,
};
use mlua::Value;
use send_wrapper::SendWrapper;


impl<'a> UserScript for TypedUserData<'a, ScriptObject> {
    type ProxyScript = LuaScript;

    type LangSpecificError = mlua::Error;
    
    type UserScriptMessage = Traitor<SendWrapper<Value<'static>>>;

    type UserScriptGenericStub = ();

    fn extract_from(proxy: &Self::ProxyScript, class_name: &str) -> Option<Self> {
        if proxy.name == class_name {
            let script_data = proxy.data.inner_unpacked();
            return Some(script_data.expect("expected to be unpacked here"));
        }
        None
    }

    fn into_proxy_script(self) -> mlua::Result<Self::ProxyScript> {
        let name = self.borrow()?.def.metadata.class.to_string();
        // it's sound, because Lua outlives a process
        let ud: TypedUserData<'static, ScriptObject> = unsafe { mem::transmute(self) };
        let data = crate::script_data::ScriptData::Unpacked(SendWrapper::new(ud));
        Ok(LuaScript { name, data })
    }
}

impl LiteDataType for Traitor<SendWrapper<Value<'static>>> {}

impl<'a> LiteDataType for TypedUserData<'a, ScriptObject> {}