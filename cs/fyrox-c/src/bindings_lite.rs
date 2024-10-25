
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
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

// fyrox_lite::lite_event::Event

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeEvent {
    pub tag: u8,
    pub value: NativeEventVariantContainer,
}
native_utils!(
    NativeEvent,
    NativeEvent_array,
    NativeEvent_option,
    NativeEvent_result
);
impl From<NativeEvent> for fyrox_lite::lite_event::Event {
    fn from(__value: NativeEvent) -> Self {
        if __value.tag == NativeEvent::WindowEvent {
            let window_id = unsafe { __value.value.WindowEvent.window_id };
            let window_id = window_id;
            let event = unsafe { __value.value.WindowEvent.event };
            let event = event.into();
            return Self::WindowEvent { window_id, event };
        }
        if __value.tag == NativeEvent::DeviceEvent {
            let event = unsafe { __value.value.DeviceEvent.event };
            let event = event.into();
            return Self::DeviceEvent { event };
        }
        if __value.tag == NativeEvent::Suspended {
            return Self::Suspended;
        }
        if __value.tag == NativeEvent::Resumed {
            return Self::Resumed;
        }
        if __value.tag == NativeEvent::AboutToWait {
            return Self::AboutToWait;
        }
        if __value.tag == NativeEvent::LoopExiting {
            return Self::LoopExiting;
        }
        if __value.tag == NativeEvent::MemoryWarning {
            return Self::MemoryWarning;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::Event> for NativeEvent {
    fn from(__value: fyrox_lite::lite_event::Event) -> Self {
        if let fyrox_lite::lite_event::Event::WindowEvent { window_id, event } = __value {
            let window_id = window_id;
            let event = event.into();
            return NativeEvent {
                tag: NativeEvent::WindowEvent,
                value: NativeEventVariantContainer {
                    WindowEvent: NativeEvent_WindowEvent { window_id, event },
                },
            };
        }
        if let fyrox_lite::lite_event::Event::DeviceEvent { event } = __value {
            let event = event.into();
            return NativeEvent {
                tag: NativeEvent::DeviceEvent,
                value: NativeEventVariantContainer {
                    DeviceEvent: NativeEvent_DeviceEvent { event },
                },
            };
        }
        if let fyrox_lite::lite_event::Event::Suspended = __value {
            return NativeEvent {
                tag: NativeEvent::Suspended,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::Event::Resumed = __value {
            return NativeEvent {
                tag: NativeEvent::Resumed,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::Event::AboutToWait = __value {
            return NativeEvent {
                tag: NativeEvent::AboutToWait,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::Event::LoopExiting = __value {
            return NativeEvent {
                tag: NativeEvent::LoopExiting,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::Event::MemoryWarning = __value {
            return NativeEvent {
                tag: NativeEvent::MemoryWarning,
                value: unsafe { std::mem::zeroed() },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeEvent {
    pub const WindowEvent: u8 = 0;
    pub const DeviceEvent: u8 = 1;
    pub const Suspended: u8 = 2;
    pub const Resumed: u8 = 3;
    pub const AboutToWait: u8 = 4;
    pub const LoopExiting: u8 = 5;
    pub const MemoryWarning: u8 = 6;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeEventVariantContainer {
    pub WindowEvent: NativeEvent_WindowEvent,
    pub DeviceEvent: NativeEvent_DeviceEvent,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeEvent_WindowEvent {
    pub window_id: i64,
    pub event: NativeWindowEvent,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeEvent_DeviceEvent {
    pub event: NativeDeviceEvent,
}

// fyrox_lite::lite_event::StartCause

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeStartCause {
    pub tag: u8,
}
native_utils!(
    NativeStartCause,
    NativeStartCause_array,
    NativeStartCause_option,
    NativeStartCause_result
);
impl From<NativeStartCause> for fyrox_lite::lite_event::StartCause {
    fn from(__value: NativeStartCause) -> Self {
        if __value.tag == NativeStartCause::ResumeTimeReached {
            return Self::ResumeTimeReached;
        }
        if __value.tag == NativeStartCause::WaitCancelled {
            return Self::WaitCancelled;
        }
        if __value.tag == NativeStartCause::Poll {
            return Self::Poll;
        }
        if __value.tag == NativeStartCause::Init {
            return Self::Init;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::StartCause> for NativeStartCause {
    fn from(__value: fyrox_lite::lite_event::StartCause) -> Self {
        if let fyrox_lite::lite_event::StartCause::ResumeTimeReached = __value {
            return NativeStartCause {
                tag: NativeStartCause::ResumeTimeReached,
            };
        }
        if let fyrox_lite::lite_event::StartCause::WaitCancelled = __value {
            return NativeStartCause {
                tag: NativeStartCause::WaitCancelled,
            };
        }
        if let fyrox_lite::lite_event::StartCause::Poll = __value {
            return NativeStartCause {
                tag: NativeStartCause::Poll,
            };
        }
        if let fyrox_lite::lite_event::StartCause::Init = __value {
            return NativeStartCause {
                tag: NativeStartCause::Init,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeStartCause {
    pub const ResumeTimeReached: u8 = 0;
    pub const WaitCancelled: u8 = 1;
    pub const Poll: u8 = 2;
    pub const Init: u8 = 3;
}

// fyrox_lite::lite_event::WindowEvent

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent {
    pub tag: u8,
    pub value: NativeWindowEventVariantContainer,
}
native_utils!(
    NativeWindowEvent,
    NativeWindowEvent_array,
    NativeWindowEvent_option,
    NativeWindowEvent_result
);
impl From<NativeWindowEvent> for fyrox_lite::lite_event::WindowEvent {
    fn from(__value: NativeWindowEvent) -> Self {
        if __value.tag == NativeWindowEvent::ActivationTokenDone {
            return Self::ActivationTokenDone;
        }
        if __value.tag == NativeWindowEvent::Resized {
            let _0 = unsafe { __value.value.Resized._0 };
            let _0 = _0.into();
            return Self::Resized(_0);
        }
        if __value.tag == NativeWindowEvent::Moved {
            let _0 = unsafe { __value.value.Moved._0 };
            let _0 = _0.into();
            return Self::Moved(_0);
        }
        if __value.tag == NativeWindowEvent::CloseRequested {
            return Self::CloseRequested;
        }
        if __value.tag == NativeWindowEvent::Destroyed {
            return Self::Destroyed;
        }
        if __value.tag == NativeWindowEvent::DroppedFile {
            let _0 = unsafe { __value.value.DroppedFile._0 };
            let _0 = String::from_utf8(<u8 as NativeType>::from_native_array(_0)).unwrap();
            return Self::DroppedFile(_0);
        }
        if __value.tag == NativeWindowEvent::HoveredFile {
            let _0 = unsafe { __value.value.HoveredFile._0 };
            let _0 = String::from_utf8(<u8 as NativeType>::from_native_array(_0)).unwrap();
            return Self::HoveredFile(_0);
        }
        if __value.tag == NativeWindowEvent::HoveredFileCancelled {
            return Self::HoveredFileCancelled;
        }
        if __value.tag == NativeWindowEvent::Focused {
            let _0 = unsafe { __value.value.Focused._0 };
            let _0 = _0;
            return Self::Focused(_0);
        }
        if __value.tag == NativeWindowEvent::KeyboardInput {
            let event = unsafe { __value.value.KeyboardInput.event };
            let event = event.into();
            let is_synthetic = unsafe { __value.value.KeyboardInput.is_synthetic };
            let is_synthetic = is_synthetic;
            return Self::KeyboardInput {
                event,
                is_synthetic,
            };
        }
        if __value.tag == NativeWindowEvent::ModifiersChanged {
            return Self::ModifiersChanged;
        }
        if __value.tag == NativeWindowEvent::Ime {
            return Self::Ime;
        }
        if __value.tag == NativeWindowEvent::CursorMoved {
            let position = unsafe { __value.value.CursorMoved.position };
            let position = position.into();
            return Self::CursorMoved { position };
        }
        if __value.tag == NativeWindowEvent::CursorEntered {
            return Self::CursorEntered;
        }
        if __value.tag == NativeWindowEvent::CursorLeft {
            return Self::CursorLeft;
        }
        if __value.tag == NativeWindowEvent::MouseWheel {
            let delta = unsafe { __value.value.MouseWheel.delta };
            let delta = delta.into();
            let phase = unsafe { __value.value.MouseWheel.phase };
            let phase = phase.into();
            return Self::MouseWheel { delta, phase };
        }
        if __value.tag == NativeWindowEvent::MouseInput {
            let state = unsafe { __value.value.MouseInput.state };
            let state = state.into();
            let button = unsafe { __value.value.MouseInput.button };
            let button = button.into();
            return Self::MouseInput { state, button };
        }
        if __value.tag == NativeWindowEvent::TouchpadMagnify {
            let delta = unsafe { __value.value.TouchpadMagnify.delta };
            let delta = delta;
            let phase = unsafe { __value.value.TouchpadMagnify.phase };
            let phase = phase.into();
            return Self::TouchpadMagnify { delta, phase };
        }
        if __value.tag == NativeWindowEvent::SmartMagnify {
            return Self::SmartMagnify;
        }
        if __value.tag == NativeWindowEvent::TouchpadRotate {
            let delta = unsafe { __value.value.TouchpadRotate.delta };
            let delta = delta;
            let phase = unsafe { __value.value.TouchpadRotate.phase };
            let phase = phase.into();
            return Self::TouchpadRotate { delta, phase };
        }
        if __value.tag == NativeWindowEvent::TouchpadPressure {
            let pressure = unsafe { __value.value.TouchpadPressure.pressure };
            let pressure = pressure;
            let stage = unsafe { __value.value.TouchpadPressure.stage };
            let stage = stage;
            return Self::TouchpadPressure { pressure, stage };
        }
        if __value.tag == NativeWindowEvent::AxisMotion {
            let axis = unsafe { __value.value.AxisMotion.axis };
            let axis = axis;
            let value = unsafe { __value.value.AxisMotion.value };
            let value = value;
            return Self::AxisMotion { axis, value };
        }
        if __value.tag == NativeWindowEvent::Touch {
            let _0 = unsafe { __value.value.Touch._0 };
            let _0 = _0.into();
            return Self::Touch(_0);
        }
        if __value.tag == NativeWindowEvent::ScaleFactorChanged {
            let scale_factor = unsafe { __value.value.ScaleFactorChanged.scale_factor };
            let scale_factor = scale_factor;
            let inner_size_writer = unsafe { __value.value.ScaleFactorChanged.inner_size_writer };
            let inner_size_writer = Externalizable::from_external(inner_size_writer.as_u128());
            return Self::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            };
        }
        if __value.tag == NativeWindowEvent::ThemeChanged {
            return Self::ThemeChanged;
        }
        if __value.tag == NativeWindowEvent::Occluded {
            let _0 = unsafe { __value.value.Occluded._0 };
            let _0 = _0;
            return Self::Occluded(_0);
        }
        if __value.tag == NativeWindowEvent::RedrawRequested {
            return Self::RedrawRequested;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::WindowEvent> for NativeWindowEvent {
    fn from(__value: fyrox_lite::lite_event::WindowEvent) -> Self {
        if let fyrox_lite::lite_event::WindowEvent::ActivationTokenDone = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::ActivationTokenDone,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Resized(_0) = __value {
            let _0 = _0.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::Resized,
                value: NativeWindowEventVariantContainer {
                    Resized: NativeWindowEvent_Resized { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Moved(_0) = __value {
            let _0 = _0.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::Moved,
                value: NativeWindowEventVariantContainer {
                    Moved: NativeWindowEvent_Moved { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::CloseRequested = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::CloseRequested,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Destroyed = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::Destroyed,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::DroppedFile(_0) = __value {
            let _0 = <u8 as NativeType>::to_native_array(_0.into_bytes());
            return NativeWindowEvent {
                tag: NativeWindowEvent::DroppedFile,
                value: NativeWindowEventVariantContainer {
                    DroppedFile: NativeWindowEvent_DroppedFile { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::HoveredFile(_0) = __value {
            let _0 = <u8 as NativeType>::to_native_array(_0.into_bytes());
            return NativeWindowEvent {
                tag: NativeWindowEvent::HoveredFile,
                value: NativeWindowEventVariantContainer {
                    HoveredFile: NativeWindowEvent_HoveredFile { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::HoveredFileCancelled = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::HoveredFileCancelled,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Focused(_0) = __value {
            let _0 = _0;
            return NativeWindowEvent {
                tag: NativeWindowEvent::Focused,
                value: NativeWindowEventVariantContainer {
                    Focused: NativeWindowEvent_Focused { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::KeyboardInput {
            event,
            is_synthetic,
        } = __value
        {
            let event = event.into();
            let is_synthetic = is_synthetic;
            return NativeWindowEvent {
                tag: NativeWindowEvent::KeyboardInput,
                value: NativeWindowEventVariantContainer {
                    KeyboardInput: NativeWindowEvent_KeyboardInput {
                        event,
                        is_synthetic,
                    },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::ModifiersChanged = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::ModifiersChanged,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Ime = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::Ime,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::CursorMoved { position } = __value {
            let position = position.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::CursorMoved,
                value: NativeWindowEventVariantContainer {
                    CursorMoved: NativeWindowEvent_CursorMoved { position },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::CursorEntered = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::CursorEntered,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::CursorLeft = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::CursorLeft,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::MouseWheel { delta, phase } = __value {
            let delta = delta.into();
            let phase = phase.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::MouseWheel,
                value: NativeWindowEventVariantContainer {
                    MouseWheel: NativeWindowEvent_MouseWheel { delta, phase },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::MouseInput { state, button } = __value {
            let state = state.into();
            let button = button.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::MouseInput,
                value: NativeWindowEventVariantContainer {
                    MouseInput: NativeWindowEvent_MouseInput { state, button },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::TouchpadMagnify { delta, phase } = __value {
            let delta = delta;
            let phase = phase.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::TouchpadMagnify,
                value: NativeWindowEventVariantContainer {
                    TouchpadMagnify: NativeWindowEvent_TouchpadMagnify { delta, phase },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::SmartMagnify = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::SmartMagnify,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::TouchpadRotate { delta, phase } = __value {
            let delta = delta;
            let phase = phase.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::TouchpadRotate,
                value: NativeWindowEventVariantContainer {
                    TouchpadRotate: NativeWindowEvent_TouchpadRotate { delta, phase },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::TouchpadPressure { pressure, stage } = __value {
            let pressure = pressure;
            let stage = stage;
            return NativeWindowEvent {
                tag: NativeWindowEvent::TouchpadPressure,
                value: NativeWindowEventVariantContainer {
                    TouchpadPressure: NativeWindowEvent_TouchpadPressure { pressure, stage },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::AxisMotion { axis, value } = __value {
            let axis = axis;
            let value = value;
            return NativeWindowEvent {
                tag: NativeWindowEvent::AxisMotion,
                value: NativeWindowEventVariantContainer {
                    AxisMotion: NativeWindowEvent_AxisMotion { axis, value },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Touch(_0) = __value {
            let _0 = _0.into();
            return NativeWindowEvent {
                tag: NativeWindowEvent::Touch,
                value: NativeWindowEventVariantContainer {
                    Touch: NativeWindowEvent_Touch { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::ScaleFactorChanged {
            scale_factor,
            inner_size_writer,
        } = __value
        {
            let scale_factor = scale_factor;
            let inner_size_writer =
                NativeHandle::from_u128(Externalizable::to_external(&inner_size_writer));
            return NativeWindowEvent {
                tag: NativeWindowEvent::ScaleFactorChanged,
                value: NativeWindowEventVariantContainer {
                    ScaleFactorChanged: NativeWindowEvent_ScaleFactorChanged {
                        scale_factor,
                        inner_size_writer,
                    },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::ThemeChanged = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::ThemeChanged,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::Occluded(_0) = __value {
            let _0 = _0;
            return NativeWindowEvent {
                tag: NativeWindowEvent::Occluded,
                value: NativeWindowEventVariantContainer {
                    Occluded: NativeWindowEvent_Occluded { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::WindowEvent::RedrawRequested = __value {
            return NativeWindowEvent {
                tag: NativeWindowEvent::RedrawRequested,
                value: unsafe { std::mem::zeroed() },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeWindowEvent {
    pub const ActivationTokenDone: u8 = 0;
    pub const Resized: u8 = 1;
    pub const Moved: u8 = 2;
    pub const CloseRequested: u8 = 3;
    pub const Destroyed: u8 = 4;
    pub const DroppedFile: u8 = 5;
    pub const HoveredFile: u8 = 6;
    pub const HoveredFileCancelled: u8 = 7;
    pub const Focused: u8 = 8;
    pub const KeyboardInput: u8 = 9;
    pub const ModifiersChanged: u8 = 10;
    pub const Ime: u8 = 11;
    pub const CursorMoved: u8 = 12;
    pub const CursorEntered: u8 = 13;
    pub const CursorLeft: u8 = 14;
    pub const MouseWheel: u8 = 15;
    pub const MouseInput: u8 = 16;
    pub const TouchpadMagnify: u8 = 17;
    pub const SmartMagnify: u8 = 18;
    pub const TouchpadRotate: u8 = 19;
    pub const TouchpadPressure: u8 = 20;
    pub const AxisMotion: u8 = 21;
    pub const Touch: u8 = 22;
    pub const ScaleFactorChanged: u8 = 23;
    pub const ThemeChanged: u8 = 24;
    pub const Occluded: u8 = 25;
    pub const RedrawRequested: u8 = 26;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeWindowEventVariantContainer {
    pub Resized: NativeWindowEvent_Resized,
    pub Moved: NativeWindowEvent_Moved,
    pub DroppedFile: NativeWindowEvent_DroppedFile,
    pub HoveredFile: NativeWindowEvent_HoveredFile,
    pub Focused: NativeWindowEvent_Focused,
    pub KeyboardInput: NativeWindowEvent_KeyboardInput,
    pub CursorMoved: NativeWindowEvent_CursorMoved,
    pub MouseWheel: NativeWindowEvent_MouseWheel,
    pub MouseInput: NativeWindowEvent_MouseInput,
    pub TouchpadMagnify: NativeWindowEvent_TouchpadMagnify,
    pub TouchpadRotate: NativeWindowEvent_TouchpadRotate,
    pub TouchpadPressure: NativeWindowEvent_TouchpadPressure,
    pub AxisMotion: NativeWindowEvent_AxisMotion,
    pub Touch: NativeWindowEvent_Touch,
    pub ScaleFactorChanged: NativeWindowEvent_ScaleFactorChanged,
    pub Occluded: NativeWindowEvent_Occluded,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_Resized {
    pub _0: NativeVector2i,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_Moved {
    pub _0: NativeVector2i,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_DroppedFile {
    pub _0: NativeString,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_HoveredFile {
    pub _0: NativeString,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_Focused {
    pub _0: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_KeyboardInput {
    pub event: NativeKeyEvent,
    pub is_synthetic: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_CursorMoved {
    pub position: NativeVector2i,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_MouseWheel {
    pub delta: NativeMouseScrollDelta,
    pub phase: NativeTouchPhase,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_MouseInput {
    pub state: NativeElementState,
    pub button: NativeMouseButton,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_TouchpadMagnify {
    pub delta: f64,
    pub phase: NativeTouchPhase,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_TouchpadRotate {
    pub delta: f32,
    pub phase: NativeTouchPhase,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_TouchpadPressure {
    pub pressure: f32,
    pub stage: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_AxisMotion {
    pub axis: i32,
    pub value: f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_Touch {
    pub _0: NativeTouch,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_ScaleFactorChanged {
    pub scale_factor: f64,
    pub inner_size_writer: NativeHandle,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindowEvent_Occluded {
    pub _0: bool,
}

// fyrox_lite::lite_event::DeviceEvent

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent {
    pub tag: u8,
    pub value: NativeDeviceEventVariantContainer,
}
native_utils!(
    NativeDeviceEvent,
    NativeDeviceEvent_array,
    NativeDeviceEvent_option,
    NativeDeviceEvent_result
);
impl From<NativeDeviceEvent> for fyrox_lite::lite_event::DeviceEvent {
    fn from(__value: NativeDeviceEvent) -> Self {
        if __value.tag == NativeDeviceEvent::Added {
            return Self::Added;
        }
        if __value.tag == NativeDeviceEvent::Removed {
            return Self::Removed;
        }
        if __value.tag == NativeDeviceEvent::MouseMotion {
            let delta = unsafe { __value.value.MouseMotion.delta };
            let delta = delta.into();
            return Self::MouseMotion { delta };
        }
        if __value.tag == NativeDeviceEvent::MouseWheel {
            let delta = unsafe { __value.value.MouseWheel.delta };
            let delta = delta.into();
            return Self::MouseWheel { delta };
        }
        if __value.tag == NativeDeviceEvent::Motion {
            let axis = unsafe { __value.value.Motion.axis };
            let axis = axis;
            let value = unsafe { __value.value.Motion.value };
            let value = value;
            return Self::Motion { axis, value };
        }
        if __value.tag == NativeDeviceEvent::Button {
            let button = unsafe { __value.value.Button.button };
            let button = button;
            let state = unsafe { __value.value.Button.state };
            let state = state.into();
            return Self::Button { button, state };
        }
        if __value.tag == NativeDeviceEvent::Key {
            let _0 = unsafe { __value.value.Key._0 };
            let _0 = _0.into();
            return Self::Key(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::DeviceEvent> for NativeDeviceEvent {
    fn from(__value: fyrox_lite::lite_event::DeviceEvent) -> Self {
        if let fyrox_lite::lite_event::DeviceEvent::Added = __value {
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::Added,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::Removed = __value {
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::Removed,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::MouseMotion { delta } = __value {
            let delta = delta.into();
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::MouseMotion,
                value: NativeDeviceEventVariantContainer {
                    MouseMotion: NativeDeviceEvent_MouseMotion { delta },
                },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::MouseWheel { delta } = __value {
            let delta = delta.into();
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::MouseWheel,
                value: NativeDeviceEventVariantContainer {
                    MouseWheel: NativeDeviceEvent_MouseWheel { delta },
                },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::Motion { axis, value } = __value {
            let axis = axis;
            let value = value;
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::Motion,
                value: NativeDeviceEventVariantContainer {
                    Motion: NativeDeviceEvent_Motion { axis, value },
                },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::Button { button, state } = __value {
            let button = button;
            let state = state.into();
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::Button,
                value: NativeDeviceEventVariantContainer {
                    Button: NativeDeviceEvent_Button { button, state },
                },
            };
        }
        if let fyrox_lite::lite_event::DeviceEvent::Key(_0) = __value {
            let _0 = _0.into();
            return NativeDeviceEvent {
                tag: NativeDeviceEvent::Key,
                value: NativeDeviceEventVariantContainer {
                    Key: NativeDeviceEvent_Key { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeDeviceEvent {
    pub const Added: u8 = 0;
    pub const Removed: u8 = 1;
    pub const MouseMotion: u8 = 2;
    pub const MouseWheel: u8 = 3;
    pub const Motion: u8 = 4;
    pub const Button: u8 = 5;
    pub const Key: u8 = 6;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeDeviceEventVariantContainer {
    pub MouseMotion: NativeDeviceEvent_MouseMotion,
    pub MouseWheel: NativeDeviceEvent_MouseWheel,
    pub Motion: NativeDeviceEvent_Motion,
    pub Button: NativeDeviceEvent_Button,
    pub Key: NativeDeviceEvent_Key,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent_MouseMotion {
    pub delta: NativeVector2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent_MouseWheel {
    pub delta: NativeMouseScrollDelta,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent_Motion {
    pub axis: i32,
    pub value: f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent_Button {
    pub button: i32,
    pub state: NativeElementState,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeDeviceEvent_Key {
    pub _0: NativeRawKeyEvent,
}

// fyrox_lite::lite_event::RawKeyEvent

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRawKeyEvent {
    pub physical_key: NativePhysicalKey,
    pub state: NativeElementState,
}
native_utils!(
    NativeRawKeyEvent,
    NativeRawKeyEvent_array,
    NativeRawKeyEvent_option,
    NativeRawKeyEvent_result
);
impl From<NativeRawKeyEvent> for fyrox_lite::lite_event::RawKeyEvent {
    fn from(__value: NativeRawKeyEvent) -> Self {
        let physical_key = __value.physical_key;
        let physical_key = physical_key.into();
        let state = __value.state;
        let state = state.into();
        Self {
            physical_key,
            state,
        }
    }
}
impl From<fyrox_lite::lite_event::RawKeyEvent> for NativeRawKeyEvent {
    fn from(__value: fyrox_lite::lite_event::RawKeyEvent) -> Self {
        let physical_key = __value.physical_key;
        let physical_key = physical_key.into();
        let state = __value.state;
        let state = state.into();
        Self {
            physical_key,
            state,
        }
    }
}

// fyrox_lite::lite_event::PhysicalKey

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePhysicalKey {
    pub tag: u8,
    pub value: NativePhysicalKeyVariantContainer,
}
native_utils!(
    NativePhysicalKey,
    NativePhysicalKey_array,
    NativePhysicalKey_option,
    NativePhysicalKey_result
);
impl From<NativePhysicalKey> for fyrox_lite::lite_event::PhysicalKey {
    fn from(__value: NativePhysicalKey) -> Self {
        if __value.tag == NativePhysicalKey::Code {
            let _0 = unsafe { __value.value.Code._0 };
            let _0 = _0.into();
            return Self::Code(_0);
        }
        if __value.tag == NativePhysicalKey::Unidentified {
            let _0 = unsafe { __value.value.Unidentified._0 };
            let _0 = _0.into();
            return Self::Unidentified(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::PhysicalKey> for NativePhysicalKey {
    fn from(__value: fyrox_lite::lite_event::PhysicalKey) -> Self {
        if let fyrox_lite::lite_event::PhysicalKey::Code(_0) = __value {
            let _0 = _0.into();
            return NativePhysicalKey {
                tag: NativePhysicalKey::Code,
                value: NativePhysicalKeyVariantContainer {
                    Code: NativePhysicalKey_Code { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::PhysicalKey::Unidentified(_0) = __value {
            let _0 = _0.into();
            return NativePhysicalKey {
                tag: NativePhysicalKey::Unidentified,
                value: NativePhysicalKeyVariantContainer {
                    Unidentified: NativePhysicalKey_Unidentified { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativePhysicalKey {
    pub const Code: u8 = 0;
    pub const Unidentified: u8 = 1;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativePhysicalKeyVariantContainer {
    pub Code: NativePhysicalKey_Code,
    pub Unidentified: NativePhysicalKey_Unidentified,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePhysicalKey_Code {
    pub _0: NativeKeyCode,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePhysicalKey_Unidentified {
    pub _0: NativeNativeKeyCode,
}

// fyrox_lite::lite_event::KeyCode

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
impl From<NativeKeyCode> for fyrox_lite::lite_event::KeyCode {
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
        if __value.tag == NativeKeyCode::KeyA {
            return Self::KeyA;
        }
        if __value.tag == NativeKeyCode::KeyB {
            return Self::KeyB;
        }
        if __value.tag == NativeKeyCode::KeyC {
            return Self::KeyC;
        }
        if __value.tag == NativeKeyCode::KeyD {
            return Self::KeyD;
        }
        if __value.tag == NativeKeyCode::KeyE {
            return Self::KeyE;
        }
        if __value.tag == NativeKeyCode::KeyF {
            return Self::KeyF;
        }
        if __value.tag == NativeKeyCode::KeyG {
            return Self::KeyG;
        }
        if __value.tag == NativeKeyCode::KeyH {
            return Self::KeyH;
        }
        if __value.tag == NativeKeyCode::KeyI {
            return Self::KeyI;
        }
        if __value.tag == NativeKeyCode::KeyJ {
            return Self::KeyJ;
        }
        if __value.tag == NativeKeyCode::KeyK {
            return Self::KeyK;
        }
        if __value.tag == NativeKeyCode::KeyL {
            return Self::KeyL;
        }
        if __value.tag == NativeKeyCode::KeyM {
            return Self::KeyM;
        }
        if __value.tag == NativeKeyCode::KeyN {
            return Self::KeyN;
        }
        if __value.tag == NativeKeyCode::KeyO {
            return Self::KeyO;
        }
        if __value.tag == NativeKeyCode::KeyP {
            return Self::KeyP;
        }
        if __value.tag == NativeKeyCode::KeyQ {
            return Self::KeyQ;
        }
        if __value.tag == NativeKeyCode::KeyR {
            return Self::KeyR;
        }
        if __value.tag == NativeKeyCode::KeyS {
            return Self::KeyS;
        }
        if __value.tag == NativeKeyCode::KeyT {
            return Self::KeyT;
        }
        if __value.tag == NativeKeyCode::KeyU {
            return Self::KeyU;
        }
        if __value.tag == NativeKeyCode::KeyV {
            return Self::KeyV;
        }
        if __value.tag == NativeKeyCode::KeyW {
            return Self::KeyW;
        }
        if __value.tag == NativeKeyCode::KeyX {
            return Self::KeyX;
        }
        if __value.tag == NativeKeyCode::KeyY {
            return Self::KeyY;
        }
        if __value.tag == NativeKeyCode::KeyZ {
            return Self::KeyZ;
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
impl From<fyrox_lite::lite_event::KeyCode> for NativeKeyCode {
    fn from(__value: fyrox_lite::lite_event::KeyCode) -> Self {
        if let fyrox_lite::lite_event::KeyCode::Backquote = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backquote,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Backslash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backslash,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BracketLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BracketLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BracketRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BracketRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Comma = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Comma,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit0 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit0,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit1,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit2,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit3,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit4,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit5,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit6,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit7,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit8,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Digit9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Digit9,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Equal = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Equal,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::IntlBackslash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlBackslash,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::IntlRo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlRo,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::IntlYen = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::IntlYen,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyA = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyA,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyB = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyB,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyC = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyC,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyD = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyD,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyE = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyE,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyF = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyF,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyG = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyG,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyH = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyH,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyI = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyI,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyJ = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyJ,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyK = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyK,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyL = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyL,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyM = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyM,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyN = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyN,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyO = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyO,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyP = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyP,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyQ = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyQ,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyR = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyR,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyS = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyS,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyT = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyT,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyU = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyU,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyV = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyV,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyW = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyW,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyX = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyX,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyY = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyY,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KeyZ = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KeyZ,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Minus = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Minus,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Period = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Period,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Quote = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Quote,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Semicolon = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Semicolon,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Slash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Slash,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::AltLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AltLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::AltRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AltRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Backspace = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Backspace,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::CapsLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::CapsLock,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ContextMenu = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ContextMenu,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ControlLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ControlLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ControlRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ControlRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Enter = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Enter,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::SuperLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::SuperLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::SuperRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::SuperRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ShiftLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ShiftLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ShiftRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ShiftRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Space = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Space,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Tab = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Tab,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Convert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Convert,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::KanaMode = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::KanaMode,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Lang1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang1,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Lang2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang2,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Lang3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang3,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Lang4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang4,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Lang5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Lang5,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NonConvert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NonConvert,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Delete = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Delete,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::End = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::End,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Help = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Help,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Home = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Home,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Insert = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Insert,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::PageDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PageDown,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::PageUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PageUp,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ArrowDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowDown,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ArrowLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ArrowRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ArrowUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ArrowUp,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumLock,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad0 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad0,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad1,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad2,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad3,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad4,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad5,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad6,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad7,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad8,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Numpad9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Numpad9,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadAdd = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadAdd,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadBackspace = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadBackspace,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadClear = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadClear,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadClearEntry = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadClearEntry,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadComma = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadComma,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadDecimal = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadDecimal,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadDivide = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadDivide,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadEnter = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadEnter,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadEqual = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadEqual,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadHash = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadHash,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMemoryAdd = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryAdd,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMemoryClear = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryClear,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMemoryRecall = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryRecall,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMemoryStore = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemoryStore,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMemorySubtract = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMemorySubtract,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadMultiply = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadMultiply,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadParenLeft = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadParenLeft,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadParenRight = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadParenRight,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadStar = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadStar,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::NumpadSubtract = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::NumpadSubtract,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Escape = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Escape,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Fn = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Fn,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::FnLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::FnLock,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::PrintScreen = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::PrintScreen,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::ScrollLock = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::ScrollLock,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Pause = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Pause,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserBack = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserBack,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserFavorites = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserFavorites,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserForward = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserForward,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserHome = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserHome,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserRefresh = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserRefresh,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserSearch = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserSearch,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::BrowserStop = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::BrowserStop,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Eject = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Eject,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::LaunchApp1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchApp1,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::LaunchApp2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchApp2,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::LaunchMail = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::LaunchMail,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::MediaPlayPause = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaPlayPause,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::MediaSelect = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaSelect,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::MediaStop = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaStop,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::MediaTrackNext = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaTrackNext,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::MediaTrackPrevious = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::MediaTrackPrevious,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Power = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Power,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Sleep = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Sleep,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::AudioVolumeDown = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeDown,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::AudioVolumeMute = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeMute,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::AudioVolumeUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::AudioVolumeUp,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::WakeUp = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::WakeUp,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Meta = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Meta,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Hyper = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Hyper,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Turbo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Turbo,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Abort = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Abort,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Resume = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Resume,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Suspend = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Suspend,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Again = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Again,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Copy = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Copy,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Cut = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Cut,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Find = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Find,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Open = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Open,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Paste = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Paste,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Props = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Props,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Select = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Select,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Undo = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Undo,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Hiragana = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Hiragana,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::Katakana = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::Katakana,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F1 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F1,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F2 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F2,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F3 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F3,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F4 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F4,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F5 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F5,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F6 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F6,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F7 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F7,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F8 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F8,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F9 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F9,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F10 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F10,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F11 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F11,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F12 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F12,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F13 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F13,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F14 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F14,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F15 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F15,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F16 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F16,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F17 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F17,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F18 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F18,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F19 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F19,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F20 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F20,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F21 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F21,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F22 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F22,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F23 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F23,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F24 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F24,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F25 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F25,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F26 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F26,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F27 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F27,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F28 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F28,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F29 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F29,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F30 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F30,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F31 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F31,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F32 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F32,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F33 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F33,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F34 = __value {
            return NativeKeyCode {
                tag: NativeKeyCode::F34,
            };
        }
        if let fyrox_lite::lite_event::KeyCode::F35 = __value {
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
    pub const KeyA: u8 = 19;
    pub const KeyB: u8 = 20;
    pub const KeyC: u8 = 21;
    pub const KeyD: u8 = 22;
    pub const KeyE: u8 = 23;
    pub const KeyF: u8 = 24;
    pub const KeyG: u8 = 25;
    pub const KeyH: u8 = 26;
    pub const KeyI: u8 = 27;
    pub const KeyJ: u8 = 28;
    pub const KeyK: u8 = 29;
    pub const KeyL: u8 = 30;
    pub const KeyM: u8 = 31;
    pub const KeyN: u8 = 32;
    pub const KeyO: u8 = 33;
    pub const KeyP: u8 = 34;
    pub const KeyQ: u8 = 35;
    pub const KeyR: u8 = 36;
    pub const KeyS: u8 = 37;
    pub const KeyT: u8 = 38;
    pub const KeyU: u8 = 39;
    pub const KeyV: u8 = 40;
    pub const KeyW: u8 = 41;
    pub const KeyX: u8 = 42;
    pub const KeyY: u8 = 43;
    pub const KeyZ: u8 = 44;
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

// fyrox_lite::lite_event::NativeKeyCode

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNativeKeyCode {
    pub tag: u8,
    pub value: NativeNativeKeyCodeVariantContainer,
}
native_utils!(
    NativeNativeKeyCode,
    NativeNativeKeyCode_array,
    NativeNativeKeyCode_option,
    NativeNativeKeyCode_result
);
impl From<NativeNativeKeyCode> for fyrox_lite::lite_event::NativeKeyCode {
    fn from(__value: NativeNativeKeyCode) -> Self {
        if __value.tag == NativeNativeKeyCode::Unidentified {
            return Self::Unidentified;
        }
        if __value.tag == NativeNativeKeyCode::Android {
            let _0 = unsafe { __value.value.Android._0 };
            let _0 = _0;
            return Self::Android(_0);
        }
        if __value.tag == NativeNativeKeyCode::MacOS {
            let _0 = unsafe { __value.value.MacOS._0 };
            let _0 = _0;
            return Self::MacOS(_0);
        }
        if __value.tag == NativeNativeKeyCode::Windows {
            let _0 = unsafe { __value.value.Windows._0 };
            let _0 = _0;
            return Self::Windows(_0);
        }
        if __value.tag == NativeNativeKeyCode::Xkb {
            let _0 = unsafe { __value.value.Xkb._0 };
            let _0 = _0;
            return Self::Xkb(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::NativeKeyCode> for NativeNativeKeyCode {
    fn from(__value: fyrox_lite::lite_event::NativeKeyCode) -> Self {
        if let fyrox_lite::lite_event::NativeKeyCode::Unidentified = __value {
            return NativeNativeKeyCode {
                tag: NativeNativeKeyCode::Unidentified,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::NativeKeyCode::Android(_0) = __value {
            let _0 = _0;
            return NativeNativeKeyCode {
                tag: NativeNativeKeyCode::Android,
                value: NativeNativeKeyCodeVariantContainer {
                    Android: NativeNativeKeyCode_Android { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::NativeKeyCode::MacOS(_0) = __value {
            let _0 = _0;
            return NativeNativeKeyCode {
                tag: NativeNativeKeyCode::MacOS,
                value: NativeNativeKeyCodeVariantContainer {
                    MacOS: NativeNativeKeyCode_MacOS { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::NativeKeyCode::Windows(_0) = __value {
            let _0 = _0;
            return NativeNativeKeyCode {
                tag: NativeNativeKeyCode::Windows,
                value: NativeNativeKeyCodeVariantContainer {
                    Windows: NativeNativeKeyCode_Windows { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::NativeKeyCode::Xkb(_0) = __value {
            let _0 = _0;
            return NativeNativeKeyCode {
                tag: NativeNativeKeyCode::Xkb,
                value: NativeNativeKeyCodeVariantContainer {
                    Xkb: NativeNativeKeyCode_Xkb { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeNativeKeyCode {
    pub const Unidentified: u8 = 0;
    pub const Android: u8 = 1;
    pub const MacOS: u8 = 2;
    pub const Windows: u8 = 3;
    pub const Xkb: u8 = 4;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeNativeKeyCodeVariantContainer {
    pub Android: NativeNativeKeyCode_Android,
    pub MacOS: NativeNativeKeyCode_MacOS,
    pub Windows: NativeNativeKeyCode_Windows,
    pub Xkb: NativeNativeKeyCode_Xkb,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNativeKeyCode_Android {
    pub _0: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNativeKeyCode_MacOS {
    pub _0: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNativeKeyCode_Windows {
    pub _0: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNativeKeyCode_Xkb {
    pub _0: i32,
}

// fyrox_lite::lite_event::KeyEvent

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeKeyEvent {
    pub physical_key: NativePhysicalKey,
    pub state: NativeElementState,
    pub repeat: bool,
}
native_utils!(
    NativeKeyEvent,
    NativeKeyEvent_array,
    NativeKeyEvent_option,
    NativeKeyEvent_result
);
impl From<NativeKeyEvent> for fyrox_lite::lite_event::KeyEvent {
    fn from(__value: NativeKeyEvent) -> Self {
        let physical_key = __value.physical_key;
        let physical_key = physical_key.into();
        let state = __value.state;
        let state = state.into();
        let repeat = __value.repeat;
        let repeat = repeat;
        Self {
            physical_key,
            state,
            repeat,
        }
    }
}
impl From<fyrox_lite::lite_event::KeyEvent> for NativeKeyEvent {
    fn from(__value: fyrox_lite::lite_event::KeyEvent) -> Self {
        let physical_key = __value.physical_key;
        let physical_key = physical_key.into();
        let state = __value.state;
        let state = state.into();
        let repeat = __value.repeat;
        let repeat = repeat;
        Self {
            physical_key,
            state,
            repeat,
        }
    }
}

// fyrox_lite::lite_event::KeyLocation

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeKeyLocation {
    pub tag: u8,
}
native_utils!(
    NativeKeyLocation,
    NativeKeyLocation_array,
    NativeKeyLocation_option,
    NativeKeyLocation_result
);
impl From<NativeKeyLocation> for fyrox_lite::lite_event::KeyLocation {
    fn from(__value: NativeKeyLocation) -> Self {
        if __value.tag == NativeKeyLocation::Standard {
            return Self::Standard;
        }
        if __value.tag == NativeKeyLocation::Left {
            return Self::Left;
        }
        if __value.tag == NativeKeyLocation::Right {
            return Self::Right;
        }
        if __value.tag == NativeKeyLocation::Numpad {
            return Self::Numpad;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::KeyLocation> for NativeKeyLocation {
    fn from(__value: fyrox_lite::lite_event::KeyLocation) -> Self {
        if let fyrox_lite::lite_event::KeyLocation::Standard = __value {
            return NativeKeyLocation {
                tag: NativeKeyLocation::Standard,
            };
        }
        if let fyrox_lite::lite_event::KeyLocation::Left = __value {
            return NativeKeyLocation {
                tag: NativeKeyLocation::Left,
            };
        }
        if let fyrox_lite::lite_event::KeyLocation::Right = __value {
            return NativeKeyLocation {
                tag: NativeKeyLocation::Right,
            };
        }
        if let fyrox_lite::lite_event::KeyLocation::Numpad = __value {
            return NativeKeyLocation {
                tag: NativeKeyLocation::Numpad,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeKeyLocation {
    pub const Standard: u8 = 0;
    pub const Left: u8 = 1;
    pub const Right: u8 = 2;
    pub const Numpad: u8 = 3;
}

// fyrox_lite::lite_event::TouchPhase

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTouchPhase {
    pub tag: u8,
}
native_utils!(
    NativeTouchPhase,
    NativeTouchPhase_array,
    NativeTouchPhase_option,
    NativeTouchPhase_result
);
impl From<NativeTouchPhase> for fyrox_lite::lite_event::TouchPhase {
    fn from(__value: NativeTouchPhase) -> Self {
        if __value.tag == NativeTouchPhase::Started {
            return Self::Started;
        }
        if __value.tag == NativeTouchPhase::Moved {
            return Self::Moved;
        }
        if __value.tag == NativeTouchPhase::Ended {
            return Self::Ended;
        }
        if __value.tag == NativeTouchPhase::Cancelled {
            return Self::Cancelled;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::TouchPhase> for NativeTouchPhase {
    fn from(__value: fyrox_lite::lite_event::TouchPhase) -> Self {
        if let fyrox_lite::lite_event::TouchPhase::Started = __value {
            return NativeTouchPhase {
                tag: NativeTouchPhase::Started,
            };
        }
        if let fyrox_lite::lite_event::TouchPhase::Moved = __value {
            return NativeTouchPhase {
                tag: NativeTouchPhase::Moved,
            };
        }
        if let fyrox_lite::lite_event::TouchPhase::Ended = __value {
            return NativeTouchPhase {
                tag: NativeTouchPhase::Ended,
            };
        }
        if let fyrox_lite::lite_event::TouchPhase::Cancelled = __value {
            return NativeTouchPhase {
                tag: NativeTouchPhase::Cancelled,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeTouchPhase {
    pub const Started: u8 = 0;
    pub const Moved: u8 = 1;
    pub const Ended: u8 = 2;
    pub const Cancelled: u8 = 3;
}

// fyrox_lite::lite_event::Touch

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTouch {
    pub phase: NativeTouchPhase,
    pub location: NativeVector2,
    pub force: NativeForce_option,
    pub id: i64,
}
native_utils!(
    NativeTouch,
    NativeTouch_array,
    NativeTouch_option,
    NativeTouch_result
);
impl From<NativeTouch> for fyrox_lite::lite_event::Touch {
    fn from(__value: NativeTouch) -> Self {
        let phase = __value.phase;
        let phase = phase.into();
        let location = __value.location;
        let location = location.into();
        let force = __value.force;
        let force = if force.present {
            Some({
                let force = force.value;
                force.into()
            })
        } else {
            None
        };
        let id = __value.id;
        let id = id;
        Self {
            phase,
            location,
            force,
            id,
        }
    }
}
impl From<fyrox_lite::lite_event::Touch> for NativeTouch {
    fn from(__value: fyrox_lite::lite_event::Touch) -> Self {
        let phase = __value.phase;
        let phase = phase.into();
        let location = __value.location;
        let location = location.into();
        let force = __value.force;
        let force = <NativeForce as NativeType>::to_native_option(if let Some(force) = force {
            Some(force.into())
        } else {
            None
        });
        let id = __value.id;
        let id = id;
        Self {
            phase,
            location,
            force,
            id,
        }
    }
}

// fyrox_lite::lite_event::Force

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeForce {
    pub tag: u8,
    pub value: NativeForceVariantContainer,
}
native_utils!(
    NativeForce,
    NativeForce_array,
    NativeForce_option,
    NativeForce_result
);
impl From<NativeForce> for fyrox_lite::lite_event::Force {
    fn from(__value: NativeForce) -> Self {
        if __value.tag == NativeForce::Calibrated {
            let force = unsafe { __value.value.Calibrated.force };
            let force = force;
            let max_possible_force = unsafe { __value.value.Calibrated.max_possible_force };
            let max_possible_force = max_possible_force;
            let altitude_angle = unsafe { __value.value.Calibrated.altitude_angle };
            let altitude_angle = if altitude_angle.present {
                Some({
                    let altitude_angle = altitude_angle.value;
                    altitude_angle
                })
            } else {
                None
            };
            return Self::Calibrated {
                force,
                max_possible_force,
                altitude_angle,
            };
        }
        if __value.tag == NativeForce::Normalized {
            let _0 = unsafe { __value.value.Normalized._0 };
            let _0 = _0;
            return Self::Normalized(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::Force> for NativeForce {
    fn from(__value: fyrox_lite::lite_event::Force) -> Self {
        if let fyrox_lite::lite_event::Force::Calibrated {
            force,
            max_possible_force,
            altitude_angle,
        } = __value
        {
            let force = force;
            let max_possible_force = max_possible_force;
            let altitude_angle = <f64 as NativeType>::to_native_option(
                if let Some(altitude_angle) = altitude_angle {
                    Some(altitude_angle)
                } else {
                    None
                },
            );
            return NativeForce {
                tag: NativeForce::Calibrated,
                value: NativeForceVariantContainer {
                    Calibrated: NativeForce_Calibrated {
                        force,
                        max_possible_force,
                        altitude_angle,
                    },
                },
            };
        }
        if let fyrox_lite::lite_event::Force::Normalized(_0) = __value {
            let _0 = _0;
            return NativeForce {
                tag: NativeForce::Normalized,
                value: NativeForceVariantContainer {
                    Normalized: NativeForce_Normalized { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeForce {
    pub const Calibrated: u8 = 0;
    pub const Normalized: u8 = 1;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeForceVariantContainer {
    pub Calibrated: NativeForce_Calibrated,
    pub Normalized: NativeForce_Normalized,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeForce_Calibrated {
    pub force: f64,
    pub max_possible_force: f64,
    pub altitude_angle: f64_option,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeForce_Normalized {
    pub _0: f64,
}

// fyrox_lite::lite_event::ElementState

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeElementState {
    pub tag: u8,
}
native_utils!(
    NativeElementState,
    NativeElementState_array,
    NativeElementState_option,
    NativeElementState_result
);
impl From<NativeElementState> for fyrox_lite::lite_event::ElementState {
    fn from(__value: NativeElementState) -> Self {
        if __value.tag == NativeElementState::Pressed {
            return Self::Pressed;
        }
        if __value.tag == NativeElementState::Released {
            return Self::Released;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::ElementState> for NativeElementState {
    fn from(__value: fyrox_lite::lite_event::ElementState) -> Self {
        if let fyrox_lite::lite_event::ElementState::Pressed = __value {
            return NativeElementState {
                tag: NativeElementState::Pressed,
            };
        }
        if let fyrox_lite::lite_event::ElementState::Released = __value {
            return NativeElementState {
                tag: NativeElementState::Released,
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeElementState {
    pub const Pressed: u8 = 0;
    pub const Released: u8 = 1;
}

// fyrox_lite::lite_event::MouseButton

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeMouseButton {
    pub tag: u8,
    pub value: NativeMouseButtonVariantContainer,
}
native_utils!(
    NativeMouseButton,
    NativeMouseButton_array,
    NativeMouseButton_option,
    NativeMouseButton_result
);
impl From<NativeMouseButton> for fyrox_lite::lite_event::MouseButton {
    fn from(__value: NativeMouseButton) -> Self {
        if __value.tag == NativeMouseButton::Left {
            return Self::Left;
        }
        if __value.tag == NativeMouseButton::Right {
            return Self::Right;
        }
        if __value.tag == NativeMouseButton::Middle {
            return Self::Middle;
        }
        if __value.tag == NativeMouseButton::Back {
            return Self::Back;
        }
        if __value.tag == NativeMouseButton::Forward {
            return Self::Forward;
        }
        if __value.tag == NativeMouseButton::Other {
            let _0 = unsafe { __value.value.Other._0 };
            let _0 = _0;
            return Self::Other(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::MouseButton> for NativeMouseButton {
    fn from(__value: fyrox_lite::lite_event::MouseButton) -> Self {
        if let fyrox_lite::lite_event::MouseButton::Left = __value {
            return NativeMouseButton {
                tag: NativeMouseButton::Left,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::MouseButton::Right = __value {
            return NativeMouseButton {
                tag: NativeMouseButton::Right,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::MouseButton::Middle = __value {
            return NativeMouseButton {
                tag: NativeMouseButton::Middle,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::MouseButton::Back = __value {
            return NativeMouseButton {
                tag: NativeMouseButton::Back,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::MouseButton::Forward = __value {
            return NativeMouseButton {
                tag: NativeMouseButton::Forward,
                value: unsafe { std::mem::zeroed() },
            };
        }
        if let fyrox_lite::lite_event::MouseButton::Other(_0) = __value {
            let _0 = _0;
            return NativeMouseButton {
                tag: NativeMouseButton::Other,
                value: NativeMouseButtonVariantContainer {
                    Other: NativeMouseButton_Other { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeMouseButton {
    pub const Left: u8 = 0;
    pub const Right: u8 = 1;
    pub const Middle: u8 = 2;
    pub const Back: u8 = 3;
    pub const Forward: u8 = 4;
    pub const Other: u8 = 5;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeMouseButtonVariantContainer {
    pub Other: NativeMouseButton_Other,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeMouseButton_Other {
    pub _0: i32,
}

// fyrox_lite::lite_event::MouseScrollDelta

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeMouseScrollDelta {
    pub tag: u8,
    pub value: NativeMouseScrollDeltaVariantContainer,
}
native_utils!(
    NativeMouseScrollDelta,
    NativeMouseScrollDelta_array,
    NativeMouseScrollDelta_option,
    NativeMouseScrollDelta_result
);
impl From<NativeMouseScrollDelta> for fyrox_lite::lite_event::MouseScrollDelta {
    fn from(__value: NativeMouseScrollDelta) -> Self {
        if __value.tag == NativeMouseScrollDelta::LineDelta {
            let _0 = unsafe { __value.value.LineDelta._0 };
            let _0 = _0.into();
            return Self::LineDelta(_0);
        }
        if __value.tag == NativeMouseScrollDelta::PixelDelta {
            let _0 = unsafe { __value.value.PixelDelta._0 };
            let _0 = _0.into();
            return Self::PixelDelta(_0);
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_event::MouseScrollDelta> for NativeMouseScrollDelta {
    fn from(__value: fyrox_lite::lite_event::MouseScrollDelta) -> Self {
        if let fyrox_lite::lite_event::MouseScrollDelta::LineDelta(_0) = __value {
            let _0 = _0.into();
            return NativeMouseScrollDelta {
                tag: NativeMouseScrollDelta::LineDelta,
                value: NativeMouseScrollDeltaVariantContainer {
                    LineDelta: NativeMouseScrollDelta_LineDelta { _0 },
                },
            };
        }
        if let fyrox_lite::lite_event::MouseScrollDelta::PixelDelta(_0) = __value {
            let _0 = _0.into();
            return NativeMouseScrollDelta {
                tag: NativeMouseScrollDelta::PixelDelta,
                value: NativeMouseScrollDeltaVariantContainer {
                    PixelDelta: NativeMouseScrollDelta_PixelDelta { _0 },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeMouseScrollDelta {
    pub const LineDelta: u8 = 0;
    pub const PixelDelta: u8 = 1;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeMouseScrollDeltaVariantContainer {
    pub LineDelta: NativeMouseScrollDelta_LineDelta,
    pub PixelDelta: NativeMouseScrollDelta_PixelDelta,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeMouseScrollDelta_LineDelta {
    pub _0: NativeVector2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeMouseScrollDelta_PixelDelta {
    pub _0: NativeVector2,
}

// fyrox_lite::lite_event::InnerSizeWriter

pub extern "C" fn fyrox_lite_InnerSizeWriter_request_inner_size(
    __this: NativeHandle,
    new_inner_size: NativeVector2i,
) -> bool {
    let mut __this: fyrox_lite::lite_event::InnerSizeWriter =
        Externalizable::from_external(__this.as_u128());
    let mut new_inner_size = new_inner_size.into();
    let __result = __this.request_inner_size(new_inner_size);
    let __result = __result;
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
    pub tag: u8,
    pub value: NativeFeatureIdVariantContainer,
}
native_utils!(
    NativeFeatureId,
    NativeFeatureId_array,
    NativeFeatureId_option,
    NativeFeatureId_result
);
impl From<NativeFeatureId> for fyrox_lite::lite_physics::LiteFeatureId {
    fn from(__value: NativeFeatureId) -> Self {
        if __value.tag == NativeFeatureId::Vertex {
            let _0 = unsafe { __value.value.Vertex._0 };
            let _0 = _0;
            return Self::Vertex(_0);
        }
        if __value.tag == NativeFeatureId::Edge {
            let _0 = unsafe { __value.value.Edge._0 };
            let _0 = _0;
            return Self::Edge(_0);
        }
        if __value.tag == NativeFeatureId::Face {
            let _0 = unsafe { __value.value.Face._0 };
            let _0 = _0;
            return Self::Face(_0);
        }
        if __value.tag == NativeFeatureId::Unknown {
            return Self::Unknown;
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_physics::LiteFeatureId> for NativeFeatureId {
    fn from(__value: fyrox_lite::lite_physics::LiteFeatureId) -> Self {
        if let fyrox_lite::lite_physics::LiteFeatureId::Vertex(_0) = __value {
            let _0 = _0;
            return NativeFeatureId {
                tag: NativeFeatureId::Vertex,
                value: NativeFeatureIdVariantContainer {
                    Vertex: NativeFeatureId_Vertex { _0 },
                },
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureId::Edge(_0) = __value {
            let _0 = _0;
            return NativeFeatureId {
                tag: NativeFeatureId::Edge,
                value: NativeFeatureIdVariantContainer {
                    Edge: NativeFeatureId_Edge { _0 },
                },
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureId::Face(_0) = __value {
            let _0 = _0;
            return NativeFeatureId {
                tag: NativeFeatureId::Face,
                value: NativeFeatureIdVariantContainer {
                    Face: NativeFeatureId_Face { _0 },
                },
            };
        }
        if let fyrox_lite::lite_physics::LiteFeatureId::Unknown = __value {
            return NativeFeatureId {
                tag: NativeFeatureId::Unknown,
                value: unsafe { std::mem::zeroed() },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeFeatureId {
    pub const Vertex: u8 = 0;
    pub const Edge: u8 = 1;
    pub const Face: u8 = 2;
    pub const Unknown: u8 = 3;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeFeatureIdVariantContainer {
    pub Vertex: NativeFeatureId_Vertex,
    pub Edge: NativeFeatureId_Edge,
    pub Face: NativeFeatureId_Face,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_Vertex {
    pub _0: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_Edge {
    pub _0: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_Face {
    pub _0: i32,
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
    pub foregound: NativeBrush_option,
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
        let foregound = __value.foregound;
        let foregound = if foregound.present {
            Some({
                let foregound = foregound.value;
                foregound.into()
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
            foregound,
            font_size,
        }
    }
}
impl From<fyrox_lite::lite_ui::TextBuilder> for NativeTextBuilder {
    fn from(__value: fyrox_lite::lite_ui::TextBuilder) -> Self {
        let foregound = __value.foregound;
        let foregound =
            <NativeBrush as NativeType>::to_native_option(if let Some(foregound) = foregound {
                Some(foregound.into())
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
            foregound,
            font_size,
        }
    }
}

// fyrox_lite::lite_ui::Brush

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush {
    pub tag: u8,
    pub value: NativeBrushVariantContainer,
}
native_utils!(
    NativeBrush,
    NativeBrush_array,
    NativeBrush_option,
    NativeBrush_result
);
impl From<NativeBrush> for fyrox_lite::lite_ui::Brush {
    fn from(__value: NativeBrush) -> Self {
        if __value.tag == NativeBrush::Solid {
            let _0 = unsafe { __value.value.Solid._0 };
            let _0 = Externalizable::from_external(_0.as_u128());
            return Self::Solid(_0);
        }
        if __value.tag == NativeBrush::LinearGradient {
            let from = unsafe { __value.value.LinearGradient.from };
            let from = from.into();
            let to = unsafe { __value.value.LinearGradient.to };
            let to = to.into();
            let stops = unsafe { __value.value.LinearGradient.stops };
            let stops = <NativeGradientPoint as NativeType>::from_native_array(stops)
                .into_iter()
                .map(|stops| stops.into())
                .collect::<Vec<_>>();
            return Self::LinearGradient { from, to, stops };
        }
        if __value.tag == NativeBrush::RadialGradient {
            let center = unsafe { __value.value.RadialGradient.center };
            let center = center.into();
            let stops = unsafe { __value.value.RadialGradient.stops };
            let stops = <NativeGradientPoint as NativeType>::from_native_array(stops)
                .into_iter()
                .map(|stops| stops.into())
                .collect::<Vec<_>>();
            return Self::RadialGradient { center, stops };
        }
        panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
    }
}
impl From<fyrox_lite::lite_ui::Brush> for NativeBrush {
    fn from(__value: fyrox_lite::lite_ui::Brush) -> Self {
        if let fyrox_lite::lite_ui::Brush::Solid(_0) = __value {
            let _0 = NativeHandle::from_u128(Externalizable::to_external(&_0));
            return NativeBrush {
                tag: NativeBrush::Solid,
                value: NativeBrushVariantContainer {
                    Solid: NativeBrush_Solid { _0 },
                },
            };
        }
        if let fyrox_lite::lite_ui::Brush::LinearGradient { from, to, stops } = __value {
            let from = from.into();
            let to = to.into();
            let stops = <NativeGradientPoint as NativeType>::to_native_array(
                stops
                    .into_iter()
                    .map(|stops| stops.into())
                    .collect::<Vec<_>>(),
            );
            return NativeBrush {
                tag: NativeBrush::LinearGradient,
                value: NativeBrushVariantContainer {
                    LinearGradient: NativeBrush_LinearGradient { from, to, stops },
                },
            };
        }
        if let fyrox_lite::lite_ui::Brush::RadialGradient { center, stops } = __value {
            let center = center.into();
            let stops = <NativeGradientPoint as NativeType>::to_native_array(
                stops
                    .into_iter()
                    .map(|stops| stops.into())
                    .collect::<Vec<_>>(),
            );
            return NativeBrush {
                tag: NativeBrush::RadialGradient,
                value: NativeBrushVariantContainer {
                    RadialGradient: NativeBrush_RadialGradient { center, stops },
                },
            };
        }
        panic!("unsupported enum: {:?}", __value)
    }
}

impl NativeBrush {
    pub const Solid: u8 = 0;
    pub const LinearGradient: u8 = 1;
    pub const RadialGradient: u8 = 2;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeBrushVariantContainer {
    pub Solid: NativeBrush_Solid,
    pub LinearGradient: NativeBrush_LinearGradient,
    pub RadialGradient: NativeBrush_RadialGradient,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_Solid {
    pub _0: NativeHandle,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_LinearGradient {
    pub from: NativeVector2,
    pub to: NativeVector2,
    pub stops: NativeGradientPoint_array,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_RadialGradient {
    pub center: NativeVector2,
    pub stops: NativeGradientPoint_array,
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
pub extern "C" fn fyrox_lite_Node_get_name(__this: NativeHandle) -> NativeString_option {
    let mut __this: fyrox_lite::lite_node::LiteNode =
        Externalizable::from_external(__this.as_u128());
    let __result = __this.get_name();
    let __result =
        <NativeString as NativeType>::to_native_option(if let Some(__result) = __result {
            Some(<u8 as NativeType>::to_native_array(__result.into_bytes()))
        } else {
            None
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
