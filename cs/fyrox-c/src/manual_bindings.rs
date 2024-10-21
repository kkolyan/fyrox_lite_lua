use std::ffi::{c_char, CString};

use crate::scripted_app::{ScriptedApp, APP};

#[no_mangle]
///@owner_class FyroxCApi
pub extern "C" fn init_fyrox(app: NativeScriptedApp) {
    APP.set(Some(ScriptedApp::from_native(app)));
}

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

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeValue {
    pub bool: bool,
    pub f32: f32,
    pub f64: f64,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    /// pointer, because CString is not Copy
    pub String: *const c_char,
    /// Node and their derivatives. also, Resource passed as the handles in the specially allocated pool of Resource. because Fyrox Resource is not portable itself.
    pub Handle: NativeHandle,
    pub Vector3: NativeVector3,
    pub Quaternion: NativeQuaternion,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeHandle {
    pub high: u64,
    pub low: u64,
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

pub type NodeOnUpdate = extern fn(thiz: NativeInstanceId, dt: f32);
pub type NodeOnInit= extern fn(thiz: NativeInstanceId);
pub type NodeOnDeinit= extern fn(thiz: NativeInstanceId);
pub type NodeOnStart= extern fn(thiz: NativeInstanceId);
pub type NodeOnOsEvent= extern fn(thiz: NativeInstanceId);
pub type NodeOnMessage= extern fn(thiz: NativeInstanceId);

pub type GameOnInit = extern fn(thiz: NativeInstanceId);
pub type GameOnUpdate = extern fn(thiz: NativeInstanceId);
pub type GameOnOsEvent = extern fn(thiz: NativeInstanceId);

pub type CreateScriptInstance = extern fn(thiz: NativeClassId) -> NativeInstanceId;
pub type SetProperty = extern fn(thiz: NativeInstanceId, property: u16, value: NativeValue);
