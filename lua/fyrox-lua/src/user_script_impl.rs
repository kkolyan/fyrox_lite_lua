use std::mem;

use crate::{
    external_script_proxy::ExternalScriptProxy, fyrox_lua_plugin::LuaPlugin, lua_error, lua_lang::UnpackedScriptObjectVisit, lua_lifecycle::lua_vm, script_class::ScriptClass, script_object::ScriptObject, script_object_residence::{ensure_unpacked, inner_unpacked}, typed_userdata::TypedUserData, user_data_plus::Traitor
};
use fyrox_lite::{script_context::with_script_context, spi::UserScript, LiteDataType};
use mlua::{UserDataRef, Value};
use send_wrapper::SendWrapper;

impl<'a> UserScript for TypedUserData<'a, Traitor<ScriptObject>> {
    type Plugin = LuaPlugin;

    type ProxyScript = ExternalScriptProxy;

    type LangSpecificError = mlua::Error;

    type UserScriptMessage = Traitor<SendWrapper<Value<'static>>>;

    type UserScriptGenericStub = ();

    fn extract_from(
        proxy: &mut Self::ProxyScript,
        class_name: &str,
        plugin: &mut Self::Plugin,
    ) -> Option<Self> {
        if proxy.name == class_name {
            ensure_unpacked(&mut proxy.data, plugin);
            let script_data = inner_unpacked(&mut proxy.data);
            return Some(script_data.expect("expected to be unpacked here"));
        }
        None
    }

    fn into_proxy_script(self) -> mlua::Result<Self::ProxyScript> {
        let name = self.borrow()?.def.metadata.class.to_string();
        // it's sound, because Lua outlives a process
        let ud: TypedUserData<'static, Traitor<ScriptObject>> = unsafe { mem::transmute(self) };
        let data = crate::script_object_residence::ScriptResidence::Unpacked(UnpackedScriptObjectVisit(SendWrapper::new(ud)));
        Ok(ExternalScriptProxy { name, data })
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
                    return Ok(inner_unpacked(&mut script.data).unwrap());
                }
            }
            Err(lua_error!("plugin script not found: {}", class_name))
        })
    }

    fn create_error(msg: &str) -> Self::LangSpecificError {
        mlua::Error::runtime(msg)
    }

    fn new_instance(class_name: &str) -> Result<Self, Self::LangSpecificError> {
        let class = lua_vm()
            .globals()
            .get::<_, Option<UserDataRef<ScriptClass>>>(class_name)?;
        let Some(class) = class else {
            return Err(lua_error!("class not found: {}", class_name));
        };
        let Some(def) = &class.def else {
            return Err(lua_error!("invalid class: {}", class_name));
        };
        let obj = lua_vm().create_userdata(Traitor::new(ScriptObject::new(def)))?;
        Ok(TypedUserData::<Traitor<ScriptObject>>::new(obj))
    }
}

impl LiteDataType for Traitor<SendWrapper<Value<'static>>> {}

impl<'a> LiteDataType for TypedUserData<'a, Traitor<ScriptObject>> {}
