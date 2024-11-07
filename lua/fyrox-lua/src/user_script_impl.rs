use std::cmp::Ordering;
use std::mem;

use crate::{
    external_script_proxy::ExternalScriptProxy, fyrox_lua_plugin::LuaPlugin, lua_error,
    lua_lang::UnpackedScriptObjectVisit, lua_lifecycle::lua_vm, script_class::ScriptClass,
    script_object::ScriptObject, typed_userdata::TypedUserData, user_data_plus::Traitor,
};
use fyrox_lite::{script_context::with_script_context, spi::UserScript, LiteDataType};
use mlua::{UserDataRef, Value};
use send_wrapper::SendWrapper;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use fyrox_lite::spi::ClassId;

impl<'a> UserScript for TypedUserData<'a, Traitor<ScriptObject>> {
    type Plugin = LuaPlugin;

    type ProxyScript = ExternalScriptProxy;
    
    type ClassId = String;

    type LangSpecificError = mlua::Error;

    type UserScriptMessage = Traitor<SendWrapper<Value<'static>>>;

    type UserScriptGenericStub = ();

    fn extract_from(
        node: Handle<Node>,
        proxy: &mut Self::ProxyScript,
        class: &Self::ClassId,
        plugin: &mut Self::Plugin,
    ) -> Option<Self> {
        if &proxy.name == class {
            proxy.data.ensure_unpacked(&mut plugin.failed, node);
            let script_data = &mut proxy.data.inner_unpacked();
            return Some(TypedUserData::clone(
                &script_data.expect("expected to be unpacked here").0,
            ));
        }
        None
    }

    fn into_proxy_script(self, _class: &Self::ClassId) -> mlua::Result<Self::ProxyScript> {
        let name = self.borrow()?.def.metadata.class.to_string();
        // it's sound, because Lua outlives a process
        let ud: TypedUserData<'static, Traitor<ScriptObject>> = unsafe { mem::transmute(self) };
        let data = crate::script_object_residence::ScriptResidence::Unpacked(
            UnpackedScriptObjectVisit(SendWrapper::new(ud)),
        );
        Ok(ExternalScriptProxy { name, data })
    }

    fn find_plugin_script(class: &Self::ClassId) -> Result<Self, Self::LangSpecificError> {
        with_script_context(|ctx| {
            let Some(plugins) = ctx.plugins.as_mut() else {
                return Err(lua_error!("plugins not available here"));
            };
            let plugin = plugins
                .of_type_mut::<Self::Plugin>()
                .expect("WTF: Lua Plugin not found!");
            for script in plugin.scripts.borrow_mut().inner_mut().iter_mut() {
                if &script.name == class {
                    return Ok(TypedUserData::clone(
                        &script.data.inner_unpacked().unwrap().0,
                    ));
                }
            }
            Err(lua_error!("plugin script not found: {}", class))
        })
    }

    fn create_error(msg: &str) -> Self::LangSpecificError {
        mlua::Error::runtime(msg)
    }

    fn new_instance(node: Handle<Node>, class_id: &Self::ClassId) -> Result<Self, Self::LangSpecificError> {
        let class = lua_vm()
            .globals()
            .get::<_, Option<UserDataRef<ScriptClass>>>(class_id.clone())?;
        let Some(class) = class else {
            return Err(lua_error!("class not found: {}", class_id.lookup_class_name()));
        };
        let Some(def) = &class.def else {
            return Err(lua_error!("invalid class: {}", class_id.lookup_class_name()));
        };
        let mut script_object = ScriptObject::new(def);
        script_object.node = node;
        let obj = lua_vm().create_userdata(Traitor::new(script_object))?;
        Ok(TypedUserData::<Traitor<ScriptObject>>::new(obj))
    }
}

impl LiteDataType for Traitor<SendWrapper<Value<'static>>> {}

impl<'a> LiteDataType for TypedUserData<'a, Traitor<ScriptObject>> {}
