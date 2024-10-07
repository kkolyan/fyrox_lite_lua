use std::mem;

use crate::{
    fyrox_lite_class::Traitor, lua_error, plugin::LuaPlugin, script::LuaScript,
    script_object::ScriptObject, typed_userdata::TypedUserData,
};
use fyrox_lite::{script_context::with_script_context, spi::UserScript, LiteDataType};
use mlua::Value;
use send_wrapper::SendWrapper;

impl<'a> UserScript for TypedUserData<'a, ScriptObject> {
    type Plugin = LuaPlugin;

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

    fn find_plugin_script(class_name: &str) -> Result<Self, Self::LangSpecificError> {
        with_script_context(|ctx| {
            let Some(plugins) = ctx.plugins.as_mut() else {
                return Err(lua_error!("plugins not available here"));
            };
            let plugin = plugins
                .of_type_mut::<Self::Plugin>()
                .expect("WTF: Lua Plugin not found!");
            for script in plugin.scripts.borrow_mut().inner_mut().iter_mut() {
                if script.name == class_name {
                    return Ok(script.data.inner_unpacked().unwrap());
                }
            }
            Err(lua_error!("plugin script not found: {}", class_name))
        })
    }
}

impl LiteDataType for Traitor<SendWrapper<Value<'static>>> {}

impl<'a> LiteDataType for TypedUserData<'a, ScriptObject> {}
