use fyrox_lite::{spi::UserScript, LiteDataType};

use crate::{bindings_manual::{NativeHandle, NativeInstanceId}, external_script_proxy::ExternalScriptProxy, fyrox_c_plugin::CPlugin};

impl UserScript for NativeHandle {
    type Plugin = CPlugin;

    type ProxyScript = ExternalScriptProxy;

    type LangSpecificError = String;

    type UserScriptMessage = NativeInstanceId;

    type UserScriptGenericStub = ();

    fn extract_from(
        proxy: &mut Self::ProxyScript,
        class_name: &str,
        ctx: &mut Self::Plugin,
    ) -> Option<Self> {
        todo!()
    }

    fn into_proxy_script(self) -> Result<Self::ProxyScript, Self::LangSpecificError> {
        todo!()
    }

    fn new_instance(class: &str) -> Result<Self, Self::LangSpecificError> {
        todo!()
    }

    fn find_plugin_script(class_name: &str) -> Result<Self, Self::LangSpecificError> {
        todo!()
    }

    fn create_error(msg: &str) -> Self::LangSpecificError {
        todo!()
    }
}

impl LiteDataType for NativeInstanceId {}
impl LiteDataType for NativeHandle {}