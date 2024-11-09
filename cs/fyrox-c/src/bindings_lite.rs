
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_locals)]
use crate::bindings_manual::*;
use crate::native_utils;
use fyrox_lite::externalizable::Externalizable;
use fyrox_lite::spi::UserScript;
use std::fmt::Display;

// fyrox_lite::lite_prefab::LitePrefab

pub extern "C" fn fyrox_lite_Prefab_instantiate_at(
    __this: NativeHandle,
    position: NativeVector3,
    orientation: NativeQuaternion,
) -> NativeHandle {
    let mut __this: fyrox_lite::lite_prefab::LitePrefab =
        Externalizable::from_external(__this.as_u128());
    let mut position = position.into();
    let mut orientation = orientation.into();
    let __result = __this.instantiate_at(position, orientation);
    let __result = NativeHandle::from_u128(Externalizable::to_external(&__result));
    __result
}

// fyrox_lite::lite_log::LiteLog

pub extern "C" fn fyrox_lite_Log_info(msg: NativeString) -> () {
    let mut msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::info(msg);
    __result
}
pub extern "C" fn fyrox_lite_Log_warn(msg: NativeString) -> () {
    let mut msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::warn(msg);
    __result
}
pub extern "C" fn fyrox_lite_Log_err(msg: NativeString) -> () {
    let mut msg = String::from_utf8(<u8 as NativeType>::from_native_array(msg)).unwrap();
    let __result = fyrox_lite::lite_log::LiteLog::err(msg);
    __result
}

// fyrox_lite::lite_window::LiteWindow

pub extern "C" fn fyrox_lite_Window_set_cursor_grab(mode: NativeCursorGrabMode) -> () {
    let mut mode = mode.into();
    let __result = fyrox_lite::lite_window::LiteWindow::set_cursor_grab(mode);
    __result
}

