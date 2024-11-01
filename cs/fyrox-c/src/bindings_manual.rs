use std::{
    ffi::{c_char, CString},
    fmt::Display,
};

use fyrox::core::{algebra::iter, pool::Handle};
use fyrox_lite::{lite_math::{PodQuaternion, PodVector3}, spi::UserScript, LiteDataType};

use crate::{
    bindings_lite::{NativeQuaternion, NativeVector3}, native_utils, scripted_app::{ScriptedApp, APP}
};
use crate::c_lang::CCompatibleLang;

#[no_mangle]
///@owner_class FyroxCApi
pub extern "C" fn init_fyrox(app: NativeScriptedApp) {
    APP.set(Some(ScriptedApp::from_native(app)));
}

/// identifier of some entity allocated on scripting side and managed by scripting engine.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeInstanceId {
    pub value: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeClassId {
    pub value: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeScriptProperty {
    pub name: *const c_char,
    pub ty: NativeValueType,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeValueType {
    bool,
    f32,
    f64,
    i16,
    i32,
    i64,
    String,
    Node,
    UiNode,
    Prefab,
    Vector3,
    Quaternion,
}

native_utils!(u8, u8_array, u8_option, u8_result);
native_utils!(bool, bool_array, bool_option, bool_result);
native_utils!(f32, f32_array, f32_option, f32_result);
native_utils!(f64, f64_array, f64_option, f64_result);
native_utils!(i16, i16_array, i16_option, i16_result);
native_utils!(i32, i32_array, i32_option, i32_result);
native_utils!(i64, i64_array, i64_option, i64_result);
native_utils!(
    u8_array,
    NativeString_array,
    NativeString_option,
    NativeString_result
);
native_utils!(
    NativeHandle,
    NativeHandle_array,
    NativeHandle_option,
    NativeHandle_result
);
native_utils!(
    NativeInstanceId,
    NativeInstanceId_array,
    NativeInstanceId_option,
    NativeInstanceId_result
);
native_utils!(
    NativeInstanceId_option,
    NativeInstanceId_option_array,
    NativeInstanceId_option_option,
    NativeInstanceId_option_result
);
native_utils!(
    NativeHandle_option,
    NativeHandle_option_array,
    NativeHandle_option_option,
    NativeHandle_option_result
);

pub type NativeString = <u8 as NativeType>::Array;

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeValue {
    pub bool: bool,
    pub f32: f32,
    pub f64: f64,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub String: NativeString,
    /// Node and their derivatives. also, Resource passed as the handles in the specially allocated pool of Resource. because Fyrox Resource is not portable itself.
    pub Handle: NativeHandle,
    pub Vector3: NativeVector3,
    pub Quaternion: NativeQuaternion,
}

pub trait NativeType: Sized {
    type Array;
    type Option;
    type Result;

    fn to_native_array(v: Vec<Self>) -> Self::Array;
    fn from_native_array(v: Self::Array) -> Vec<Self>;

    fn to_native_option(v: Option<Self>) -> Self::Option;
    fn from_native_option(v: Self::Option) -> Option<Self>;

    fn to_native_result<E: Display>(v: Result<Self, E>) -> Self::Result;
    fn from_native_result<U: UserScript>(v: Self::Result) -> Result<Self, U::LangSpecificError>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NativeHandle {
    pub high: u64,
    pub low: u64,
}

impl NativeHandle {
    pub fn from_u128(value: u128) -> Self {
        Self {
            high: (value >> 64) as u64,
            low: value as u64,
        }
    }

    pub fn as_u128(&self) -> u128 {
        (self.high as u128) << 64 | (self.low as u128)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeScriptMetadata {
    pub id: NativeClassId,
    pub uuid: *mut c_char,
    pub kind: NativeScriptKind,
    pub name: *mut c_char,
    /// null (the same as empty for zero-terminated) if no parent
    pub parent: *const c_char,
    pub has_on_init: bool,
    pub has_on_start: bool,
    pub has_on_deinit: bool,
    pub has_on_os_event: bool,
    pub has_on_update: bool,
    pub has_on_message: bool,
    pub properties: *mut NativeScriptProperty,
    pub properties_len: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NativeScriptKind {
    Node,
    Global,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeScriptedApp {
    pub scripts: *const NativeScriptMetadata,
    pub scripts_len: u16,
    pub functions: NativeScriptAppFunctions,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeScriptAppFunctions {
    pub on_init: NodeOnInit,
    pub on_start: NodeOnStart,
    pub on_deinit: NodeOnDeinit,
    pub on_os_event: NodeOnOsEvent,
    pub on_update: NodeOnUpdate,
    pub on_message: NodeOnMessage,

    pub on_game_init: GameOnInit,
    pub on_game_update: GameOnUpdate,
    pub on_game_on_os_event: GameOnOsEvent,
    pub create_script_instance: CreateScriptInstance,
    pub set_property: SetProperty,
}

pub type NodeOnUpdate = extern "C" fn(thiz: NativeInstanceId, dt: f32);
pub type NodeOnInit = extern "C" fn(thiz: NativeInstanceId);
pub type NodeOnDeinit = extern "C" fn(thiz: NativeInstanceId);
pub type NodeOnStart = extern "C" fn(thiz: NativeInstanceId);
pub type NodeOnOsEvent = extern "C" fn(thiz: NativeInstanceId);
pub type NodeOnMessage = extern "C" fn(thiz: NativeInstanceId);

pub type GameOnInit = extern "C" fn(thiz: NativeInstanceId);
pub type GameOnUpdate = extern "C" fn(thiz: NativeInstanceId);
pub type GameOnOsEvent = extern "C" fn(thiz: NativeInstanceId);

pub type CreateScriptInstance = extern "C" fn(thiz: NativeClassId) -> NativeInstanceId;
pub type SetProperty = extern "C" fn(thiz: NativeInstanceId, property: u16, value: NativeValue);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserScriptMessage {
    pub id: i64,
}

impl LiteDataType for UserScriptMessage {}