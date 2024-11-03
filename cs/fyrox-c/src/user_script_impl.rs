use fyrox::core::Uuid;
use fyrox::script::PluginsRefMut;
use fyrox_lite::{spi::UserScript, LiteDataType};
use fyrox_lite::script_context::with_script_context;
use fyrox_lite::script_object_residence::ScriptResidence;
use fyrox_lite::spi::ClassId;
use crate::{bindings_manual::{NativeHandle, NativeInstanceId}, external_script_proxy::ExternalScriptProxy, fyrox_c_plugin::CPlugin};
use crate::bindings_manual::NativeClassId;
use crate::c_lang::UnpackedObject;
use crate::scripted_app::APP;


impl LiteDataType for UnpackedObject {}

impl UserScript for UnpackedObject {
    type Plugin = CPlugin;

    type ProxyScript = ExternalScriptProxy;

    type ClassId = NativeClassId;

    type LangSpecificError = crate::LangSpecificError;

    type UserScriptMessage = crate::UserScriptMessage;

    type UserScriptGenericStub = ();

    fn extract_from(
        proxy: &mut Self::ProxyScript,
        class_name: &Self::ClassId,
        plugin: &mut Self::Plugin,
    ) -> Option<Self> {
        if &proxy.class == class_name {
            proxy.data.ensure_unpacked(&mut plugin.failed);
            let script_data = &mut proxy.data.inner_unpacked();
            return Some(*script_data.expect("expected to be unpacked here"));
        }
        None
    }

    fn into_proxy_script(self, class: &Self::ClassId) -> Result<Self::ProxyScript, Self::LangSpecificError> {
        APP.with_borrow(|it| {
            Ok(ExternalScriptProxy {
                name: class.lookup_class_name(),
                class: class.clone(),
                data: ScriptResidence::Unpacked(self),
            })
        })
    }

    fn new_instance(class: &Self::ClassId) -> Result<Self, Self::LangSpecificError> {
        APP.with_borrow(|it| {
            let app = it.as_ref().unwrap();
            let uuid = app.uuid_by_class.get(class).unwrap();
            let md = app.scripts.get(uuid).unwrap();
            let instance_id = (app.functions.create_script_instance)(md.id);
            Ok(UnpackedObject {
                uuid: *uuid,
                class: md.id,
                instance: instance_id,
            })
        })
    }

    fn find_plugin_script(class: &Self::ClassId) -> Result<Self, Self::LangSpecificError> {
        with_script_context(|it| {
            match &it.plugins {
                None => Err(format!("global scripts not available in this context")),
                Some(it) => {
                    let scripts = it.get::<CPlugin>().scripts.borrow();
                    let script = scripts.inner().iter().find(|it| &it.class == class);
                    if let Some(script) = script {
                        Ok(*script.data.inner_unpacked().unwrap())
                    } else {
                        Err(format!("script not found: '{}'", class.lookup_class_name()))
                    }
                }
            }
        })
    }

    fn create_error(msg: &str) -> Self::LangSpecificError {
        msg.to_string()
    }
}

impl LiteDataType for NativeInstanceId {}
impl LiteDataType for NativeHandle {}

impl LiteDataType for NativeClassId {}
impl ClassId for NativeClassId {
    fn lookup_class_name(&self) -> String {
        APP.with_borrow(|app| {
            let app = app.as_ref().unwrap();
            let uuid = app.uuid_by_class.get(self).unwrap();
            let x = app.scripts.get(uuid).unwrap();
            x.md.class.clone()
        })
    }
}