// fyrox_lite::lite_window::LiteCursorGrabMode

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeCursorGrabMode {
    pub tag: u8,
}
native_utils!(
    NativeCursorGrabMode,
    NativeCursorGrabMode_array,
    NativeCursorGrabMode_option,
    NativeCursorGrabMode_result
);
impl From<NativeCursorGrabMode> for fyrox_lite::lite_window::LiteCursorGrabMode {
    fn from(__value: NativeCursorGrabMode) -> Self {
        if __value.tag == NativeCursorGrabMode::None {
            return Self::None;
        }
        if __value.tag == NativeCursorGrabMode::Confined {
            return Self::Confined;
        }
        if __value.tag == NativeCursorGrabMode::Locked {
            return Self::Locked;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_window::LiteCursorGrabMode> for NativeCursorGrabMode {
    fn from(__value: fyrox_lite::lite_window::LiteCursorGrabMode) -> Self {
        if let fyrox_lite::lite_window::LiteCursorGrabMode::None = __value {
            return NativeCursorGrabMode {
                tag: NativeCursorGrabMode::None,
            };
        }
        if let fyrox_lite::lite_window::LiteCursorGrabMode::Confined = __value {
            return NativeCursorGrabMode {
                tag: NativeCursorGrabMode::Confined,
            };
        }
        if let fyrox_lite::lite_window::LiteCursorGrabMode::Locked = __value {
            return NativeCursorGrabMode {
                tag: NativeCursorGrabMode::Locked,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeCursorGrabMode {
    pub const None: u8 = 0;
    pub const Confined: u8 = 1;
    pub const Locked: u8 = 2;
}

// fyrox_lite::lite_scene::LiteScene

pub extern "C" fn fyrox_lite_Scene_load_async(scene_path: NativeString) -> () {
    let mut scene_path =
        String::from_utf8(<u8 as NativeType>::from_native_array(scene_path)).unwrap();
    let __result = fyrox_lite::lite_scene::LiteScene::load_async(scene_path);
    __result
}

// fyrox_lite::lite_physics::LitePhysics

pub extern "C" fn fyrox_lite_Physics_cast_ray(
    opts: NativeRayCastOptions,
) -> NativeIntersection_array {
    let mut opts = opts.into();
    let __result = fyrox_lite::lite_physics::LitePhysics::cast_ray(opts);
    let __result = <NativeIntersection as NativeType>::to_native_array(
        __result
            .into_iter()
            .map(|__result| __result.into())
            .collect::<Vec<_>>(),
    );
    __result
}

// fyrox_lite::lite_physics::LiteIntersection

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeIntersection {
    pub collider: NativeHandle,
    pub normal: NativeVector3,
    pub position: NativeVector3,
    pub feature: NativeFeatureId,
    pub toi: f32,
}
native_utils!(
    NativeIntersection,
    NativeIntersection_array,
    NativeIntersection_option,
    NativeIntersection_result
);
impl From<NativeIntersection> for fyrox_lite::lite_physics::LiteIntersection {
    fn from(__value: NativeIntersection) -> Self {
        let collider = __value.collider;
        let collider = Externalizable::from_external(collider.as_u128());
        let normal = __value.normal;
        let normal = normal.into();
        let position = __value.position;
        let position = position.into();
        let feature = __value.feature;
        let feature = feature.into();
        let toi = __value.toi;
        let toi = toi;
        Self {
            collider,
            normal,
            position,
            feature,
            toi,
        }
    }
}
impl From<fyrox_lite::lite_physics::LiteIntersection> for NativeIntersection {
    fn from(__value: fyrox_lite::lite_physics::LiteIntersection) -> Self {
        let collider = __value.collider;
        let collider = NativeHandle::from_u128(Externalizable::to_external(&collider));
        let normal = __value.normal;
        let normal = normal.into();
        let position = __value.position;
        let position = position.into();
        let feature = __value.feature;
        let feature = feature.into();
        let toi = __value.toi;
        let toi = toi;
        Self {
            collider,
            normal,
            position,
            feature,
            toi,
        }
    }
}

// fyrox_lite::lite_physics::LiteFeatureId

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId {
    pub kind: NativeFeatureKind,
    pub id: i32,
}
native_utils!(
    NativeFeatureId,
    NativeFeatureId_array,
    NativeFeatureId_option,
    NativeFeatureId_result
);
impl From<NativeFeatureId> for fyrox_lite::lite_physics::LiteFeatureId {
    fn from(__value: NativeFeatureId) -> Self {
        let kind = __value.kind;
        let kind = kind.into();
        let id = __value.id;
        let id = id;
        Self { kind, id }
    }
}
impl From<fyrox_lite::lite_physics::LiteFeatureId> for NativeFeatureId {
    fn from(__value: fyrox_lite::lite_physics::LiteFeatureId) -> Self {
        let kind = __value.kind;
        let kind = kind.into();
        let id = __value.id;
        let id = id;
        Self { kind, id }
    }
}

// fyrox_lite::lite_physics::LiteFeatureKind

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureKind {
    pub tag: u8,
}
native_utils!(
    NativeFeatureKind,
    NativeFeatureKind_array,
    NativeFeatureKind_option,
    NativeFeatureKind_result
);
impl From<NativeFeatureKind> for fyrox_lite::lite_physics::LiteFeatureKind {
    fn from(__value: NativeFeatureKind) -> Self {
        if __value.tag == NativeFeatureKind::Vertex {
            return Self::Vertex;
        }
        if __value.tag == NativeFeatureKind::Edge {
            return Self::Edge;
        }
        if __value.tag == NativeFeatureKind::Face {
            return Self::Face;
        }
        if __value.tag == NativeFeatureKind::Unknown {
            return Self::Unknown;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_physics::LiteFeatureKind> for NativeFeatureKind {
    fn from(__value: fyrox_lite::lite_physics::LiteFeatureKind) -> Self {
        if let fyrox_lite::lite_physics::LiteFeatureKind::Vertex = __value {
            return NativeFeatureKind {
                tag: NativeFeatureKind::Vertex,
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureKind::Edge = __value {
            return NativeFeatureKind {
                tag: NativeFeatureKind::Edge,
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureKind::Face = __value {
            return NativeFeatureKind {
                tag: NativeFeatureKind::Face,
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureKind::Unknown = __value {
            return NativeFeatureKind {
                tag: NativeFeatureKind::Unknown,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeFeatureKind {
    pub const Vertex: u8 = 0;
    pub const Edge: u8 = 1;
    pub const Face: u8 = 2;
    pub const Unknown: u8 = 3;
}

// fyrox_lite::lite_physics::LiteRayCastOptions

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRayCastOptions {
    pub ray_origin: NativeVector3,
    pub ray_direction: NativeVector3,
    pub max_len: f32,
    pub groups: NativeInteractionGroups_option,
    pub sort_results: bool,
}
native_utils!(
    NativeRayCastOptions,
    NativeRayCastOptions_array,
    NativeRayCastOptions_option,
    NativeRayCastOptions_result
);
impl From<NativeRayCastOptions> for fyrox_lite::lite_physics::LiteRayCastOptions {
    fn from(__value: NativeRayCastOptions) -> Self {
        let ray_origin = __value.ray_origin;
        let ray_origin = ray_origin.into();
        let ray_direction = __value.ray_direction;
        let ray_direction = ray_direction.into();
        let max_len = __value.max_len;
        let max_len = max_len;
        let groups = __value.groups;
        let groups = if groups.present {
            Some({
                let groups = groups.value;
                groups.into()
            })
        } else {
            None
        };
        let sort_results = __value.sort_results;
        let sort_results = sort_results;
        Self {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }
    }
}
impl From<fyrox_lite::lite_physics::LiteRayCastOptions> for NativeRayCastOptions {
    fn from(__value: fyrox_lite::lite_physics::LiteRayCastOptions) -> Self {
        let ray_origin = __value.ray_origin;
        let ray_origin = ray_origin.into();
        let ray_direction = __value.ray_direction;
        let ray_direction = ray_direction.into();
        let max_len = __value.max_len;
        let max_len = max_len;
        let groups = __value.groups;
        let groups = <NativeInteractionGroups as NativeType>::to_native_option(
            if let Some(groups) = groups {
                Some(groups.into())
            } else {
                None
            },
        );
        let sort_results = __value.sort_results;
        let sort_results = sort_results;
        Self {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }
    }
}

// fyrox_lite::lite_physics::LiteInteractionGroups

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInteractionGroups {
    pub memberships: i32,
    pub filter: i32,
}
native_utils!(
    NativeInteractionGroups,
    NativeInteractionGroups_array,
    NativeInteractionGroups_option,
    NativeInteractionGroups_result
);
impl From<NativeInteractionGroups> for fyrox_lite::lite_physics::LiteInteractionGroups {
    fn from(__value: NativeInteractionGroups) -> Self {
        let memberships = __value.memberships;
        let memberships = memberships;
        let filter = __value.filter;
        let filter = filter;
        Self {
            memberships,
            filter,
        }
    }
}
impl From<fyrox_lite::lite_physics::LiteInteractionGroups> for NativeInteractionGroups {
    fn from(__value: fyrox_lite::lite_physics::LiteInteractionGroups) -> Self {
        let memberships = __value.memberships;
        let memberships = memberships;
        let filter = __value.filter;
        let filter = filter;
        Self {
            memberships,
            filter,
        }
    }
}

// fyrox_lite::lite_physics::LiteRigidBody

pub extern "C" fn fyrox_lite_RigidBody_apply_force(
    __this: NativeHandle,
    force: NativeVector3,
) -> () {
    let mut __this: fyrox_lite::lite_physics::LiteRigidBody =
        Externalizable::from_external(__this.as_u128());
    let mut force = force.into();
    let __result = __this.apply_force(force);
    __result
}

// fyrox_lite::lite_input::Input

pub extern "C" fn fyrox_lite_Input_is_mouse_button_down(button: i32) -> bool {
    let mut button = button;
    let __result = fyrox_lite::lite_input::Input::is_mouse_button_down(button);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_is_mouse_button_up(button: i32) -> bool {
    let mut button = button;
    let __result = fyrox_lite::lite_input::Input::is_mouse_button_up(button);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_is_mouse_button(button: i32) -> bool {
    let mut button = button;
    let __result = fyrox_lite::lite_input::Input::is_mouse_button_pressed(button);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_is_key_down(key: NativeKeyCode) -> bool {
    let mut key = key.into();
    let __result = fyrox_lite::lite_input::Input::is_key_down(key);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_is_key_up(key: NativeKeyCode) -> bool {
    let mut key = key.into();
    let __result = fyrox_lite::lite_input::Input::is_key_up(key);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_is_key(key: NativeKeyCode) -> bool {
    let mut key = key.into();
    let __result = fyrox_lite::lite_input::Input::is_key_pressed(key);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Input_get_mouse_move() -> NativeVector2 {
    let __result = fyrox_lite::lite_input::Input::get_mouse_move();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Input_get_mouse_scroll() -> NativeVector2 {
    let __result = fyrox_lite::lite_input::Input::get_mouse_scroll();
    let __result = __result.into();
    __result
}

// fyrox_lite::lite_input::LiteKeyCode

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeKeyCode {
    pub tag: u8,
}
native_utils!(
    NativeKeyCode,
    NativeKeyCode_array,
    NativeKeyCode_option,
    NativeKeyCode_result
);
impl From<NativeKeyCode> for fyrox_lite::lite_input::LiteKeyCode {
    fn from(__value: NativeKeyCode) -> Self {
        if __value.tag == NativeKeyCode::Backquote {
            return Self::Backquote;
        }
        if __value.tag == NativeKeyCode::Backslash {
            return Self::Backslash;
        }
        if __value.tag == NativeKeyCode::BracketLeft {
            return Self::BracketLeft;
        }
        if __value.tag == NativeKeyCode::BracketRight {
            return Self::BracketRight;
        }
        if __value.tag == NativeKeyCode::Comma {
            return Self::Comma;
        }
        if __value.tag == NativeKeyCode::Digit0 {
            return Self::Digit0;
        }
        if __value.tag == NativeKeyCode::Digit1 {
            return Self::Digit1;
        }
        if __value.tag == NativeKeyCode::Digit2 {
            return Self::Digit2;
        }
        if __value.tag == NativeKeyCode::Digit3 {
            return Self::Digit3;
        }
        if __value.tag == NativeKeyCode::Digit4 {
            return Self::Digit4;
        }
        if __value.tag == NativeKeyCode::Digit5 {
            return Self::Digit5;
        }
        if __value.tag == NativeKeyCode::Digit6 {
            return Self::Digit6;
        }
        if __value.tag == NativeKeyCode::Digit7 {
            return Self::Digit7;
        }
        if __value.tag == NativeKeyCode::Digit8 {
            return Self::Digit8;
        }
        if __value.tag == NativeKeyCode::Digit9 {
            return Self::Digit9;
        }
        if __value.tag == NativeKeyCode::Equal {
            return Self::Equal;
        }
        if __value.tag == NativeKeyCode::IntlBackslash {
            return Self::IntlBackslash;
        }
        if __value.tag == NativeKeyCode::IntlRo {
            return Self::IntlRo;
        }
        if __value.tag == NativeKeyCode::IntlYen {
            return Self::IntlYen;
        }
        if __value.tag == NativeKeyCode::A {
            return Self::A;
        }
        if __value.tag == NativeKeyCode::B {
            return Self::B;
        }
        if __value.tag == NativeKeyCode::C {
            return Self::C;
        }
        if __value.tag == NativeKeyCode::D {
            return Self::D;
        }
        if __value.tag == NativeKeyCode::E {
            return Self::E;
        }
        if __value.tag == NativeKeyCode::F {
            return Self::F;
        }
        if __value.tag == NativeKeyCode::G {
            return Self::G;
        }
        if __value.tag == NativeKeyCode::H {
            return Self::H;
        }
        if __value.tag == NativeKeyCode::I {
            return Self::I;
        }
        if __value.tag == NativeKeyCode::J {
            return Self::J;
        }
        if __value.tag == NativeKeyCode::K {
            return Self::K;
        }
        if __value.tag == NativeKeyCode::L {
            return Self::L;
        }
        if __value.tag == NativeKeyCode::M {
            return Self::M;
        }
        if __value.tag == NativeKeyCode::N {
            return Self::N;
        }
        if __value.tag == NativeKeyCode::O {
            return Self::O;
        }
        if __value.tag == NativeKeyCode::P {
            return Self::P;
        }
        if __value.tag == NativeKeyCode::Q {
            return Self::Q;
        }
        if __value.tag == NativeKeyCode::R {
            return Self::R;
        }
        if __value.tag == NativeKeyCode::S {
            return Self::S;
        }
        if __value.tag == NativeKeyCode::T {
            return Self::T;
        }
        if __value.tag == NativeKeyCode::U {
            return Self::U;
        }
        if __value.tag == NativeKeyCode::V {
            return Self::V;
        }
        if __value.tag == NativeKeyCode::W {
            return Self::W;
        }
        if __value.tag == NativeKeyCode::X {
            return Self::X;
        }
        if __value.tag == NativeKeyCode::Y {
            return Self::Y;
        }
        if __value.tag == NativeKeyCode::Z {
            return Self::Z;
        }
        if __value.tag == NativeKeyCode::Minus {
            return Self::Minus;
        }
        if __value.tag == NativeKeyCode::Period {
            return Self::Period;
        }
        if __value.tag == NativeKeyCode::Quote {
            return Self::Quote;
        }
        if __value.tag == NativeKeyCode::Semicolon {
            return Self::Semicolon;
        }
        if __value.tag == NativeKeyCode::Slash {
            return Self::Slash;
        }
        if __value.tag == NativeKeyCode::AltLeft {
            return Self::AltLeft;
        }
        if __value.tag == NativeKeyCode::AltRight {
            return Self::AltRight;
        }
        if __value.tag == NativeKeyCode::Backspace {
            return Self::Backspace;
        }
        if __value.tag == NativeKeyCode::CapsLock {
            return Self::CapsLock;
        }
        if __value.tag == NativeKeyCode::ContextMenu {
            return Self::ContextMenu;
        }
        if __value.tag == NativeKeyCode::ControlLeft {
            return Self::ControlLeft;
        }
        if __value.tag == NativeKeyCode::ControlRight {
            return Self::ControlRight;
        }
        if __value.tag == NativeKeyCode::Enter {
            return Self::Enter;
        }
        if __value.tag == NativeKeyCode::SuperLeft {
            return Self::SuperLeft;
        }
        if __value.tag == NativeKeyCode::SuperRight {
            return Self::SuperRight;
        }
        if __value.tag == NativeKeyCode::ShiftLeft {
            return Self::ShiftLeft;
        }
        if __value.tag == NativeKeyCode::ShiftRight {
            return Self::ShiftRight;
        }
        if __value.tag == NativeKeyCode::Space {
            return Self::Space;
        }
        if __value.tag == NativeKeyCode::Tab {
            return Self::Tab;
        }
        if __value.tag == NativeKeyCode::Convert {
            return Self::Convert;
        }
        if __value.tag == NativeKeyCode::KanaMode {
            return Self::KanaMode;
        }
        if __value.tag == NativeKeyCode::Lang1 {
            return Self::Lang1;
        }
        if __value.tag == NativeKeyCode::Lang2 {
            return Self::Lang2;
        }
        if __value.tag == NativeKeyCode::Lang3 {
            return Self::Lang3;
        }
        if __value.tag == NativeKeyCode::Lang4 {
            return Self::Lang4;
        }
        if __value.tag == NativeKeyCode::Lang5 {
            return Self::Lang5;
        }
        if __value.tag == NativeKeyCode::NonConvert {
            return Self::NonConvert;
        }
        if __value.tag == NativeKeyCode::Delete {
            return Self::Delete;
        }
        if __value.tag == NativeKeyCode::End {
            return Self::End;
        }
        if __value.tag == NativeKeyCode::Help {
            return Self::Help;
        }
        if __value.tag == NativeKeyCode::Home {
            return Self::Home;
        }
        if __value.tag == NativeKeyCode::Insert {
            return Self::Insert;
        }
        if __value.tag == NativeKeyCode::PageDown {
            return Self::PageDown;
        }
        if __value.tag == NativeKeyCode::PageUp {
            return Self::PageUp;
        }
        if __value.tag == NativeKeyCode::ArrowDown {
            return Self::ArrowDown;
        }
        if __value.tag == NativeKeyCode::ArrowLeft {
            return Self::ArrowLeft;
        }
        if __value.tag == NativeKeyCode::ArrowRight {
            return Self::ArrowRight;
        }
        if __value.tag == NativeKeyCode::ArrowUp {
            return Self::ArrowUp;
        }
        if __value.tag == NativeKeyCode::NumLock {
            return Self::NumLock;
        }
        if __value.tag == NativeKeyCode::Numpad0 {
            return Self::Numpad0;
        }
        if __value.tag == NativeKeyCode::Numpad1 {
            return Self::Numpad1;
        }
        if __value.tag == NativeKeyCode::Numpad2 {
            return Self::Numpad2;
        }
        if __value.tag == NativeKeyCode::Numpad3 {
            return Self::Numpad3;
        }
        if __value.tag == NativeKeyCode::Numpad4 {
            return Self::Numpad4;
        }
        if __value.tag == NativeKeyCode::Numpad5 {
            return Self::Numpad5;
        }
        if __value.tag == NativeKeyCode::Numpad6 {
            return Self::Numpad6;
        }
        if __value.tag == NativeKeyCode::Numpad7 {
            return Self::Numpad7;
        }
        if __value.tag == NativeKeyCode::Numpad8 {
            return Self::Numpad8;
        }
        if __value.tag == NativeKeyCode::Numpad9 {
            return Self::Numpad9;
        }
        if __value.tag == NativeKeyCode::NumpadAdd {
            return Self::NumpadAdd;
        }
        if __value.tag == NativeKeyCode::NumpadBackspace {
            return Self::NumpadBackspace;
        }
        if __value.tag == NativeKeyCode::NumpadClear {
            return Self::NumpadClear;
        }
        if __value.tag == NativeKeyCode::NumpadClearEntry {
            return Self::NumpadClearEntry;
        }
        if __value.tag == NativeKeyCode::NumpadComma {
            return Self::NumpadComma;
        }
        if __value.tag == NativeKeyCode::NumpadDecimal {
            return Self::NumpadDecimal;
        }
        if __value.tag == NativeKeyCode::NumpadDivide {
            return Self::NumpadDivide;
        }
        if __value.tag == NativeKeyCode::NumpadEnter {
            return Self::NumpadEnter;
        }
        if __value.tag == NativeKeyCode::NumpadEqual {
            return Self::NumpadEqual;
        }
        if __value.tag == NativeKeyCode::NumpadHash {
            return Self::NumpadHash;
        }
        if __value.tag == NativeKeyCode::NumpadMemoryAdd {
            return Self::NumpadMemoryAdd;
        }
        if __value.tag == NativeKeyCode::NumpadMemoryClear {
            return Self::NumpadMemoryClear;
        }
        if __value.tag == NativeKeyCode::NumpadMemoryRecall {
            return Self::NumpadMemoryRecall;
        }
        if __value.tag == NativeKeyCode::NumpadMemoryStore {
            return Self::NumpadMemoryStore;
        }
        if __value.tag == NativeKeyCode::NumpadMemorySubtract {
            return Self::NumpadMemorySubtract;
        }
        if __value.tag == NativeKeyCode::NumpadMultiply {
            return Self::NumpadMultiply;
        }
        if __value.tag == NativeKeyCode::NumpadParenLeft {
            return Self::NumpadParenLeft;
        }
        if __value.tag == NativeKeyCode::NumpadParenRight {
            return Self::NumpadParenRight;
        }
        if __value.tag == NativeKeyCode::NumpadStar {
            return Self::NumpadStar;
        }
        if __value.tag == NativeKeyCode::NumpadSubtract {
            return Self::NumpadSubtract;
        }
        if __value.tag == NativeKeyCode::Escape {
            return Self::Escape;
        }
        if __value.tag == NativeKeyCode::Fn {
            return Self::Fn;
        }
        if __value.tag == NativeKeyCode::FnLock {
            return Self::FnLock;
        }
        if __value.tag == NativeKeyCode::PrintScreen {
            return Self::PrintScreen;
        }
        if __value.tag == NativeKeyCode::ScrollLock {
            return Self::ScrollLock;
        }
        if __value.tag == NativeKeyCode::Pause {
            return Self::Pause;
        }
        if __value.tag == NativeKeyCode::BrowserBack {
            return Self::BrowserBack;
        }
        if __value.tag == NativeKeyCode::BrowserFavorites {
            return Self::BrowserFavorites;
        }
        if __value.tag == NativeKeyCode::BrowserForward {
            return Self::BrowserForward;
        }
        if __value.tag == NativeKeyCode::BrowserHome {
            return Self::BrowserHome;
        }
        if __value.tag == NativeKeyCode::BrowserRefresh {
            return Self::BrowserRefresh;
        }
        if __value.tag == NativeKeyCode::BrowserSearch {
            return Self::BrowserSearch;
        }
        if __value.tag == NativeKeyCode::BrowserStop {
            return Self::BrowserStop;
        }
        if __value.tag == NativeKeyCode::Eject {
            return Self::Eject;
        }
        if __value.tag == NativeKeyCode::LaunchApp1 {
            return Self::LaunchApp1;
        }
        if __value.tag == NativeKeyCode::LaunchApp2 {
            return Self::LaunchApp2;
        }
        if __value.tag == NativeKeyCode::LaunchMail {
            return Self::LaunchMail;
        }
        if __value.tag == NativeKeyCode::MediaPlayPause {
            return Self::MediaPlayPause;
        }
        if __value.tag == NativeKeyCode::MediaSelect {
            return Self::MediaSelect;
        }
        if __value.tag == NativeKeyCode::MediaStop {
            return Self::MediaStop;
        }
        if __value.tag == NativeKeyCode::MediaTrackNext {
            return Self::MediaTrackNext;
        }
        if __value.tag == NativeKeyCode::MediaTrackPrevious {
            return Self::MediaTrackPrevious;
        }
        if __value.tag == NativeKeyCode::Power {
            return Self::Power;
        }
        if __value.tag == NativeKeyCode::Sleep {
            return Self::Sleep;
        }
        if __value.tag == NativeKeyCode::AudioVolumeDown {
            return Self::AudioVolumeDown;
        }
        if __value.tag == NativeKeyCode::AudioVolumeMute {
            return Self::AudioVolumeMute;
        }
        if __value.tag == NativeKeyCode::AudioVolumeUp {
            return Self::AudioVolumeUp;
        }
        if __value.tag == NativeKeyCode::WakeUp {
            return Self::WakeUp;
        }
        if __value.tag == NativeKeyCode::Meta {
            return Self::Meta;
        }
        if __value.tag == NativeKeyCode::Hyper {
            return Self::Hyper;
        }
        if __value.tag == NativeKeyCode::Turbo {
            return Self::Turbo;
        }
        if __value.tag == NativeKeyCode::Abort {
            return Self::Abort;
        }
        if __value.tag == NativeKeyCode::Resume {
            return Self::Resume;
        }
        if __value.tag == NativeKeyCode::Suspend {
            return Self::Suspend;
        }
        if __value.tag == NativeKeyCode::Again {
            return Self::Again;
        }
        if __value.tag == NativeKeyCode::Copy {
            return Self::Copy;
        }
        if __value.tag == NativeKeyCode::Cut {
            return Self::Cut;
        }
        if __value.tag == NativeKeyCode::Find {
            return Self::Find;
        }
        if __value.tag == NativeKeyCode::Open {
            return Self::Open;
        }
        if __value.tag == NativeKeyCode::Paste {
            return Self::Paste;
        }
        if __value.tag == NativeKeyCode::Props {
            return Self::Props;
        }
        if __value.tag == NativeKeyCode::Select {
            return Self::Select;
        }
        if __value.tag == NativeKeyCode::Undo {
            return Self::Undo;
        }
        if __value.tag == NativeKeyCode::Hiragana {
            return Self::Hiragana;
        }
        if __value.tag == NativeKeyCode::Katakana {
            return Self::Katakana;
        }
        if __value.tag == NativeKeyCode::F1 {
            return Self::F1;
        }
        if __value.tag == NativeKeyCode::F2 {
            return Self::F2;
        }
        if __value.tag == NativeKeyCode::F3 {
            return Self::F3;
        }
        if __value.tag == NativeKeyCode::F4 {
            return Self::F4;
        }
        if __value.tag == NativeKeyCode::F5 {
            return Self::F5;
        }
        if __value.tag == NativeKeyCode::F6 {
            return Self::F6;
        }
        if __value.tag == NativeKeyCode::F7 {
            return Self::F7;
        }
        if __value.tag == NativeKeyCode::F8 {
            return Self::F8;
        }
        if __value.tag == NativeKeyCode::F9 {
            return Self::F9;
        }
        if __value.tag == NativeKeyCode::F10 {
            return Self::F10;
        }
        if __value.tag == NativeKeyCode::F11 {
            return Self::F11;
        }
        if __value.tag == NativeKeyCode::F12 {
            return Self::F12;
        }
        if __value.tag == NativeKeyCode::F13 {
            return Self::F13;
        }
        if __value.tag == NativeKeyCode::F14 {
            return Self::F14;
        }
        if __value.tag == NativeKeyCode::F15 {
            return Self::F15;
        }
        if __value.tag == NativeKeyCode::F16 {
            return Self::F16;
        }
        if __value.tag == NativeKeyCode::F17 {
            return Self::F17;
        }
        if __value.tag == NativeKeyCode::F18 {
            return Self::F18;
        }
        if __value.tag == NativeKeyCode::F19 {
            return Self::F19;
        }
        if __value.tag == NativeKeyCode::F20 {
            return Self::F20;
        }
        if __value.tag == NativeKeyCode::F21 {
            return Self::F21;
        }
        if __value.tag == NativeKeyCode::F22 {
            return Self::F22;
        }
        if __value.tag == NativeKeyCode::F23 {
            return Self::F23;
        }
        if __value.tag == NativeKeyCode::F24 {
            return Self::F24;
        }
        if __value.tag == NativeKeyCode::F25 {
            return Self::F25;
        }
        if __value.tag == NativeKeyCode::F26 {
            return Self::F26;
        }
        if __value.tag == NativeKeyCode::F27 {
            return Self::F27;
        }
        if __value.tag == NativeKeyCode::F28 {
            return Self::F28;
        }
        if __value.tag == NativeKeyCode::F29 {
            return Self::F29;
        }
        if __value.tag == NativeKeyCode::F30 {
            return Self::F30;
        }
        if __value.tag == NativeKeyCode::F31 {
            return Self::F31;
        }
        if __value.tag == NativeKeyCode::F32 {
            return Self::F32;
        }
        if __value.tag == NativeKeyCode::F33 {
            return Self::F33;
        }
        if __value.tag == NativeKeyCode::F34 {
            return Self::F34;
        }
        if __value.tag == NativeKeyCode::F35 {
            return Self::F35;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_input::LiteKeyCode> for NativeKeyCode {
    fn from(__value: fyrox_lite::lite_input::LiteKeyCode) -> Self {
        if let fyrox_lite::lite_input::LiteKeyCode::Backquote = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backquote,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Backslash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backslash,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BracketLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BracketLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BracketRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BracketRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Comma = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Comma,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit0 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit0,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit1,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit2,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit3,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit4,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit5,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit6,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit7,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit8,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Digit9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit9,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Equal = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Equal,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::IntlBackslash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlBackslash,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::IntlRo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlRo,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::IntlYen = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlYen,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::A = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::A,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::B = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::B,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::C = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::C,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::D = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::D,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::E = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::E,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::G = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::G,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::H = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::H,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::I = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::I,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::J = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::J,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::K = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::K,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::L = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::L,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::M = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::M,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::N = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::N,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::O = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::O,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::P = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::P,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Q = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Q,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::R = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::R,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::S = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::S,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::T = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::T,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::U = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::U,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::V = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::V,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::W = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::W,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::X = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::X,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Y = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Y,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Z = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Z,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Minus = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Minus,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Period = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Period,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Quote = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Quote,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Semicolon = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Semicolon,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Slash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Slash,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::AltLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AltLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::AltRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AltRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Backspace = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backspace,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::CapsLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::CapsLock,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ContextMenu = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ContextMenu,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ControlLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ControlLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ControlRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ControlRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Enter = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Enter,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::SuperLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::SuperLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::SuperRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::SuperRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ShiftLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ShiftLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ShiftRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ShiftRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Space = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Space,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Tab = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Tab,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Convert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Convert,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::KanaMode = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KanaMode,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Lang1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang1,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Lang2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang2,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Lang3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang3,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Lang4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang4,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Lang5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang5,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NonConvert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NonConvert,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Delete = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Delete,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::End = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::End,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Help = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Help,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Home = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Home,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Insert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Insert,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::PageDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PageDown,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::PageUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PageUp,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ArrowDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowDown,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ArrowLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ArrowRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ArrowUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowUp,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumLock,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad0 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad0,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad1,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad2,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad3,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad4,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad5,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad6,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad7,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad8,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Numpad9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad9,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadAdd = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadAdd,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadBackspace = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadBackspace,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadClear = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadClear,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadClearEntry = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadClearEntry,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadComma = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadComma,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadDecimal = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadDecimal,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadDivide = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadDivide,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadEnter = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadEnter,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadEqual = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadEqual,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadHash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadHash,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryAdd = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryAdd,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryClear = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryClear,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryRecall = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryRecall,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryStore = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryStore,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMemorySubtract = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemorySubtract,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadMultiply = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMultiply,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadParenLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadParenLeft,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadParenRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadParenRight,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadStar = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadStar,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::NumpadSubtract = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadSubtract,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Escape = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Escape,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Fn = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Fn,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::FnLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::FnLock,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::PrintScreen = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PrintScreen,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::ScrollLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ScrollLock,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Pause = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Pause,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserBack = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserBack,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserFavorites = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserFavorites,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserForward = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserForward,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserHome = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserHome,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserRefresh = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserRefresh,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserSearch = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserSearch,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::BrowserStop = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserStop,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Eject = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Eject,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::LaunchApp1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchApp1,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::LaunchApp2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchApp2,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::LaunchMail = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchMail,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::MediaPlayPause = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaPlayPause,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::MediaSelect = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaSelect,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::MediaStop = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaStop,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::MediaTrackNext = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaTrackNext,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::MediaTrackPrevious = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaTrackPrevious,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Power = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Power,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Sleep = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Sleep,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeDown,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeMute = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeMute,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeUp,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::WakeUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::WakeUp,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Meta = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Meta,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Hyper = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Hyper,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Turbo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Turbo,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Abort = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Abort,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Resume = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Resume,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Suspend = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Suspend,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Again = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Again,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Copy = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Copy,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Cut = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Cut,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Find = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Find,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Open = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Open,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Paste = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Paste,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Props = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Props,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Select = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Select,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Undo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Undo,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Hiragana = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Hiragana,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::Katakana = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Katakana,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F1,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F2,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F3,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F4,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F5,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F6,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F7,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F8,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F9,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F10 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F10,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F11 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F11,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F12 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F12,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F13 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F13,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F14 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F14,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F15 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F15,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F16 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F16,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F17 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F17,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F18 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F18,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F19 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F19,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F20 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F20,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F21 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F21,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F22 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F22,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F23 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F23,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F24 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F24,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F25 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F25,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F26 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F26,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F27 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F27,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F28 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F28,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F29 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F29,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F30 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F30,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F31 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F31,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F32 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F32,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F33 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F33,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F34 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F34,
            };
        }
        if let fyrox_lite::lite_input::LiteKeyCode::F35 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F35,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeKeyCode {
    pub const Backquote: u8 = 0;
    pub const Backslash: u8 = 1;
    pub const BracketLeft: u8 = 2;
    pub const BracketRight: u8 = 3;
    pub const Comma: u8 = 4;
    pub const Digit0: u8 = 5;
    pub const Digit1: u8 = 6;
    pub const Digit2: u8 = 7;
    pub const Digit3: u8 = 8;
    pub const Digit4: u8 = 9;
    pub const Digit5: u8 = 10;
    pub const Digit6: u8 = 11;
    pub const Digit7: u8 = 12;
    pub const Digit8: u8 = 13;
    pub const Digit9: u8 = 14;
    pub const Equal: u8 = 15;
    pub const IntlBackslash: u8 = 16;
    pub const IntlRo: u8 = 17;
    pub const IntlYen: u8 = 18;
    pub const A: u8 = 19;
    pub const B: u8 = 20;
    pub const C: u8 = 21;
    pub const D: u8 = 22;
    pub const E: u8 = 23;
    pub const F: u8 = 24;
    pub const G: u8 = 25;
    pub const H: u8 = 26;
    pub const I: u8 = 27;
    pub const J: u8 = 28;
    pub const K: u8 = 29;
    pub const L: u8 = 30;
    pub const M: u8 = 31;
    pub const N: u8 = 32;
    pub const O: u8 = 33;
    pub const P: u8 = 34;
    pub const Q: u8 = 35;
    pub const R: u8 = 36;
    pub const S: u8 = 37;
    pub const T: u8 = 38;
    pub const U: u8 = 39;
    pub const V: u8 = 40;
    pub const W: u8 = 41;
    pub const X: u8 = 42;
    pub const Y: u8 = 43;
    pub const Z: u8 = 44;
    pub const Minus: u8 = 45;
    pub const Period: u8 = 46;
    pub const Quote: u8 = 47;
    pub const Semicolon: u8 = 48;
    pub const Slash: u8 = 49;
    pub const AltLeft: u8 = 50;
    pub const AltRight: u8 = 51;
    pub const Backspace: u8 = 52;
    pub const CapsLock: u8 = 53;
    pub const ContextMenu: u8 = 54;
    pub const ControlLeft: u8 = 55;
    pub const ControlRight: u8 = 56;
    pub const Enter: u8 = 57;
    pub const SuperLeft: u8 = 58;
    pub const SuperRight: u8 = 59;
    pub const ShiftLeft: u8 = 60;
    pub const ShiftRight: u8 = 61;
    pub const Space: u8 = 62;
    pub const Tab: u8 = 63;
    pub const Convert: u8 = 64;
    pub const KanaMode: u8 = 65;
    pub const Lang1: u8 = 66;
    pub const Lang2: u8 = 67;
    pub const Lang3: u8 = 68;
    pub const Lang4: u8 = 69;
    pub const Lang5: u8 = 70;
    pub const NonConvert: u8 = 71;
    pub const Delete: u8 = 72;
    pub const End: u8 = 73;
    pub const Help: u8 = 74;
    pub const Home: u8 = 75;
    pub const Insert: u8 = 76;
    pub const PageDown: u8 = 77;
    pub const PageUp: u8 = 78;
    pub const ArrowDown: u8 = 79;
    pub const ArrowLeft: u8 = 80;
    pub const ArrowRight: u8 = 81;
    pub const ArrowUp: u8 = 82;
    pub const NumLock: u8 = 83;
    pub const Numpad0: u8 = 84;
    pub const Numpad1: u8 = 85;
    pub const Numpad2: u8 = 86;
    pub const Numpad3: u8 = 87;
    pub const Numpad4: u8 = 88;
    pub const Numpad5: u8 = 89;
    pub const Numpad6: u8 = 90;
    pub const Numpad7: u8 = 91;
    pub const Numpad8: u8 = 92;
    pub const Numpad9: u8 = 93;
    pub const NumpadAdd: u8 = 94;
    pub const NumpadBackspace: u8 = 95;
    pub const NumpadClear: u8 = 96;
    pub const NumpadClearEntry: u8 = 97;
    pub const NumpadComma: u8 = 98;
    pub const NumpadDecimal: u8 = 99;
    pub const NumpadDivide: u8 = 100;
    pub const NumpadEnter: u8 = 101;
    pub const NumpadEqual: u8 = 102;
    pub const NumpadHash: u8 = 103;
    pub const NumpadMemoryAdd: u8 = 104;
    pub const NumpadMemoryClear: u8 = 105;
    pub const NumpadMemoryRecall: u8 = 106;
    pub const NumpadMemoryStore: u8 = 107;
    pub const NumpadMemorySubtract: u8 = 108;
    pub const NumpadMultiply: u8 = 109;
    pub const NumpadParenLeft: u8 = 110;
    pub const NumpadParenRight: u8 = 111;
    pub const NumpadStar: u8 = 112;
    pub const NumpadSubtract: u8 = 113;
    pub const Escape: u8 = 114;
    pub const Fn: u8 = 115;
    pub const FnLock: u8 = 116;
    pub const PrintScreen: u8 = 117;
    pub const ScrollLock: u8 = 118;
    pub const Pause: u8 = 119;
    pub const BrowserBack: u8 = 120;
    pub const BrowserFavorites: u8 = 121;
    pub const BrowserForward: u8 = 122;
    pub const BrowserHome: u8 = 123;
    pub const BrowserRefresh: u8 = 124;
    pub const BrowserSearch: u8 = 125;
    pub const BrowserStop: u8 = 126;
    pub const Eject: u8 = 127;
    pub const LaunchApp1: u8 = 128;
    pub const LaunchApp2: u8 = 129;
    pub const LaunchMail: u8 = 130;
    pub const MediaPlayPause: u8 = 131;
    pub const MediaSelect: u8 = 132;
    pub const MediaStop: u8 = 133;
    pub const MediaTrackNext: u8 = 134;
    pub const MediaTrackPrevious: u8 = 135;
    pub const Power: u8 = 136;
    pub const Sleep: u8 = 137;
    pub const AudioVolumeDown: u8 = 138;
    pub const AudioVolumeMute: u8 = 139;
    pub const AudioVolumeUp: u8 = 140;
    pub const WakeUp: u8 = 141;
    pub const Meta: u8 = 142;
    pub const Hyper: u8 = 143;
    pub const Turbo: u8 = 144;
    pub const Abort: u8 = 145;
    pub const Resume: u8 = 146;
    pub const Suspend: u8 = 147;
    pub const Again: u8 = 148;
    pub const Copy: u8 = 149;
    pub const Cut: u8 = 150;
    pub const Find: u8 = 151;
    pub const Open: u8 = 152;
    pub const Paste: u8 = 153;
    pub const Props: u8 = 154;
    pub const Select: u8 = 155;
    pub const Undo: u8 = 156;
    pub const Hiragana: u8 = 157;
    pub const Katakana: u8 = 158;
    pub const F1: u8 = 159;
    pub const F2: u8 = 160;
    pub const F3: u8 = 161;
    pub const F4: u8 = 162;
    pub const F5: u8 = 163;
    pub const F6: u8 = 164;
    pub const F7: u8 = 165;
    pub const F8: u8 = 166;
    pub const F9: u8 = 167;
    pub const F10: u8 = 168;
    pub const F11: u8 = 169;
    pub const F12: u8 = 170;
    pub const F13: u8 = 171;
    pub const F14: u8 = 172;
    pub const F15: u8 = 173;
    pub const F16: u8 = 174;
    pub const F17: u8 = 175;
    pub const F18: u8 = 176;
    pub const F19: u8 = 177;
    pub const F20: u8 = 178;
    pub const F21: u8 = 179;
    pub const F22: u8 = 180;
    pub const F23: u8 = 181;
    pub const F24: u8 = 182;
    pub const F25: u8 = 183;
    pub const F26: u8 = 184;
    pub const F27: u8 = 185;
    pub const F28: u8 = 186;
    pub const F29: u8 = 187;
    pub const F30: u8 = 188;
    pub const F31: u8 = 189;
    pub const F32: u8 = 190;
    pub const F33: u8 = 191;
    pub const F34: u8 = 192;
    pub const F35: u8 = 193;
}

// fyrox_lite::lite_ui::LiteUiNode

// fyrox_lite::lite_ui::LiteText

pub extern "C" fn fyrox_lite_Text_set_text_async(__this: NativeHandle, text: NativeString) -> () {
    let mut __this: fyrox_lite::lite_ui::LiteText = Externalizable::from_external(__this.as_u128());
    let mut text = String::from_utf8(<u8 as NativeType>::from_native_array(text)).unwrap();
    let __result = __this.set_text_async(text);
    __result
}
pub extern "C" fn fyrox_lite_Text_new(state: NativeTextBuilder) -> NativeHandle {
    let mut state = state.into();
    let __result = fyrox_lite::lite_ui::LiteText::new(state);
    let __result = NativeHandle::from_u128(Externalizable::to_external(&__result));
    __result
}

// fyrox_lite::lite_ui::TextBuilder

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTextBuilder {
    pub foreground: NativeBrush_option,
    pub font_size: f32_option,
}
native_utils!(
    NativeTextBuilder,
    NativeTextBuilder_array,
    NativeTextBuilder_option,
    NativeTextBuilder_result
);
impl From<NativeTextBuilder> for fyrox_lite::lite_ui::TextBuilder {
    fn from(__value: NativeTextBuilder) -> Self {
        let foreground = __value.foreground;
        let foreground = if foreground.present {
            Some({
                let foreground = foreground.value;
                foreground.into()
            })
        } else {
            None
        };
        let font_size = __value.font_size;
        let font_size = if font_size.present {
            Some({
                let font_size = font_size.value;
                font_size
            })
        } else {
            None
        };
        Self {
            foreground,
            font_size,
        }
    }
}
impl From<fyrox_lite::lite_ui::TextBuilder> for NativeTextBuilder {
    fn from(__value: fyrox_lite::lite_ui::TextBuilder) -> Self {
        let foreground = __value.foreground;
        let foreground =
            <NativeBrush as NativeType>::to_native_option(if let Some(foreground) = foreground {
                Some(foreground.into())
            } else {
                None
            });
        let font_size = __value.font_size;
        let font_size = <f32 as NativeType>::to_native_option(if let Some(font_size) = font_size {
            Some(font_size)
        } else {
            None
        });
        Self {
            foreground,
            font_size,
        }
    }
}

// fyrox_lite::lite_ui::Brush

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush {
    pub solid_color: NativeHandle_option,
    pub linear_gradient: NativeLinearGradient_option,
    pub radial_gradient: NativeRadialGradient_option,
}
native_utils!(
    NativeBrush,
    NativeBrush_array,
    NativeBrush_option,
    NativeBrush_result
);
impl From<NativeBrush> for fyrox_lite::lite_ui::Brush {
    fn from(__value: NativeBrush) -> Self {
        let solid_color = __value.solid_color;
        let solid_color = if solid_color.present {
            Some({
                let solid_color = solid_color.value;
                Externalizable::from_external(solid_color.as_u128())
            })
        } else {
            None
        };
        let linear_gradient = __value.linear_gradient;
        let linear_gradient = if linear_gradient.present {
            Some({
                let linear_gradient = linear_gradient.value;
                linear_gradient.into()
            })
        } else {
            None
        };
        let radial_gradient = __value.radial_gradient;
        let radial_gradient = if radial_gradient.present {
            Some({
                let radial_gradient = radial_gradient.value;
                radial_gradient.into()
            })
        } else {
            None
        };
        Self {
            solid_color,
            linear_gradient,
            radial_gradient,
        }
    }
}
impl From<fyrox_lite::lite_ui::Brush> for NativeBrush {
    fn from(__value: fyrox_lite::lite_ui::Brush) -> Self {
        let solid_color = __value.solid_color;
        let solid_color = <NativeHandle as NativeType>::to_native_option(
            if let Some(solid_color) = solid_color {
                Some(NativeHandle::from_u128(Externalizable::to_external(
                    &solid_color,
                )))
            } else {
                None
            },
        );
        let linear_gradient = __value.linear_gradient;
        let linear_gradient = <NativeLinearGradient as NativeType>::to_native_option(
            if let Some(linear_gradient) = linear_gradient {
                Some(linear_gradient.into())
            } else {
                None
            },
        );
        let radial_gradient = __value.radial_gradient;
        let radial_gradient = <NativeRadialGradient as NativeType>::to_native_option(
            if let Some(radial_gradient) = radial_gradient {
                Some(radial_gradient.into())
            } else {
                None
            },
        );
        Self {
            solid_color,
            linear_gradient,
            radial_gradient,
        }
    }
}

// fyrox_lite::lite_ui::LinearGradient

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLinearGradient {
    pub from: NativeVector2,
    pub to: NativeVector2,
    pub stops: NativeGradientPoint_array,
}
native_utils!(
    NativeLinearGradient,
    NativeLinearGradient_array,
    NativeLinearGradient_option,
    NativeLinearGradient_result
);
impl From<NativeLinearGradient> for fyrox_lite::lite_ui::LinearGradient {
    fn from(__value: NativeLinearGradient) -> Self {
        let from = __value.from;
        let from = from.into();
        let to = __value.to;
        let to = to.into();
        let stops = __value.stops;
        let stops = <NativeGradientPoint as NativeType>::from_native_array(stops)
            .into_iter()
            .map(|stops| stops.into())
            .collect::<Vec<_>>();
        Self { from, to, stops }
    }
}
impl From<fyrox_lite::lite_ui::LinearGradient> for NativeLinearGradient {
    fn from(__value: fyrox_lite::lite_ui::LinearGradient) -> Self {
        let from = __value.from;
        let from = from.into();
        let to = __value.to;
        let to = to.into();
        let stops = __value.stops;
        let stops = <NativeGradientPoint as NativeType>::to_native_array(
            stops
                .into_iter()
                .map(|stops| stops.into())
                .collect::<Vec<_>>(),
        );
        Self { from, to, stops }
    }
}

// fyrox_lite::lite_ui::RadialGradient

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRadialGradient {
    pub center: NativeVector2,
    pub stops: NativeGradientPoint_array,
}
native_utils!(
    NativeRadialGradient,
    NativeRadialGradient_array,
    NativeRadialGradient_option,
    NativeRadialGradient_result
);
impl From<NativeRadialGradient> for fyrox_lite::lite_ui::RadialGradient {
    fn from(__value: NativeRadialGradient) -> Self {
        let center = __value.center;
        let center = center.into();
        let stops = __value.stops;
        let stops = <NativeGradientPoint as NativeType>::from_native_array(stops)
            .into_iter()
            .map(|stops| stops.into())
            .collect::<Vec<_>>();
        Self { center, stops }
    }
}
impl From<fyrox_lite::lite_ui::RadialGradient> for NativeRadialGradient {
    fn from(__value: fyrox_lite::lite_ui::RadialGradient) -> Self {
        let center = __value.center;
        let center = center.into();
        let stops = __value.stops;
        let stops = <NativeGradientPoint as NativeType>::to_native_array(
            stops
                .into_iter()
                .map(|stops| stops.into())
                .collect::<Vec<_>>(),
        );
        Self { center, stops }
    }
}

// fyrox_lite::lite_ui::Color

// fyrox_lite::lite_ui::GradientPoint

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeGradientPoint {
    pub stop: f32,
    pub color: NativeHandle,
}
native_utils!(
    NativeGradientPoint,
    NativeGradientPoint_array,
    NativeGradientPoint_option,
    NativeGradientPoint_result
);
impl From<NativeGradientPoint> for fyrox_lite::lite_ui::GradientPoint {
    fn from(__value: NativeGradientPoint) -> Self {
        let stop = __value.stop;
        let stop = stop;
        let color = __value.color;
        let color = Externalizable::from_external(color.as_u128());
        Self { stop, color }
    }
}
impl From<fyrox_lite::lite_ui::GradientPoint> for NativeGradientPoint {
    fn from(__value: fyrox_lite::lite_ui::GradientPoint) -> Self {
        let stop = __value.stop;
        let stop = stop;
        let color = __value.color;
        let color = NativeHandle::from_u128(Externalizable::to_external(&color));
        Self { stop, color }
    }
}

// fyrox_lite::lite_math::PodVector3

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
native_utils!(
    NativeVector3,
    NativeVector3_array,
    NativeVector3_option,
    NativeVector3_result
);
impl From<NativeVector3> for fyrox_lite::lite_math::PodVector3 {
    fn from(__value: NativeVector3) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        let z = __value.z;
        let z = z;
        Self { x, y, z }
    }
}
impl From<fyrox_lite::lite_math::PodVector3> for NativeVector3 {
    fn from(__value: fyrox_lite::lite_math::PodVector3) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        let z = __value.z;
        let z = z;
        Self { x, y, z }
    }
}

// fyrox_lite::lite_math::PodVector2

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2 {
    pub x: f32,
    pub y: f32,
}
native_utils!(
    NativeVector2,
    NativeVector2_array,
    NativeVector2_option,
    NativeVector2_result
);
impl From<NativeVector2> for fyrox_lite::lite_math::PodVector2 {
    fn from(__value: NativeVector2) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        Self { x, y }
    }
}
impl From<fyrox_lite::lite_math::PodVector2> for NativeVector2 {
    fn from(__value: fyrox_lite::lite_math::PodVector2) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        Self { x, y }
    }
}

// fyrox_lite::lite_math::PodVector2i

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2i {
    pub x: i64,
    pub y: i64,
}
native_utils!(
    NativeVector2i,
    NativeVector2i_array,
    NativeVector2i_option,
    NativeVector2i_result
);
impl From<NativeVector2i> for fyrox_lite::lite_math::PodVector2i {
    fn from(__value: NativeVector2i) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        Self { x, y }
    }
}
impl From<fyrox_lite::lite_math::PodVector2i> for NativeVector2i {
    fn from(__value: fyrox_lite::lite_math::PodVector2i) -> Self {
        let x = __value.x;
        let x = x;
        let y = __value.y;
        let y = y;
        Self { x, y }
    }
}

// fyrox_lite::lite_math::PodQuaternion

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub w: f32,
}
native_utils!(
    NativeQuaternion,
    NativeQuaternion_array,
    NativeQuaternion_option,
    NativeQuaternion_result
);
impl From<NativeQuaternion> for fyrox_lite::lite_math::PodQuaternion {
    fn from(__value: NativeQuaternion) -> Self {
        let i = __value.i;
        let i = i;
        let j = __value.j;
        let j = j;
        let k = __value.k;
        let k = k;
        let w = __value.w;
        let w = w;
        Self { i, j, k, w }
    }
}
impl From<fyrox_lite::lite_math::PodQuaternion> for NativeQuaternion {
    fn from(__value: fyrox_lite::lite_math::PodQuaternion) -> Self {
        let i = __value.i;
        let i = i;
        let j = __value.j;
        let j = j;
        let k = __value.k;
        let k = k;
        let w = __value.w;
        let w = w;
        Self { i, j, k, w }
    }
}

// fyrox_lite::lite_plugin::LitePlugin

pub extern "C" fn fyrox_lite_Plugin_get(class_name: NativeString) -> NativeHandle_result {
    let mut class_name =
        String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let mut _stub = ();
    let __result = fyrox_lite::lite_plugin::LitePlugin::get::<NativeHandle>(class_name, _stub);
    let __result = <NativeHandle as NativeType>::to_native_result(match __result {
        Ok(__result) => Ok(__result.into()),
        Err(err) => Err(err),
    });
    __result
}

// fyrox_lite::lite_node::LiteNode

pub extern "C" fn fyrox_lite_Node_as_rigid_body(__this: NativeHandle) -> NativeHandle_option {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.as_rigid_body();
    let __result =
        <NativeHandle as NativeType>::to_native_option(if let Some(__result) = __result {
            Some(NativeHandle::from_u128(Externalizable::to_external(
                &__result,
            )))
        } else {
            None
        });
    __result
}
pub extern "C" fn fyrox_lite_Node_get_name(__this: NativeHandle) -> NativeString_result {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_name();
    let __result = <NativeString as NativeType>::to_native_result(match __result {
        Ok(__result) => Ok(<u8 as NativeType>::to_native_array(__result.into_bytes())),
        Err(err) => Err(err),
    });
    __result
}
pub extern "C" fn fyrox_lite_Node_get_alive(__this: NativeHandle) -> bool {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_alive();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_destroy(__this: NativeHandle) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.destroy();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_global_position(__this: NativeHandle) -> NativeVector3 {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_global_position();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_local_position(__this: NativeHandle) -> NativeVector3 {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_local_position();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_get_local_rotation(__this: NativeHandle) -> NativeQuaternion {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_local_rotation();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_send_hierarchical(
    __this: NativeHandle,
    routing: NativeRoutingStrategy,
    payload: NativeInstanceId,
) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut routing = routing.into();
    let mut payload = payload;
    let __result = __this.send_hierarchical::<NativeHandle>(routing, payload);
    __result
}
pub extern "C" fn fyrox_lite_Node_set_local_position(
    __this: NativeHandle,
    new_pos: NativeVector3,
) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut new_pos = new_pos.into();
    let __result = __this.set_local_position(new_pos);
    __result
}
pub extern "C" fn fyrox_lite_Node_set_local_rotation(
    __this: NativeHandle,
    value: NativeQuaternion,
) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut value = value.into();
    let __result = __this.set_local_rotation(value);
    __result
}
pub extern "C" fn fyrox_lite_Node_subscribe_to(__this: NativeHandle) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut _stub = ();
    let __result = __this.subscribe_to::<NativeHandle>(_stub);
    __result
}
pub extern "C" fn fyrox_lite_Node_find_collider_in_children(
    __this: NativeHandle,
) -> NativeHandle_option {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.find_collider_in_children();
    let __result =
        <NativeHandle as NativeType>::to_native_option(if let Some(__result) = __result {
            Some(NativeHandle::from_u128(Externalizable::to_external(
                &__result,
            )))
        } else {
            None
        });
    __result
}
pub extern "C" fn fyrox_lite_Node_get_valid(__this: NativeHandle) -> bool {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_valid();
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_get_parent(__this: NativeHandle) -> NativeHandle {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_parent();
    let __result = NativeHandle::from_u128(Externalizable::to_external(&__result));
    __result
}
pub extern "C" fn fyrox_lite_Node_add_script(
    __this: NativeHandle,
    class_name: NativeString,
) -> NativeHandle_result {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut class_name =
        String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let mut _stub = ();
    let __result = __this.add_script::<NativeHandle>(class_name, _stub);
    let __result = <NativeHandle as NativeType>::to_native_result(match __result {
        Ok(__result) => Ok(__result.into()),
        Err(err) => Err(err),
    });
    __result
}
pub extern "C" fn fyrox_lite_Node_find_script(
    __this: NativeHandle,
    class_name: NativeString,
) -> NativeHandle_option_result {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut class_name =
        String::from_utf8(<u8 as NativeType>::from_native_array(class_name)).unwrap();
    let mut _stub = ();
    let __result = __this.find_script::<NativeHandle>(class_name, _stub);
    let __result = <NativeHandle_option as NativeType>::to_native_result(match __result {
        Ok(__result) => Ok(<NativeHandle as NativeType>::to_native_option(
            if let Some(__result) = __result {
                Some(__result.into())
            } else {
                None
            },
        )),
        Err(err) => Err(err),
    });
    __result
}
pub extern "C" fn fyrox_lite_Node_get_global_rotation(__this: NativeHandle) -> NativeQuaternion {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_global_rotation();
    let __result = __result.into();
    __result
}
pub extern "C" fn fyrox_lite_Node_tag_is(__this: NativeHandle, tag: NativeString) -> bool {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut tag = String::from_utf8(<u8 as NativeType>::from_native_array(tag)).unwrap();
    let __result = __this.tag_is(tag);
    let __result = __result;
    __result
}
pub extern "C" fn fyrox_lite_Node_set_tag(__this: NativeHandle, tag: NativeString) -> () {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let mut tag = String::from_utf8(<u8 as NativeType>::from_native_array(tag)).unwrap();
    let __result = __this.set_tag(tag);
    __result
}
pub extern "C" fn fyrox_lite_Node_get_tag(__this: NativeHandle) -> NativeString {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_tag();
    let __result = <u8 as NativeType>::to_native_array(__result.into_bytes());
    __result
}

// fyrox_lite::lite_node::LiteRoutingStrategy

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRoutingStrategy {
    pub tag: u8,
}
native_utils!(
    NativeRoutingStrategy,
    NativeRoutingStrategy_array,
    NativeRoutingStrategy_option,
    NativeRoutingStrategy_result
);
impl From<NativeRoutingStrategy> for fyrox_lite::lite_node::LiteRoutingStrategy {
    fn from(__value: NativeRoutingStrategy) -> Self {
        if __value.tag == NativeRoutingStrategy::Up {
            return Self::Up;
        }
        if __value.tag == NativeRoutingStrategy::Down {
            return Self::Down;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_node::LiteRoutingStrategy> for NativeRoutingStrategy {
    fn from(__value: fyrox_lite::lite_node::LiteRoutingStrategy) -> Self {
        if let fyrox_lite::lite_node::LiteRoutingStrategy::Up = __value {
            return NativeRoutingStrategy {
                tag: NativeRoutingStrategy::Up,
            };
        }
        if let fyrox_lite::lite_node::LiteRoutingStrategy::Down = __value {
            return NativeRoutingStrategy {
                tag: NativeRoutingStrategy::Down,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeRoutingStrategy {
    pub const Up: u8 = 0;
    pub const Down: u8 = 1;
}
