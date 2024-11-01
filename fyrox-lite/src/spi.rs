use std::fmt::Debug;

use fyrox::script::{ScriptMessagePayload, ScriptTrait};

pub trait UserScript: Sized + LiteDataType {
    type Plugin: fyrox::plugin::Plugin;
    type ProxyScript: ScriptTrait;
    type LangSpecificError: Clone + Debug;
    type UserScriptMessage: ScriptMessagePayload + LiteDataType;
    type UserScriptGenericStub: LiteDataType + Copy;

    fn extract_from(
        proxy: &mut Self::ProxyScript,
        class_name: &str,
        ctx: &mut Self::Plugin,
    ) -> Option<Self>;

    fn into_proxy_script(self) -> Result<Self::ProxyScript, Self::LangSpecificError>;

    fn new_instance(class: &str) -> Result<Self, Self::LangSpecificError>;

    fn find_plugin_script(class_name: &str) -> Result<Self, Self::LangSpecificError>;

    fn create_error(msg: &str) -> Self::LangSpecificError;
}

/// implemented only by the types from `lite_model::DataType` (mostly by proc macros)
pub trait LiteDataType: Clone + Debug {
    fn compiles_if_type_is_allowed() {}
}

impl<T: LiteDataType> LiteDataType for Vec<T> {}
impl<T: LiteDataType> LiteDataType for Option<T> {}
impl<T: LiteDataType, E: Clone + Debug> LiteDataType for Result<T, E> {}

impl LiteDataType for String {}
impl LiteDataType for u8 {}
impl LiteDataType for i32 {}
impl LiteDataType for i64 {}
impl LiteDataType for f32 {}
impl LiteDataType for f64 {}
impl LiteDataType for bool {}
impl LiteDataType for () {}
