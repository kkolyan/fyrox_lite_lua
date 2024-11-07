use std::fmt::Debug;
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use fyrox::script::{ScriptMessagePayload, ScriptTrait};

pub trait ClassId: LiteDataType + Clone {
    fn lookup_class_name(&self) -> String;
}

// TODO merge this trait into Lang trait
pub trait UserScript: Sized + LiteDataType {
    type Plugin: fyrox::plugin::Plugin;
    type ProxyScript: ScriptTrait;

    type ClassId: ClassId;
    type LangSpecificError: Clone + Debug;
    type UserScriptMessage: ScriptMessagePayload + LiteDataType;
    type UserScriptGenericStub: LiteDataType + Copy;

    fn extract_from(
        node: Handle<Node>,
        proxy: &mut Self::ProxyScript,
        class_id: &Self::ClassId,
        ctx: &mut Self::Plugin,
    ) -> Option<Self>;

    fn into_proxy_script(self, class_id: &Self::ClassId) -> Result<Self::ProxyScript, Self::LangSpecificError>;

    fn new_instance(node: Handle<Node>, class_id: &Self::ClassId) -> Result<Self, Self::LangSpecificError>;

    fn find_plugin_script(class_name: &Self::ClassId) -> Result<Self, Self::LangSpecificError>;

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

impl ClassId for String {
    fn lookup_class_name(&self) -> String {
        self.clone()
    }
}
