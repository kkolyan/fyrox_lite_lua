use std::ffi::{c_char, CString};

use crate::scripted_app::{ScriptedApp, APP};

#[no_mangle]
pub extern "C" fn ScriptUpdate() {
    println!("I'm a Fyrox C. 001")
}

#[no_mangle]
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
    pub Handle: u128,
    pub Vector3: [f32; 3],
    pub Quaternion: [f32; 4],
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
    pub on_init: fn(this: NativeInstanceId),
    pub on_start: fn(this: NativeInstanceId),
    pub on_deinit: fn(this: NativeInstanceId),
    pub on_os_event: fn(this: NativeInstanceId),
    pub on_update: fn(this: NativeInstanceId, dt: f32),
    pub on_message: fn(this: NativeInstanceId, message: NativeInstanceId),

    pub on_game_init: fn(this: NativeInstanceId),
    pub on_game_update: fn(this: NativeInstanceId),
    pub on_game_on_os_event: fn(this: NativeInstanceId),
    pub create_script_instance: fn(NativeClassId) -> NativeInstanceId,
    pub set_property: fn(this: NativeInstanceId, value: NativeValue),
}
