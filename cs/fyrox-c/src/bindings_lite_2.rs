
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::unused_unit)]
#![allow(clippy::let_unit_value)]
#![allow(unused)]
use crate::bindings_manual::*;
use fyrox_lite::externalizable::Externalizable;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScene {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_scene_LiteScene_load_async(scene_path: NativeString) -> () {
    let scene_path = scene_path.into();
    let ret = fyrox_lite::lite_scene::LiteScene::load_async(scene_path);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInput {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_mouse_button_down(button: i32) -> NativeBool {
    let button = button.into();
    let ret = fyrox_lite::lite_input::Input::is_mouse_button_down(button);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_mouse_button_up(button: i32) -> NativeBool {
    let button = button.into();
    let ret = fyrox_lite::lite_input::Input::is_mouse_button_up(button);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_mouse_button(button: i32) -> NativeBool {
    let button = button.into();
    let ret = fyrox_lite::lite_input::Input::is_mouse_button(button);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_key_down(key: NativeKeyCode) -> NativeBool {
    let key = key.into();
    let ret = fyrox_lite::lite_input::Input::is_key_down(key);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_key_up(key: NativeKeyCode) -> NativeBool {
    let key = key.into();
    let ret = fyrox_lite::lite_input::Input::is_key_up(key);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_is_key(key: NativeKeyCode) -> NativeBool {
    let key = key.into();
    let ret = fyrox_lite::lite_input::Input::is_key(key);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_get_mouse_move() -> NativeVector2 {
    let ret = fyrox_lite::lite_input::Input::get_mouse_move();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_input_Input_get_mouse_scroll() -> NativeVector2 {
    let ret = fyrox_lite::lite_input::Input::get_mouse_scroll();
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NativeKeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

impl From<fyrox_lite::lite_input::LiteKeyCode> for NativeKeyCode {
    fn from(value: fyrox_lite::lite_input::LiteKeyCode) -> Self {
        match value {
            fyrox_lite::lite_input::LiteKeyCode::Backquote => NativeKeyCode::Backquote,
            fyrox_lite::lite_input::LiteKeyCode::Backslash => NativeKeyCode::Backslash,
            fyrox_lite::lite_input::LiteKeyCode::BracketLeft => NativeKeyCode::BracketLeft,
            fyrox_lite::lite_input::LiteKeyCode::BracketRight => NativeKeyCode::BracketRight,
            fyrox_lite::lite_input::LiteKeyCode::Comma => NativeKeyCode::Comma,
            fyrox_lite::lite_input::LiteKeyCode::Digit0 => NativeKeyCode::Digit0,
            fyrox_lite::lite_input::LiteKeyCode::Digit1 => NativeKeyCode::Digit1,
            fyrox_lite::lite_input::LiteKeyCode::Digit2 => NativeKeyCode::Digit2,
            fyrox_lite::lite_input::LiteKeyCode::Digit3 => NativeKeyCode::Digit3,
            fyrox_lite::lite_input::LiteKeyCode::Digit4 => NativeKeyCode::Digit4,
            fyrox_lite::lite_input::LiteKeyCode::Digit5 => NativeKeyCode::Digit5,
            fyrox_lite::lite_input::LiteKeyCode::Digit6 => NativeKeyCode::Digit6,
            fyrox_lite::lite_input::LiteKeyCode::Digit7 => NativeKeyCode::Digit7,
            fyrox_lite::lite_input::LiteKeyCode::Digit8 => NativeKeyCode::Digit8,
            fyrox_lite::lite_input::LiteKeyCode::Digit9 => NativeKeyCode::Digit9,
            fyrox_lite::lite_input::LiteKeyCode::Equal => NativeKeyCode::Equal,
            fyrox_lite::lite_input::LiteKeyCode::IntlBackslash => NativeKeyCode::IntlBackslash,
            fyrox_lite::lite_input::LiteKeyCode::IntlRo => NativeKeyCode::IntlRo,
            fyrox_lite::lite_input::LiteKeyCode::IntlYen => NativeKeyCode::IntlYen,
            fyrox_lite::lite_input::LiteKeyCode::A => NativeKeyCode::A,
            fyrox_lite::lite_input::LiteKeyCode::B => NativeKeyCode::B,
            fyrox_lite::lite_input::LiteKeyCode::C => NativeKeyCode::C,
            fyrox_lite::lite_input::LiteKeyCode::D => NativeKeyCode::D,
            fyrox_lite::lite_input::LiteKeyCode::E => NativeKeyCode::E,
            fyrox_lite::lite_input::LiteKeyCode::F => NativeKeyCode::F,
            fyrox_lite::lite_input::LiteKeyCode::G => NativeKeyCode::G,
            fyrox_lite::lite_input::LiteKeyCode::H => NativeKeyCode::H,
            fyrox_lite::lite_input::LiteKeyCode::I => NativeKeyCode::I,
            fyrox_lite::lite_input::LiteKeyCode::J => NativeKeyCode::J,
            fyrox_lite::lite_input::LiteKeyCode::K => NativeKeyCode::K,
            fyrox_lite::lite_input::LiteKeyCode::L => NativeKeyCode::L,
            fyrox_lite::lite_input::LiteKeyCode::M => NativeKeyCode::M,
            fyrox_lite::lite_input::LiteKeyCode::N => NativeKeyCode::N,
            fyrox_lite::lite_input::LiteKeyCode::O => NativeKeyCode::O,
            fyrox_lite::lite_input::LiteKeyCode::P => NativeKeyCode::P,
            fyrox_lite::lite_input::LiteKeyCode::Q => NativeKeyCode::Q,
            fyrox_lite::lite_input::LiteKeyCode::R => NativeKeyCode::R,
            fyrox_lite::lite_input::LiteKeyCode::S => NativeKeyCode::S,
            fyrox_lite::lite_input::LiteKeyCode::T => NativeKeyCode::T,
            fyrox_lite::lite_input::LiteKeyCode::U => NativeKeyCode::U,
            fyrox_lite::lite_input::LiteKeyCode::V => NativeKeyCode::V,
            fyrox_lite::lite_input::LiteKeyCode::W => NativeKeyCode::W,
            fyrox_lite::lite_input::LiteKeyCode::X => NativeKeyCode::X,
            fyrox_lite::lite_input::LiteKeyCode::Y => NativeKeyCode::Y,
            fyrox_lite::lite_input::LiteKeyCode::Z => NativeKeyCode::Z,
            fyrox_lite::lite_input::LiteKeyCode::Minus => NativeKeyCode::Minus,
            fyrox_lite::lite_input::LiteKeyCode::Period => NativeKeyCode::Period,
            fyrox_lite::lite_input::LiteKeyCode::Quote => NativeKeyCode::Quote,
            fyrox_lite::lite_input::LiteKeyCode::Semicolon => NativeKeyCode::Semicolon,
            fyrox_lite::lite_input::LiteKeyCode::Slash => NativeKeyCode::Slash,
            fyrox_lite::lite_input::LiteKeyCode::AltLeft => NativeKeyCode::AltLeft,
            fyrox_lite::lite_input::LiteKeyCode::AltRight => NativeKeyCode::AltRight,
            fyrox_lite::lite_input::LiteKeyCode::Backspace => NativeKeyCode::Backspace,
            fyrox_lite::lite_input::LiteKeyCode::CapsLock => NativeKeyCode::CapsLock,
            fyrox_lite::lite_input::LiteKeyCode::ContextMenu => NativeKeyCode::ContextMenu,
            fyrox_lite::lite_input::LiteKeyCode::ControlLeft => NativeKeyCode::ControlLeft,
            fyrox_lite::lite_input::LiteKeyCode::ControlRight => NativeKeyCode::ControlRight,
            fyrox_lite::lite_input::LiteKeyCode::Enter => NativeKeyCode::Enter,
            fyrox_lite::lite_input::LiteKeyCode::SuperLeft => NativeKeyCode::SuperLeft,
            fyrox_lite::lite_input::LiteKeyCode::SuperRight => NativeKeyCode::SuperRight,
            fyrox_lite::lite_input::LiteKeyCode::ShiftLeft => NativeKeyCode::ShiftLeft,
            fyrox_lite::lite_input::LiteKeyCode::ShiftRight => NativeKeyCode::ShiftRight,
            fyrox_lite::lite_input::LiteKeyCode::Space => NativeKeyCode::Space,
            fyrox_lite::lite_input::LiteKeyCode::Tab => NativeKeyCode::Tab,
            fyrox_lite::lite_input::LiteKeyCode::Convert => NativeKeyCode::Convert,
            fyrox_lite::lite_input::LiteKeyCode::KanaMode => NativeKeyCode::KanaMode,
            fyrox_lite::lite_input::LiteKeyCode::Lang1 => NativeKeyCode::Lang1,
            fyrox_lite::lite_input::LiteKeyCode::Lang2 => NativeKeyCode::Lang2,
            fyrox_lite::lite_input::LiteKeyCode::Lang3 => NativeKeyCode::Lang3,
            fyrox_lite::lite_input::LiteKeyCode::Lang4 => NativeKeyCode::Lang4,
            fyrox_lite::lite_input::LiteKeyCode::Lang5 => NativeKeyCode::Lang5,
            fyrox_lite::lite_input::LiteKeyCode::NonConvert => NativeKeyCode::NonConvert,
            fyrox_lite::lite_input::LiteKeyCode::Delete => NativeKeyCode::Delete,
            fyrox_lite::lite_input::LiteKeyCode::End => NativeKeyCode::End,
            fyrox_lite::lite_input::LiteKeyCode::Help => NativeKeyCode::Help,
            fyrox_lite::lite_input::LiteKeyCode::Home => NativeKeyCode::Home,
            fyrox_lite::lite_input::LiteKeyCode::Insert => NativeKeyCode::Insert,
            fyrox_lite::lite_input::LiteKeyCode::PageDown => NativeKeyCode::PageDown,
            fyrox_lite::lite_input::LiteKeyCode::PageUp => NativeKeyCode::PageUp,
            fyrox_lite::lite_input::LiteKeyCode::ArrowDown => NativeKeyCode::ArrowDown,
            fyrox_lite::lite_input::LiteKeyCode::ArrowLeft => NativeKeyCode::ArrowLeft,
            fyrox_lite::lite_input::LiteKeyCode::ArrowRight => NativeKeyCode::ArrowRight,
            fyrox_lite::lite_input::LiteKeyCode::ArrowUp => NativeKeyCode::ArrowUp,
            fyrox_lite::lite_input::LiteKeyCode::NumLock => NativeKeyCode::NumLock,
            fyrox_lite::lite_input::LiteKeyCode::Numpad0 => NativeKeyCode::Numpad0,
            fyrox_lite::lite_input::LiteKeyCode::Numpad1 => NativeKeyCode::Numpad1,
            fyrox_lite::lite_input::LiteKeyCode::Numpad2 => NativeKeyCode::Numpad2,
            fyrox_lite::lite_input::LiteKeyCode::Numpad3 => NativeKeyCode::Numpad3,
            fyrox_lite::lite_input::LiteKeyCode::Numpad4 => NativeKeyCode::Numpad4,
            fyrox_lite::lite_input::LiteKeyCode::Numpad5 => NativeKeyCode::Numpad5,
            fyrox_lite::lite_input::LiteKeyCode::Numpad6 => NativeKeyCode::Numpad6,
            fyrox_lite::lite_input::LiteKeyCode::Numpad7 => NativeKeyCode::Numpad7,
            fyrox_lite::lite_input::LiteKeyCode::Numpad8 => NativeKeyCode::Numpad8,
            fyrox_lite::lite_input::LiteKeyCode::Numpad9 => NativeKeyCode::Numpad9,
            fyrox_lite::lite_input::LiteKeyCode::NumpadAdd => NativeKeyCode::NumpadAdd,
            fyrox_lite::lite_input::LiteKeyCode::NumpadBackspace => NativeKeyCode::NumpadBackspace,
            fyrox_lite::lite_input::LiteKeyCode::NumpadClear => NativeKeyCode::NumpadClear,
            fyrox_lite::lite_input::LiteKeyCode::NumpadClearEntry => {
                NativeKeyCode::NumpadClearEntry
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadComma => NativeKeyCode::NumpadComma,
            fyrox_lite::lite_input::LiteKeyCode::NumpadDecimal => NativeKeyCode::NumpadDecimal,
            fyrox_lite::lite_input::LiteKeyCode::NumpadDivide => NativeKeyCode::NumpadDivide,
            fyrox_lite::lite_input::LiteKeyCode::NumpadEnter => NativeKeyCode::NumpadEnter,
            fyrox_lite::lite_input::LiteKeyCode::NumpadEqual => NativeKeyCode::NumpadEqual,
            fyrox_lite::lite_input::LiteKeyCode::NumpadHash => NativeKeyCode::NumpadHash,
            fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryAdd => NativeKeyCode::NumpadMemoryAdd,
            fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryClear => {
                NativeKeyCode::NumpadMemoryClear
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryRecall => {
                NativeKeyCode::NumpadMemoryRecall
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryStore => {
                NativeKeyCode::NumpadMemoryStore
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadMemorySubtract => {
                NativeKeyCode::NumpadMemorySubtract
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadMultiply => NativeKeyCode::NumpadMultiply,
            fyrox_lite::lite_input::LiteKeyCode::NumpadParenLeft => NativeKeyCode::NumpadParenLeft,
            fyrox_lite::lite_input::LiteKeyCode::NumpadParenRight => {
                NativeKeyCode::NumpadParenRight
            }
            fyrox_lite::lite_input::LiteKeyCode::NumpadStar => NativeKeyCode::NumpadStar,
            fyrox_lite::lite_input::LiteKeyCode::NumpadSubtract => NativeKeyCode::NumpadSubtract,
            fyrox_lite::lite_input::LiteKeyCode::Escape => NativeKeyCode::Escape,
            fyrox_lite::lite_input::LiteKeyCode::Fn => NativeKeyCode::Fn,
            fyrox_lite::lite_input::LiteKeyCode::FnLock => NativeKeyCode::FnLock,
            fyrox_lite::lite_input::LiteKeyCode::PrintScreen => NativeKeyCode::PrintScreen,
            fyrox_lite::lite_input::LiteKeyCode::ScrollLock => NativeKeyCode::ScrollLock,
            fyrox_lite::lite_input::LiteKeyCode::Pause => NativeKeyCode::Pause,
            fyrox_lite::lite_input::LiteKeyCode::BrowserBack => NativeKeyCode::BrowserBack,
            fyrox_lite::lite_input::LiteKeyCode::BrowserFavorites => {
                NativeKeyCode::BrowserFavorites
            }
            fyrox_lite::lite_input::LiteKeyCode::BrowserForward => NativeKeyCode::BrowserForward,
            fyrox_lite::lite_input::LiteKeyCode::BrowserHome => NativeKeyCode::BrowserHome,
            fyrox_lite::lite_input::LiteKeyCode::BrowserRefresh => NativeKeyCode::BrowserRefresh,
            fyrox_lite::lite_input::LiteKeyCode::BrowserSearch => NativeKeyCode::BrowserSearch,
            fyrox_lite::lite_input::LiteKeyCode::BrowserStop => NativeKeyCode::BrowserStop,
            fyrox_lite::lite_input::LiteKeyCode::Eject => NativeKeyCode::Eject,
            fyrox_lite::lite_input::LiteKeyCode::LaunchApp1 => NativeKeyCode::LaunchApp1,
            fyrox_lite::lite_input::LiteKeyCode::LaunchApp2 => NativeKeyCode::LaunchApp2,
            fyrox_lite::lite_input::LiteKeyCode::LaunchMail => NativeKeyCode::LaunchMail,
            fyrox_lite::lite_input::LiteKeyCode::MediaPlayPause => NativeKeyCode::MediaPlayPause,
            fyrox_lite::lite_input::LiteKeyCode::MediaSelect => NativeKeyCode::MediaSelect,
            fyrox_lite::lite_input::LiteKeyCode::MediaStop => NativeKeyCode::MediaStop,
            fyrox_lite::lite_input::LiteKeyCode::MediaTrackNext => NativeKeyCode::MediaTrackNext,
            fyrox_lite::lite_input::LiteKeyCode::MediaTrackPrevious => {
                NativeKeyCode::MediaTrackPrevious
            }
            fyrox_lite::lite_input::LiteKeyCode::Power => NativeKeyCode::Power,
            fyrox_lite::lite_input::LiteKeyCode::Sleep => NativeKeyCode::Sleep,
            fyrox_lite::lite_input::LiteKeyCode::AudioVolumeDown => NativeKeyCode::AudioVolumeDown,
            fyrox_lite::lite_input::LiteKeyCode::AudioVolumeMute => NativeKeyCode::AudioVolumeMute,
            fyrox_lite::lite_input::LiteKeyCode::AudioVolumeUp => NativeKeyCode::AudioVolumeUp,
            fyrox_lite::lite_input::LiteKeyCode::WakeUp => NativeKeyCode::WakeUp,
            fyrox_lite::lite_input::LiteKeyCode::Meta => NativeKeyCode::Meta,
            fyrox_lite::lite_input::LiteKeyCode::Hyper => NativeKeyCode::Hyper,
            fyrox_lite::lite_input::LiteKeyCode::Turbo => NativeKeyCode::Turbo,
            fyrox_lite::lite_input::LiteKeyCode::Abort => NativeKeyCode::Abort,
            fyrox_lite::lite_input::LiteKeyCode::Resume => NativeKeyCode::Resume,
            fyrox_lite::lite_input::LiteKeyCode::Suspend => NativeKeyCode::Suspend,
            fyrox_lite::lite_input::LiteKeyCode::Again => NativeKeyCode::Again,
            fyrox_lite::lite_input::LiteKeyCode::Copy => NativeKeyCode::Copy,
            fyrox_lite::lite_input::LiteKeyCode::Cut => NativeKeyCode::Cut,
            fyrox_lite::lite_input::LiteKeyCode::Find => NativeKeyCode::Find,
            fyrox_lite::lite_input::LiteKeyCode::Open => NativeKeyCode::Open,
            fyrox_lite::lite_input::LiteKeyCode::Paste => NativeKeyCode::Paste,
            fyrox_lite::lite_input::LiteKeyCode::Props => NativeKeyCode::Props,
            fyrox_lite::lite_input::LiteKeyCode::Select => NativeKeyCode::Select,
            fyrox_lite::lite_input::LiteKeyCode::Undo => NativeKeyCode::Undo,
            fyrox_lite::lite_input::LiteKeyCode::Hiragana => NativeKeyCode::Hiragana,
            fyrox_lite::lite_input::LiteKeyCode::Katakana => NativeKeyCode::Katakana,
            fyrox_lite::lite_input::LiteKeyCode::F1 => NativeKeyCode::F1,
            fyrox_lite::lite_input::LiteKeyCode::F2 => NativeKeyCode::F2,
            fyrox_lite::lite_input::LiteKeyCode::F3 => NativeKeyCode::F3,
            fyrox_lite::lite_input::LiteKeyCode::F4 => NativeKeyCode::F4,
            fyrox_lite::lite_input::LiteKeyCode::F5 => NativeKeyCode::F5,
            fyrox_lite::lite_input::LiteKeyCode::F6 => NativeKeyCode::F6,
            fyrox_lite::lite_input::LiteKeyCode::F7 => NativeKeyCode::F7,
            fyrox_lite::lite_input::LiteKeyCode::F8 => NativeKeyCode::F8,
            fyrox_lite::lite_input::LiteKeyCode::F9 => NativeKeyCode::F9,
            fyrox_lite::lite_input::LiteKeyCode::F10 => NativeKeyCode::F10,
            fyrox_lite::lite_input::LiteKeyCode::F11 => NativeKeyCode::F11,
            fyrox_lite::lite_input::LiteKeyCode::F12 => NativeKeyCode::F12,
            fyrox_lite::lite_input::LiteKeyCode::F13 => NativeKeyCode::F13,
            fyrox_lite::lite_input::LiteKeyCode::F14 => NativeKeyCode::F14,
            fyrox_lite::lite_input::LiteKeyCode::F15 => NativeKeyCode::F15,
            fyrox_lite::lite_input::LiteKeyCode::F16 => NativeKeyCode::F16,
            fyrox_lite::lite_input::LiteKeyCode::F17 => NativeKeyCode::F17,
            fyrox_lite::lite_input::LiteKeyCode::F18 => NativeKeyCode::F18,
            fyrox_lite::lite_input::LiteKeyCode::F19 => NativeKeyCode::F19,
            fyrox_lite::lite_input::LiteKeyCode::F20 => NativeKeyCode::F20,
            fyrox_lite::lite_input::LiteKeyCode::F21 => NativeKeyCode::F21,
            fyrox_lite::lite_input::LiteKeyCode::F22 => NativeKeyCode::F22,
            fyrox_lite::lite_input::LiteKeyCode::F23 => NativeKeyCode::F23,
            fyrox_lite::lite_input::LiteKeyCode::F24 => NativeKeyCode::F24,
            fyrox_lite::lite_input::LiteKeyCode::F25 => NativeKeyCode::F25,
            fyrox_lite::lite_input::LiteKeyCode::F26 => NativeKeyCode::F26,
            fyrox_lite::lite_input::LiteKeyCode::F27 => NativeKeyCode::F27,
            fyrox_lite::lite_input::LiteKeyCode::F28 => NativeKeyCode::F28,
            fyrox_lite::lite_input::LiteKeyCode::F29 => NativeKeyCode::F29,
            fyrox_lite::lite_input::LiteKeyCode::F30 => NativeKeyCode::F30,
            fyrox_lite::lite_input::LiteKeyCode::F31 => NativeKeyCode::F31,
            fyrox_lite::lite_input::LiteKeyCode::F32 => NativeKeyCode::F32,
            fyrox_lite::lite_input::LiteKeyCode::F33 => NativeKeyCode::F33,
            fyrox_lite::lite_input::LiteKeyCode::F34 => NativeKeyCode::F34,
            fyrox_lite::lite_input::LiteKeyCode::F35 => NativeKeyCode::F35,
        }
    }
}

impl From<NativeKeyCode> for fyrox_lite::lite_input::LiteKeyCode {
    fn from(value: NativeKeyCode) -> Self {
        match value {
            NativeKeyCode::Backquote => fyrox_lite::lite_input::LiteKeyCode::Backquote,
            NativeKeyCode::Backslash => fyrox_lite::lite_input::LiteKeyCode::Backslash,
            NativeKeyCode::BracketLeft => fyrox_lite::lite_input::LiteKeyCode::BracketLeft,
            NativeKeyCode::BracketRight => fyrox_lite::lite_input::LiteKeyCode::BracketRight,
            NativeKeyCode::Comma => fyrox_lite::lite_input::LiteKeyCode::Comma,
            NativeKeyCode::Digit0 => fyrox_lite::lite_input::LiteKeyCode::Digit0,
            NativeKeyCode::Digit1 => fyrox_lite::lite_input::LiteKeyCode::Digit1,
            NativeKeyCode::Digit2 => fyrox_lite::lite_input::LiteKeyCode::Digit2,
            NativeKeyCode::Digit3 => fyrox_lite::lite_input::LiteKeyCode::Digit3,
            NativeKeyCode::Digit4 => fyrox_lite::lite_input::LiteKeyCode::Digit4,
            NativeKeyCode::Digit5 => fyrox_lite::lite_input::LiteKeyCode::Digit5,
            NativeKeyCode::Digit6 => fyrox_lite::lite_input::LiteKeyCode::Digit6,
            NativeKeyCode::Digit7 => fyrox_lite::lite_input::LiteKeyCode::Digit7,
            NativeKeyCode::Digit8 => fyrox_lite::lite_input::LiteKeyCode::Digit8,
            NativeKeyCode::Digit9 => fyrox_lite::lite_input::LiteKeyCode::Digit9,
            NativeKeyCode::Equal => fyrox_lite::lite_input::LiteKeyCode::Equal,
            NativeKeyCode::IntlBackslash => fyrox_lite::lite_input::LiteKeyCode::IntlBackslash,
            NativeKeyCode::IntlRo => fyrox_lite::lite_input::LiteKeyCode::IntlRo,
            NativeKeyCode::IntlYen => fyrox_lite::lite_input::LiteKeyCode::IntlYen,
            NativeKeyCode::A => fyrox_lite::lite_input::LiteKeyCode::A,
            NativeKeyCode::B => fyrox_lite::lite_input::LiteKeyCode::B,
            NativeKeyCode::C => fyrox_lite::lite_input::LiteKeyCode::C,
            NativeKeyCode::D => fyrox_lite::lite_input::LiteKeyCode::D,
            NativeKeyCode::E => fyrox_lite::lite_input::LiteKeyCode::E,
            NativeKeyCode::F => fyrox_lite::lite_input::LiteKeyCode::F,
            NativeKeyCode::G => fyrox_lite::lite_input::LiteKeyCode::G,
            NativeKeyCode::H => fyrox_lite::lite_input::LiteKeyCode::H,
            NativeKeyCode::I => fyrox_lite::lite_input::LiteKeyCode::I,
            NativeKeyCode::J => fyrox_lite::lite_input::LiteKeyCode::J,
            NativeKeyCode::K => fyrox_lite::lite_input::LiteKeyCode::K,
            NativeKeyCode::L => fyrox_lite::lite_input::LiteKeyCode::L,
            NativeKeyCode::M => fyrox_lite::lite_input::LiteKeyCode::M,
            NativeKeyCode::N => fyrox_lite::lite_input::LiteKeyCode::N,
            NativeKeyCode::O => fyrox_lite::lite_input::LiteKeyCode::O,
            NativeKeyCode::P => fyrox_lite::lite_input::LiteKeyCode::P,
            NativeKeyCode::Q => fyrox_lite::lite_input::LiteKeyCode::Q,
            NativeKeyCode::R => fyrox_lite::lite_input::LiteKeyCode::R,
            NativeKeyCode::S => fyrox_lite::lite_input::LiteKeyCode::S,
            NativeKeyCode::T => fyrox_lite::lite_input::LiteKeyCode::T,
            NativeKeyCode::U => fyrox_lite::lite_input::LiteKeyCode::U,
            NativeKeyCode::V => fyrox_lite::lite_input::LiteKeyCode::V,
            NativeKeyCode::W => fyrox_lite::lite_input::LiteKeyCode::W,
            NativeKeyCode::X => fyrox_lite::lite_input::LiteKeyCode::X,
            NativeKeyCode::Y => fyrox_lite::lite_input::LiteKeyCode::Y,
            NativeKeyCode::Z => fyrox_lite::lite_input::LiteKeyCode::Z,
            NativeKeyCode::Minus => fyrox_lite::lite_input::LiteKeyCode::Minus,
            NativeKeyCode::Period => fyrox_lite::lite_input::LiteKeyCode::Period,
            NativeKeyCode::Quote => fyrox_lite::lite_input::LiteKeyCode::Quote,
            NativeKeyCode::Semicolon => fyrox_lite::lite_input::LiteKeyCode::Semicolon,
            NativeKeyCode::Slash => fyrox_lite::lite_input::LiteKeyCode::Slash,
            NativeKeyCode::AltLeft => fyrox_lite::lite_input::LiteKeyCode::AltLeft,
            NativeKeyCode::AltRight => fyrox_lite::lite_input::LiteKeyCode::AltRight,
            NativeKeyCode::Backspace => fyrox_lite::lite_input::LiteKeyCode::Backspace,
            NativeKeyCode::CapsLock => fyrox_lite::lite_input::LiteKeyCode::CapsLock,
            NativeKeyCode::ContextMenu => fyrox_lite::lite_input::LiteKeyCode::ContextMenu,
            NativeKeyCode::ControlLeft => fyrox_lite::lite_input::LiteKeyCode::ControlLeft,
            NativeKeyCode::ControlRight => fyrox_lite::lite_input::LiteKeyCode::ControlRight,
            NativeKeyCode::Enter => fyrox_lite::lite_input::LiteKeyCode::Enter,
            NativeKeyCode::SuperLeft => fyrox_lite::lite_input::LiteKeyCode::SuperLeft,
            NativeKeyCode::SuperRight => fyrox_lite::lite_input::LiteKeyCode::SuperRight,
            NativeKeyCode::ShiftLeft => fyrox_lite::lite_input::LiteKeyCode::ShiftLeft,
            NativeKeyCode::ShiftRight => fyrox_lite::lite_input::LiteKeyCode::ShiftRight,
            NativeKeyCode::Space => fyrox_lite::lite_input::LiteKeyCode::Space,
            NativeKeyCode::Tab => fyrox_lite::lite_input::LiteKeyCode::Tab,
            NativeKeyCode::Convert => fyrox_lite::lite_input::LiteKeyCode::Convert,
            NativeKeyCode::KanaMode => fyrox_lite::lite_input::LiteKeyCode::KanaMode,
            NativeKeyCode::Lang1 => fyrox_lite::lite_input::LiteKeyCode::Lang1,
            NativeKeyCode::Lang2 => fyrox_lite::lite_input::LiteKeyCode::Lang2,
            NativeKeyCode::Lang3 => fyrox_lite::lite_input::LiteKeyCode::Lang3,
            NativeKeyCode::Lang4 => fyrox_lite::lite_input::LiteKeyCode::Lang4,
            NativeKeyCode::Lang5 => fyrox_lite::lite_input::LiteKeyCode::Lang5,
            NativeKeyCode::NonConvert => fyrox_lite::lite_input::LiteKeyCode::NonConvert,
            NativeKeyCode::Delete => fyrox_lite::lite_input::LiteKeyCode::Delete,
            NativeKeyCode::End => fyrox_lite::lite_input::LiteKeyCode::End,
            NativeKeyCode::Help => fyrox_lite::lite_input::LiteKeyCode::Help,
            NativeKeyCode::Home => fyrox_lite::lite_input::LiteKeyCode::Home,
            NativeKeyCode::Insert => fyrox_lite::lite_input::LiteKeyCode::Insert,
            NativeKeyCode::PageDown => fyrox_lite::lite_input::LiteKeyCode::PageDown,
            NativeKeyCode::PageUp => fyrox_lite::lite_input::LiteKeyCode::PageUp,
            NativeKeyCode::ArrowDown => fyrox_lite::lite_input::LiteKeyCode::ArrowDown,
            NativeKeyCode::ArrowLeft => fyrox_lite::lite_input::LiteKeyCode::ArrowLeft,
            NativeKeyCode::ArrowRight => fyrox_lite::lite_input::LiteKeyCode::ArrowRight,
            NativeKeyCode::ArrowUp => fyrox_lite::lite_input::LiteKeyCode::ArrowUp,
            NativeKeyCode::NumLock => fyrox_lite::lite_input::LiteKeyCode::NumLock,
            NativeKeyCode::Numpad0 => fyrox_lite::lite_input::LiteKeyCode::Numpad0,
            NativeKeyCode::Numpad1 => fyrox_lite::lite_input::LiteKeyCode::Numpad1,
            NativeKeyCode::Numpad2 => fyrox_lite::lite_input::LiteKeyCode::Numpad2,
            NativeKeyCode::Numpad3 => fyrox_lite::lite_input::LiteKeyCode::Numpad3,
            NativeKeyCode::Numpad4 => fyrox_lite::lite_input::LiteKeyCode::Numpad4,
            NativeKeyCode::Numpad5 => fyrox_lite::lite_input::LiteKeyCode::Numpad5,
            NativeKeyCode::Numpad6 => fyrox_lite::lite_input::LiteKeyCode::Numpad6,
            NativeKeyCode::Numpad7 => fyrox_lite::lite_input::LiteKeyCode::Numpad7,
            NativeKeyCode::Numpad8 => fyrox_lite::lite_input::LiteKeyCode::Numpad8,
            NativeKeyCode::Numpad9 => fyrox_lite::lite_input::LiteKeyCode::Numpad9,
            NativeKeyCode::NumpadAdd => fyrox_lite::lite_input::LiteKeyCode::NumpadAdd,
            NativeKeyCode::NumpadBackspace => fyrox_lite::lite_input::LiteKeyCode::NumpadBackspace,
            NativeKeyCode::NumpadClear => fyrox_lite::lite_input::LiteKeyCode::NumpadClear,
            NativeKeyCode::NumpadClearEntry => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadClearEntry
            }
            NativeKeyCode::NumpadComma => fyrox_lite::lite_input::LiteKeyCode::NumpadComma,
            NativeKeyCode::NumpadDecimal => fyrox_lite::lite_input::LiteKeyCode::NumpadDecimal,
            NativeKeyCode::NumpadDivide => fyrox_lite::lite_input::LiteKeyCode::NumpadDivide,
            NativeKeyCode::NumpadEnter => fyrox_lite::lite_input::LiteKeyCode::NumpadEnter,
            NativeKeyCode::NumpadEqual => fyrox_lite::lite_input::LiteKeyCode::NumpadEqual,
            NativeKeyCode::NumpadHash => fyrox_lite::lite_input::LiteKeyCode::NumpadHash,
            NativeKeyCode::NumpadMemoryAdd => fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryAdd,
            NativeKeyCode::NumpadMemoryClear => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryClear
            }
            NativeKeyCode::NumpadMemoryRecall => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryRecall
            }
            NativeKeyCode::NumpadMemoryStore => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryStore
            }
            NativeKeyCode::NumpadMemorySubtract => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemorySubtract
            }
            NativeKeyCode::NumpadMultiply => fyrox_lite::lite_input::LiteKeyCode::NumpadMultiply,
            NativeKeyCode::NumpadParenLeft => fyrox_lite::lite_input::LiteKeyCode::NumpadParenLeft,
            NativeKeyCode::NumpadParenRight => {
                fyrox_lite::lite_input::LiteKeyCode::NumpadParenRight
            }
            NativeKeyCode::NumpadStar => fyrox_lite::lite_input::LiteKeyCode::NumpadStar,
            NativeKeyCode::NumpadSubtract => fyrox_lite::lite_input::LiteKeyCode::NumpadSubtract,
            NativeKeyCode::Escape => fyrox_lite::lite_input::LiteKeyCode::Escape,
            NativeKeyCode::Fn => fyrox_lite::lite_input::LiteKeyCode::Fn,
            NativeKeyCode::FnLock => fyrox_lite::lite_input::LiteKeyCode::FnLock,
            NativeKeyCode::PrintScreen => fyrox_lite::lite_input::LiteKeyCode::PrintScreen,
            NativeKeyCode::ScrollLock => fyrox_lite::lite_input::LiteKeyCode::ScrollLock,
            NativeKeyCode::Pause => fyrox_lite::lite_input::LiteKeyCode::Pause,
            NativeKeyCode::BrowserBack => fyrox_lite::lite_input::LiteKeyCode::BrowserBack,
            NativeKeyCode::BrowserFavorites => {
                fyrox_lite::lite_input::LiteKeyCode::BrowserFavorites
            }
            NativeKeyCode::BrowserForward => fyrox_lite::lite_input::LiteKeyCode::BrowserForward,
            NativeKeyCode::BrowserHome => fyrox_lite::lite_input::LiteKeyCode::BrowserHome,
            NativeKeyCode::BrowserRefresh => fyrox_lite::lite_input::LiteKeyCode::BrowserRefresh,
            NativeKeyCode::BrowserSearch => fyrox_lite::lite_input::LiteKeyCode::BrowserSearch,
            NativeKeyCode::BrowserStop => fyrox_lite::lite_input::LiteKeyCode::BrowserStop,
            NativeKeyCode::Eject => fyrox_lite::lite_input::LiteKeyCode::Eject,
            NativeKeyCode::LaunchApp1 => fyrox_lite::lite_input::LiteKeyCode::LaunchApp1,
            NativeKeyCode::LaunchApp2 => fyrox_lite::lite_input::LiteKeyCode::LaunchApp2,
            NativeKeyCode::LaunchMail => fyrox_lite::lite_input::LiteKeyCode::LaunchMail,
            NativeKeyCode::MediaPlayPause => fyrox_lite::lite_input::LiteKeyCode::MediaPlayPause,
            NativeKeyCode::MediaSelect => fyrox_lite::lite_input::LiteKeyCode::MediaSelect,
            NativeKeyCode::MediaStop => fyrox_lite::lite_input::LiteKeyCode::MediaStop,
            NativeKeyCode::MediaTrackNext => fyrox_lite::lite_input::LiteKeyCode::MediaTrackNext,
            NativeKeyCode::MediaTrackPrevious => {
                fyrox_lite::lite_input::LiteKeyCode::MediaTrackPrevious
            }
            NativeKeyCode::Power => fyrox_lite::lite_input::LiteKeyCode::Power,
            NativeKeyCode::Sleep => fyrox_lite::lite_input::LiteKeyCode::Sleep,
            NativeKeyCode::AudioVolumeDown => fyrox_lite::lite_input::LiteKeyCode::AudioVolumeDown,
            NativeKeyCode::AudioVolumeMute => fyrox_lite::lite_input::LiteKeyCode::AudioVolumeMute,
            NativeKeyCode::AudioVolumeUp => fyrox_lite::lite_input::LiteKeyCode::AudioVolumeUp,
            NativeKeyCode::WakeUp => fyrox_lite::lite_input::LiteKeyCode::WakeUp,
            NativeKeyCode::Meta => fyrox_lite::lite_input::LiteKeyCode::Meta,
            NativeKeyCode::Hyper => fyrox_lite::lite_input::LiteKeyCode::Hyper,
            NativeKeyCode::Turbo => fyrox_lite::lite_input::LiteKeyCode::Turbo,
            NativeKeyCode::Abort => fyrox_lite::lite_input::LiteKeyCode::Abort,
            NativeKeyCode::Resume => fyrox_lite::lite_input::LiteKeyCode::Resume,
            NativeKeyCode::Suspend => fyrox_lite::lite_input::LiteKeyCode::Suspend,
            NativeKeyCode::Again => fyrox_lite::lite_input::LiteKeyCode::Again,
            NativeKeyCode::Copy => fyrox_lite::lite_input::LiteKeyCode::Copy,
            NativeKeyCode::Cut => fyrox_lite::lite_input::LiteKeyCode::Cut,
            NativeKeyCode::Find => fyrox_lite::lite_input::LiteKeyCode::Find,
            NativeKeyCode::Open => fyrox_lite::lite_input::LiteKeyCode::Open,
            NativeKeyCode::Paste => fyrox_lite::lite_input::LiteKeyCode::Paste,
            NativeKeyCode::Props => fyrox_lite::lite_input::LiteKeyCode::Props,
            NativeKeyCode::Select => fyrox_lite::lite_input::LiteKeyCode::Select,
            NativeKeyCode::Undo => fyrox_lite::lite_input::LiteKeyCode::Undo,
            NativeKeyCode::Hiragana => fyrox_lite::lite_input::LiteKeyCode::Hiragana,
            NativeKeyCode::Katakana => fyrox_lite::lite_input::LiteKeyCode::Katakana,
            NativeKeyCode::F1 => fyrox_lite::lite_input::LiteKeyCode::F1,
            NativeKeyCode::F2 => fyrox_lite::lite_input::LiteKeyCode::F2,
            NativeKeyCode::F3 => fyrox_lite::lite_input::LiteKeyCode::F3,
            NativeKeyCode::F4 => fyrox_lite::lite_input::LiteKeyCode::F4,
            NativeKeyCode::F5 => fyrox_lite::lite_input::LiteKeyCode::F5,
            NativeKeyCode::F6 => fyrox_lite::lite_input::LiteKeyCode::F6,
            NativeKeyCode::F7 => fyrox_lite::lite_input::LiteKeyCode::F7,
            NativeKeyCode::F8 => fyrox_lite::lite_input::LiteKeyCode::F8,
            NativeKeyCode::F9 => fyrox_lite::lite_input::LiteKeyCode::F9,
            NativeKeyCode::F10 => fyrox_lite::lite_input::LiteKeyCode::F10,
            NativeKeyCode::F11 => fyrox_lite::lite_input::LiteKeyCode::F11,
            NativeKeyCode::F12 => fyrox_lite::lite_input::LiteKeyCode::F12,
            NativeKeyCode::F13 => fyrox_lite::lite_input::LiteKeyCode::F13,
            NativeKeyCode::F14 => fyrox_lite::lite_input::LiteKeyCode::F14,
            NativeKeyCode::F15 => fyrox_lite::lite_input::LiteKeyCode::F15,
            NativeKeyCode::F16 => fyrox_lite::lite_input::LiteKeyCode::F16,
            NativeKeyCode::F17 => fyrox_lite::lite_input::LiteKeyCode::F17,
            NativeKeyCode::F18 => fyrox_lite::lite_input::LiteKeyCode::F18,
            NativeKeyCode::F19 => fyrox_lite::lite_input::LiteKeyCode::F19,
            NativeKeyCode::F20 => fyrox_lite::lite_input::LiteKeyCode::F20,
            NativeKeyCode::F21 => fyrox_lite::lite_input::LiteKeyCode::F21,
            NativeKeyCode::F22 => fyrox_lite::lite_input::LiteKeyCode::F22,
            NativeKeyCode::F23 => fyrox_lite::lite_input::LiteKeyCode::F23,
            NativeKeyCode::F24 => fyrox_lite::lite_input::LiteKeyCode::F24,
            NativeKeyCode::F25 => fyrox_lite::lite_input::LiteKeyCode::F25,
            NativeKeyCode::F26 => fyrox_lite::lite_input::LiteKeyCode::F26,
            NativeKeyCode::F27 => fyrox_lite::lite_input::LiteKeyCode::F27,
            NativeKeyCode::F28 => fyrox_lite::lite_input::LiteKeyCode::F28,
            NativeKeyCode::F29 => fyrox_lite::lite_input::LiteKeyCode::F29,
            NativeKeyCode::F30 => fyrox_lite::lite_input::LiteKeyCode::F30,
            NativeKeyCode::F31 => fyrox_lite::lite_input::LiteKeyCode::F31,
            NativeKeyCode::F32 => fyrox_lite::lite_input::LiteKeyCode::F32,
            NativeKeyCode::F33 => fyrox_lite::lite_input::LiteKeyCode::F33,
            NativeKeyCode::F34 => fyrox_lite::lite_input::LiteKeyCode::F34,
            NativeKeyCode::F35 => fyrox_lite::lite_input::LiteKeyCode::F35,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLog {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_log_LiteLog_info(msg: NativeString) -> () {
    let msg = msg.into();
    let ret = fyrox_lite::lite_log::LiteLog::info(msg);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_log_LiteLog_warn(msg: NativeString) -> () {
    let msg = msg.into();
    let ret = fyrox_lite::lite_log::LiteLog::warn(msg);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_log_LiteLog_err(msg: NativeString) -> () {
    let msg = msg.into();
    let ret = fyrox_lite::lite_log::LiteLog::err(msg);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeUiNode {
    pub handle: NativeHandle,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeText {
    pub handle: NativeHandle,
}

impl From<fyrox_lite::lite_ui::LiteText> for NativeText {
    fn from(value: fyrox_lite::lite_ui::LiteText) -> Self {
        Self {
            handle: NativeHandle::from_u128(value.to_external()),
        }
    }
}

impl From<NativeText> for fyrox_lite::lite_ui::LiteText {
    fn from(value: NativeText) -> Self {
        Self::from_external(value.handle.as_u128())
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_ui_LiteText_set_text_async(
    this: NativeText,
    text: NativeString,
) -> () {
    let text = text.into();
    let ret = fyrox_lite::lite_ui::LiteText::from(this).set_text_async(text);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_ui_LiteText_new(state: NativeTextBuilder) -> NativeText {
    let state = state.into();
    let ret = fyrox_lite::lite_ui::LiteText::new(state);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeText_optional {
    pub value: NativeText,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::LiteText>> for NativeText_optional {
    fn from(value: Option<fyrox_lite::lite_ui::LiteText>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeText_optional> for Option<fyrox_lite::lite_ui::LiteText> {
    fn from(value: NativeText_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeText_result {
    pub ok: i32,
    pub value: NativeText_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeText_result_value {
    ok: NativeText,
    err: NativeString,
}

impl NativeText_result {
    pub fn into_result_shallow(self) -> Result<NativeText, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<fyrox_lite::lite_ui::LiteText, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::LiteText, crate::LangSpecificError>> for NativeText_result {
    fn from(value: Result<fyrox_lite::lite_ui::LiteText, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeText_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeText_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeText_result> for Result<fyrox_lite::lite_ui::LiteText, crate::LangSpecificError> {
    fn from(value: NativeText_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeText_optional_result {
    pub ok: i32,
    pub value: NativeText_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeText_optional_result_value {
    ok: NativeText_optional,
    err: NativeString,
}

impl NativeText_optional_result {
    pub fn into_result_shallow(self) -> Result<NativeText_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<Option<fyrox_lite::lite_ui::LiteText>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<fyrox_lite::lite_ui::LiteText>, crate::LangSpecificError>>
    for NativeText_optional_result
{
    fn from(
        value: Result<Option<fyrox_lite::lite_ui::LiteText>, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeText_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeText_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeText_optional_result>
    for Result<Option<fyrox_lite::lite_ui::LiteText>, crate::LangSpecificError>
{
    fn from(value: NativeText_optional_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<fyrox_lite::lite_ui::Color> for NativeColor {
    fn from(__value: fyrox_lite::lite_ui::Color) -> Self {
        let r = __value.r.into();
        let g = __value.g.into();
        let b = __value.b.into();
        let a = __value.a.into();
        Self { r, g, b, a }
    }
}

impl From<NativeColor> for fyrox_lite::lite_ui::Color {
    fn from(__value: NativeColor) -> Self {
        let r = __value.r.into();
        let g = __value.g.into();
        let b = __value.b.into();
        let a = __value.a.into();
        Self { r, g, b, a }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeColor_optional {
    pub value: NativeColor,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::Color>> for NativeColor_optional {
    fn from(value: Option<fyrox_lite::lite_ui::Color>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeColor_optional> for Option<fyrox_lite::lite_ui::Color> {
    fn from(value: NativeColor_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeColor_slice {
    pub begin: *mut NativeColor,
    pub len: i32,
}

impl Default for NativeColor_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::Color>> for NativeColor_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::Color>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeColor> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeColor_slice> for Vec<fyrox_lite::lite_ui::Color> {
    fn from(value: NativeColor_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_Color_slice(
    data: NativeColor_slice,
) -> NativeColor_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeColor_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeColor_result {
    pub ok: i32,
    pub value: NativeColor_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeColor_result_value {
    ok: NativeColor,
    err: NativeString,
}

impl NativeColor_result {
    pub fn into_result_shallow(self) -> Result<NativeColor, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<fyrox_lite::lite_ui::Color, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::Color, crate::LangSpecificError>> for NativeColor_result {
    fn from(value: Result<fyrox_lite::lite_ui::Color, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeColor_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeColor_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeColor_result> for Result<fyrox_lite::lite_ui::Color, crate::LangSpecificError> {
    fn from(value: NativeColor_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTextBuilder {
    pub foreground: NativeBrush_optional,
    pub font_size: f32_optional,
}

impl From<fyrox_lite::lite_ui::TextBuilder> for NativeTextBuilder {
    fn from(__value: fyrox_lite::lite_ui::TextBuilder) -> Self {
        let foreground = __value.foreground.into();
        let font_size = __value.font_size.into();
        Self {
            foreground,
            font_size,
        }
    }
}

impl From<NativeTextBuilder> for fyrox_lite::lite_ui::TextBuilder {
    fn from(__value: NativeTextBuilder) -> Self {
        let foreground = __value.foreground.into();
        let font_size = __value.font_size.into();
        Self {
            foreground,
            font_size,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTextBuilder_optional {
    pub value: NativeTextBuilder,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::TextBuilder>> for NativeTextBuilder_optional {
    fn from(value: Option<fyrox_lite::lite_ui::TextBuilder>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeTextBuilder_optional> for Option<fyrox_lite::lite_ui::TextBuilder> {
    fn from(value: NativeTextBuilder_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTextBuilder_slice {
    pub begin: *mut NativeTextBuilder,
    pub len: i32,
}

impl Default for NativeTextBuilder_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::TextBuilder>> for NativeTextBuilder_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::TextBuilder>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeTextBuilder> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeTextBuilder_slice> for Vec<fyrox_lite::lite_ui::TextBuilder> {
    fn from(value: NativeTextBuilder_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_TextBuilder_slice(
    data: NativeTextBuilder_slice,
) -> NativeTextBuilder_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeTextBuilder_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeTextBuilder_result {
    pub ok: i32,
    pub value: NativeTextBuilder_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeTextBuilder_result_value {
    ok: NativeTextBuilder,
    err: NativeString,
}

impl NativeTextBuilder_result {
    pub fn into_result_shallow(self) -> Result<NativeTextBuilder, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<fyrox_lite::lite_ui::TextBuilder, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::TextBuilder, crate::LangSpecificError>>
    for NativeTextBuilder_result
{
    fn from(value: Result<fyrox_lite::lite_ui::TextBuilder, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeTextBuilder_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeTextBuilder_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeTextBuilder_result>
    for Result<fyrox_lite::lite_ui::TextBuilder, crate::LangSpecificError>
{
    fn from(value: NativeTextBuilder_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush {
    pub solid_color: NativeColor_optional,
    pub linear_gradient: NativeLinearGradient_optional,
    pub radial_gradient: NativeRadialGradient_optional,
}

impl From<fyrox_lite::lite_ui::Brush> for NativeBrush {
    fn from(__value: fyrox_lite::lite_ui::Brush) -> Self {
        let solid_color = __value.solid_color.into();
        let linear_gradient = __value.linear_gradient.into();
        let radial_gradient = __value.radial_gradient.into();
        Self {
            solid_color,
            linear_gradient,
            radial_gradient,
        }
    }
}

impl From<NativeBrush> for fyrox_lite::lite_ui::Brush {
    fn from(__value: NativeBrush) -> Self {
        let solid_color = __value.solid_color.into();
        let linear_gradient = __value.linear_gradient.into();
        let radial_gradient = __value.radial_gradient.into();
        Self {
            solid_color,
            linear_gradient,
            radial_gradient,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_optional {
    pub value: NativeBrush,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::Brush>> for NativeBrush_optional {
    fn from(value: Option<fyrox_lite::lite_ui::Brush>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeBrush_optional> for Option<fyrox_lite::lite_ui::Brush> {
    fn from(value: NativeBrush_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_slice {
    pub begin: *mut NativeBrush,
    pub len: i32,
}

impl Default for NativeBrush_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::Brush>> for NativeBrush_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::Brush>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeBrush> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeBrush_slice> for Vec<fyrox_lite::lite_ui::Brush> {
    fn from(value: NativeBrush_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_Brush_slice(
    data: NativeBrush_slice,
) -> NativeBrush_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeBrush_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBrush_result {
    pub ok: i32,
    pub value: NativeBrush_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeBrush_result_value {
    ok: NativeBrush,
    err: NativeString,
}

impl NativeBrush_result {
    pub fn into_result_shallow(self) -> Result<NativeBrush, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<fyrox_lite::lite_ui::Brush, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::Brush, crate::LangSpecificError>> for NativeBrush_result {
    fn from(value: Result<fyrox_lite::lite_ui::Brush, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeBrush_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeBrush_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeBrush_result> for Result<fyrox_lite::lite_ui::Brush, crate::LangSpecificError> {
    fn from(value: NativeBrush_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLinearGradient {
    pub from: NativeVector2,
    pub to: NativeVector2,
    pub stops: NativeGradientPoint_slice,
}

impl From<fyrox_lite::lite_ui::LinearGradient> for NativeLinearGradient {
    fn from(__value: fyrox_lite::lite_ui::LinearGradient) -> Self {
        let from = __value.from.into();
        let to = __value.to.into();
        let stops = __value.stops.into();
        Self { from, to, stops }
    }
}

impl From<NativeLinearGradient> for fyrox_lite::lite_ui::LinearGradient {
    fn from(__value: NativeLinearGradient) -> Self {
        let from = __value.from.into();
        let to = __value.to.into();
        let stops = __value.stops.into();
        Self { from, to, stops }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLinearGradient_optional {
    pub value: NativeLinearGradient,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::LinearGradient>> for NativeLinearGradient_optional {
    fn from(value: Option<fyrox_lite::lite_ui::LinearGradient>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeLinearGradient_optional> for Option<fyrox_lite::lite_ui::LinearGradient> {
    fn from(value: NativeLinearGradient_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLinearGradient_slice {
    pub begin: *mut NativeLinearGradient,
    pub len: i32,
}

impl Default for NativeLinearGradient_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::LinearGradient>> for NativeLinearGradient_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::LinearGradient>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeLinearGradient> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeLinearGradient_slice> for Vec<fyrox_lite::lite_ui::LinearGradient> {
    fn from(value: NativeLinearGradient_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_LinearGradient_slice(
    data: NativeLinearGradient_slice,
) -> NativeLinearGradient_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeLinearGradient_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeLinearGradient_result {
    pub ok: i32,
    pub value: NativeLinearGradient_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeLinearGradient_result_value {
    ok: NativeLinearGradient,
    err: NativeString,
}

impl NativeLinearGradient_result {
    pub fn into_result_shallow(self) -> Result<NativeLinearGradient, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_ui::LinearGradient, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::LinearGradient, crate::LangSpecificError>>
    for NativeLinearGradient_result
{
    fn from(value: Result<fyrox_lite::lite_ui::LinearGradient, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeLinearGradient_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeLinearGradient_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeLinearGradient_result>
    for Result<fyrox_lite::lite_ui::LinearGradient, crate::LangSpecificError>
{
    fn from(value: NativeLinearGradient_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRadialGradient {
    pub center: NativeVector2,
    pub stops: NativeGradientPoint_slice,
}

impl From<fyrox_lite::lite_ui::RadialGradient> for NativeRadialGradient {
    fn from(__value: fyrox_lite::lite_ui::RadialGradient) -> Self {
        let center = __value.center.into();
        let stops = __value.stops.into();
        Self { center, stops }
    }
}

impl From<NativeRadialGradient> for fyrox_lite::lite_ui::RadialGradient {
    fn from(__value: NativeRadialGradient) -> Self {
        let center = __value.center.into();
        let stops = __value.stops.into();
        Self { center, stops }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRadialGradient_optional {
    pub value: NativeRadialGradient,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::RadialGradient>> for NativeRadialGradient_optional {
    fn from(value: Option<fyrox_lite::lite_ui::RadialGradient>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeRadialGradient_optional> for Option<fyrox_lite::lite_ui::RadialGradient> {
    fn from(value: NativeRadialGradient_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRadialGradient_slice {
    pub begin: *mut NativeRadialGradient,
    pub len: i32,
}

impl Default for NativeRadialGradient_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::RadialGradient>> for NativeRadialGradient_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::RadialGradient>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeRadialGradient> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeRadialGradient_slice> for Vec<fyrox_lite::lite_ui::RadialGradient> {
    fn from(value: NativeRadialGradient_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_RadialGradient_slice(
    data: NativeRadialGradient_slice,
) -> NativeRadialGradient_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeRadialGradient_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRadialGradient_result {
    pub ok: i32,
    pub value: NativeRadialGradient_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeRadialGradient_result_value {
    ok: NativeRadialGradient,
    err: NativeString,
}

impl NativeRadialGradient_result {
    pub fn into_result_shallow(self) -> Result<NativeRadialGradient, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_ui::RadialGradient, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::RadialGradient, crate::LangSpecificError>>
    for NativeRadialGradient_result
{
    fn from(value: Result<fyrox_lite::lite_ui::RadialGradient, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeRadialGradient_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeRadialGradient_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeRadialGradient_result>
    for Result<fyrox_lite::lite_ui::RadialGradient, crate::LangSpecificError>
{
    fn from(value: NativeRadialGradient_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeGradientPoint {
    pub stop: f32,
    pub color: NativeColor,
}

impl From<fyrox_lite::lite_ui::GradientPoint> for NativeGradientPoint {
    fn from(__value: fyrox_lite::lite_ui::GradientPoint) -> Self {
        let stop = __value.stop.into();
        let color = __value.color.into();
        Self { stop, color }
    }
}

impl From<NativeGradientPoint> for fyrox_lite::lite_ui::GradientPoint {
    fn from(__value: NativeGradientPoint) -> Self {
        let stop = __value.stop.into();
        let color = __value.color.into();
        Self { stop, color }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeGradientPoint_optional {
    pub value: NativeGradientPoint,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_ui::GradientPoint>> for NativeGradientPoint_optional {
    fn from(value: Option<fyrox_lite::lite_ui::GradientPoint>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeGradientPoint_optional> for Option<fyrox_lite::lite_ui::GradientPoint> {
    fn from(value: NativeGradientPoint_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeGradientPoint_slice {
    pub begin: *mut NativeGradientPoint,
    pub len: i32,
}

impl Default for NativeGradientPoint_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_ui::GradientPoint>> for NativeGradientPoint_slice {
    fn from(value: Vec<fyrox_lite::lite_ui::GradientPoint>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeGradientPoint> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeGradientPoint_slice> for Vec<fyrox_lite::lite_ui::GradientPoint> {
    fn from(value: NativeGradientPoint_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_ui_GradientPoint_slice(
    data: NativeGradientPoint_slice,
) -> NativeGradientPoint_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeGradientPoint_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeGradientPoint_result {
    pub ok: i32,
    pub value: NativeGradientPoint_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeGradientPoint_result_value {
    ok: NativeGradientPoint,
    err: NativeString,
}

impl NativeGradientPoint_result {
    pub fn into_result_shallow(self) -> Result<NativeGradientPoint, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_ui::GradientPoint, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_ui::GradientPoint, crate::LangSpecificError>>
    for NativeGradientPoint_result
{
    fn from(value: Result<fyrox_lite::lite_ui::GradientPoint, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeGradientPoint_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeGradientPoint_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeGradientPoint_result>
    for Result<fyrox_lite::lite_ui::GradientPoint, crate::LangSpecificError>
{
    fn from(value: NativeGradientPoint_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePrefab {
    pub handle: NativeHandle,
}

impl From<fyrox_lite::lite_prefab::LitePrefab> for NativePrefab {
    fn from(value: fyrox_lite::lite_prefab::LitePrefab) -> Self {
        Self {
            handle: NativeHandle::from_u128(value.to_external()),
        }
    }
}

impl From<NativePrefab> for fyrox_lite::lite_prefab::LitePrefab {
    fn from(value: NativePrefab) -> Self {
        Self::from_external(value.handle.as_u128())
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_prefab_LitePrefab_instantiate_at(
    this: NativePrefab,
    position: NativeVector3,
    orientation: NativeQuaternion,
) -> NativeNode_result {
    let position = position.into();
    let orientation = orientation.into();
    let ret = fyrox_lite::lite_prefab::LitePrefab::from(this)
        .instantiate_at::<crate::UserScriptImpl>(position, orientation, ());
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePrefab_optional {
    pub value: NativePrefab,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_prefab::LitePrefab>> for NativePrefab_optional {
    fn from(value: Option<fyrox_lite::lite_prefab::LitePrefab>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativePrefab_optional> for Option<fyrox_lite::lite_prefab::LitePrefab> {
    fn from(value: NativePrefab_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePrefab_result {
    pub ok: i32,
    pub value: NativePrefab_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativePrefab_result_value {
    ok: NativePrefab,
    err: NativeString,
}

impl NativePrefab_result {
    pub fn into_result_shallow(self) -> Result<NativePrefab, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_prefab::LitePrefab, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_prefab::LitePrefab, crate::LangSpecificError>>
    for NativePrefab_result
{
    fn from(value: Result<fyrox_lite::lite_prefab::LitePrefab, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativePrefab_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativePrefab_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativePrefab_result>
    for Result<fyrox_lite::lite_prefab::LitePrefab, crate::LangSpecificError>
{
    fn from(value: NativePrefab_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePrefab_optional_result {
    pub ok: i32,
    pub value: NativePrefab_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativePrefab_optional_result_value {
    ok: NativePrefab_optional,
    err: NativeString,
}

impl NativePrefab_optional_result {
    pub fn into_result_shallow(self) -> Result<NativePrefab_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<Option<fyrox_lite::lite_prefab::LitePrefab>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<fyrox_lite::lite_prefab::LitePrefab>, crate::LangSpecificError>>
    for NativePrefab_optional_result
{
    fn from(
        value: Result<Option<fyrox_lite::lite_prefab::LitePrefab>, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativePrefab_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativePrefab_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativePrefab_optional_result>
    for Result<Option<fyrox_lite::lite_prefab::LitePrefab>, crate::LangSpecificError>
{
    fn from(value: NativePrefab_optional_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePhysics {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_physics_LitePhysics_cast_ray(
    opts: NativeRayCastOptions,
) -> NativeIntersection_slice {
    let opts = opts.into();
    let ret = fyrox_lite::lite_physics::LitePhysics::cast_ray(opts);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeIntersection {
    pub collider: NativeNode,
    pub normal: NativeVector3,
    pub position: NativeVector3,
    pub feature: NativeFeatureId,
    pub toi: f32,
}

impl From<fyrox_lite::lite_physics::LiteIntersection> for NativeIntersection {
    fn from(__value: fyrox_lite::lite_physics::LiteIntersection) -> Self {
        let collider = __value.collider.into();
        let normal = __value.normal.into();
        let position = __value.position.into();
        let feature = __value.feature.into();
        let toi = __value.toi.into();
        Self {
            collider,
            normal,
            position,
            feature,
            toi,
        }
    }
}

impl From<NativeIntersection> for fyrox_lite::lite_physics::LiteIntersection {
    fn from(__value: NativeIntersection) -> Self {
        let collider = __value.collider.into();
        let normal = __value.normal.into();
        let position = __value.position.into();
        let feature = __value.feature.into();
        let toi = __value.toi.into();
        Self {
            collider,
            normal,
            position,
            feature,
            toi,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeIntersection_optional {
    pub value: NativeIntersection,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_physics::LiteIntersection>> for NativeIntersection_optional {
    fn from(value: Option<fyrox_lite::lite_physics::LiteIntersection>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeIntersection_optional> for Option<fyrox_lite::lite_physics::LiteIntersection> {
    fn from(value: NativeIntersection_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeIntersection_slice {
    pub begin: *mut NativeIntersection,
    pub len: i32,
}

impl Default for NativeIntersection_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_physics::LiteIntersection>> for NativeIntersection_slice {
    fn from(value: Vec<fyrox_lite::lite_physics::LiteIntersection>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeIntersection> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeIntersection_slice> for Vec<fyrox_lite::lite_physics::LiteIntersection> {
    fn from(value: NativeIntersection_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_physics_LiteIntersection_slice(
    data: NativeIntersection_slice,
) -> NativeIntersection_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeIntersection_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeIntersection_result {
    pub ok: i32,
    pub value: NativeIntersection_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeIntersection_result_value {
    ok: NativeIntersection,
    err: NativeString,
}

impl NativeIntersection_result {
    pub fn into_result_shallow(self) -> Result<NativeIntersection, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_physics::LiteIntersection, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_physics::LiteIntersection, crate::LangSpecificError>>
    for NativeIntersection_result
{
    fn from(
        value: Result<fyrox_lite::lite_physics::LiteIntersection, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeIntersection_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeIntersection_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeIntersection_result>
    for Result<fyrox_lite::lite_physics::LiteIntersection, crate::LangSpecificError>
{
    fn from(value: NativeIntersection_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId {
    pub kind: NativeFeatureKind,
    pub id: i32,
}

impl From<fyrox_lite::lite_physics::LiteFeatureId> for NativeFeatureId {
    fn from(__value: fyrox_lite::lite_physics::LiteFeatureId) -> Self {
        let kind = __value.kind.into();
        let id = __value.id.into();
        Self { kind, id }
    }
}

impl From<NativeFeatureId> for fyrox_lite::lite_physics::LiteFeatureId {
    fn from(__value: NativeFeatureId) -> Self {
        let kind = __value.kind.into();
        let id = __value.id.into();
        Self { kind, id }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_optional {
    pub value: NativeFeatureId,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_physics::LiteFeatureId>> for NativeFeatureId_optional {
    fn from(value: Option<fyrox_lite::lite_physics::LiteFeatureId>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeFeatureId_optional> for Option<fyrox_lite::lite_physics::LiteFeatureId> {
    fn from(value: NativeFeatureId_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_slice {
    pub begin: *mut NativeFeatureId,
    pub len: i32,
}

impl Default for NativeFeatureId_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_physics::LiteFeatureId>> for NativeFeatureId_slice {
    fn from(value: Vec<fyrox_lite::lite_physics::LiteFeatureId>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeFeatureId> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeFeatureId_slice> for Vec<fyrox_lite::lite_physics::LiteFeatureId> {
    fn from(value: NativeFeatureId_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_physics_LiteFeatureId_slice(
    data: NativeFeatureId_slice,
) -> NativeFeatureId_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeFeatureId_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeFeatureId_result {
    pub ok: i32,
    pub value: NativeFeatureId_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeFeatureId_result_value {
    ok: NativeFeatureId,
    err: NativeString,
}

impl NativeFeatureId_result {
    pub fn into_result_shallow(self) -> Result<NativeFeatureId, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_physics::LiteFeatureId, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_physics::LiteFeatureId, crate::LangSpecificError>>
    for NativeFeatureId_result
{
    fn from(
        value: Result<fyrox_lite::lite_physics::LiteFeatureId, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeFeatureId_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeFeatureId_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeFeatureId_result>
    for Result<fyrox_lite::lite_physics::LiteFeatureId, crate::LangSpecificError>
{
    fn from(value: NativeFeatureId_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NativeFeatureKind {
    Vertex,
    Edge,
    Face,
    Unknown,
}

impl From<fyrox_lite::lite_physics::LiteFeatureKind> for NativeFeatureKind {
    fn from(value: fyrox_lite::lite_physics::LiteFeatureKind) -> Self {
        match value {
            fyrox_lite::lite_physics::LiteFeatureKind::Vertex => NativeFeatureKind::Vertex,
            fyrox_lite::lite_physics::LiteFeatureKind::Edge => NativeFeatureKind::Edge,
            fyrox_lite::lite_physics::LiteFeatureKind::Face => NativeFeatureKind::Face,
            fyrox_lite::lite_physics::LiteFeatureKind::Unknown => NativeFeatureKind::Unknown,
        }
    }
}

impl From<NativeFeatureKind> for fyrox_lite::lite_physics::LiteFeatureKind {
    fn from(value: NativeFeatureKind) -> Self {
        match value {
            NativeFeatureKind::Vertex => fyrox_lite::lite_physics::LiteFeatureKind::Vertex,
            NativeFeatureKind::Edge => fyrox_lite::lite_physics::LiteFeatureKind::Edge,
            NativeFeatureKind::Face => fyrox_lite::lite_physics::LiteFeatureKind::Face,
            NativeFeatureKind::Unknown => fyrox_lite::lite_physics::LiteFeatureKind::Unknown,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRayCastOptions {
    pub ray_origin: NativeVector3,
    pub ray_direction: NativeVector3,
    pub max_len: f32,
    pub groups: NativeInteractionGroups_optional,
    pub sort_results: NativeBool,
}

impl From<fyrox_lite::lite_physics::LiteRayCastOptions> for NativeRayCastOptions {
    fn from(__value: fyrox_lite::lite_physics::LiteRayCastOptions) -> Self {
        let ray_origin = __value.ray_origin.into();
        let ray_direction = __value.ray_direction.into();
        let max_len = __value.max_len.into();
        let groups = __value.groups.into();
        let sort_results = __value.sort_results.into();
        Self {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }
    }
}

impl From<NativeRayCastOptions> for fyrox_lite::lite_physics::LiteRayCastOptions {
    fn from(__value: NativeRayCastOptions) -> Self {
        let ray_origin = __value.ray_origin.into();
        let ray_direction = __value.ray_direction.into();
        let max_len = __value.max_len.into();
        let groups = __value.groups.into();
        let sort_results = __value.sort_results.into();
        Self {
            ray_origin,
            ray_direction,
            max_len,
            groups,
            sort_results,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRayCastOptions_optional {
    pub value: NativeRayCastOptions,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_physics::LiteRayCastOptions>> for NativeRayCastOptions_optional {
    fn from(value: Option<fyrox_lite::lite_physics::LiteRayCastOptions>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeRayCastOptions_optional> for Option<fyrox_lite::lite_physics::LiteRayCastOptions> {
    fn from(value: NativeRayCastOptions_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRayCastOptions_slice {
    pub begin: *mut NativeRayCastOptions,
    pub len: i32,
}

impl Default for NativeRayCastOptions_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_physics::LiteRayCastOptions>> for NativeRayCastOptions_slice {
    fn from(value: Vec<fyrox_lite::lite_physics::LiteRayCastOptions>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeRayCastOptions> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeRayCastOptions_slice> for Vec<fyrox_lite::lite_physics::LiteRayCastOptions> {
    fn from(value: NativeRayCastOptions_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_physics_LiteRayCastOptions_slice(
    data: NativeRayCastOptions_slice,
) -> NativeRayCastOptions_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeRayCastOptions_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRayCastOptions_result {
    pub ok: i32,
    pub value: NativeRayCastOptions_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeRayCastOptions_result_value {
    ok: NativeRayCastOptions,
    err: NativeString,
}

impl NativeRayCastOptions_result {
    pub fn into_result_shallow(self) -> Result<NativeRayCastOptions, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_physics::LiteRayCastOptions, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_physics::LiteRayCastOptions, crate::LangSpecificError>>
    for NativeRayCastOptions_result
{
    fn from(
        value: Result<fyrox_lite::lite_physics::LiteRayCastOptions, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeRayCastOptions_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeRayCastOptions_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeRayCastOptions_result>
    for Result<fyrox_lite::lite_physics::LiteRayCastOptions, crate::LangSpecificError>
{
    fn from(value: NativeRayCastOptions_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInteractionGroups {
    pub memberships: i32,
    pub filter: i32,
}

impl From<fyrox_lite::lite_physics::LiteInteractionGroups> for NativeInteractionGroups {
    fn from(__value: fyrox_lite::lite_physics::LiteInteractionGroups) -> Self {
        let memberships = __value.memberships.into();
        let filter = __value.filter.into();
        Self {
            memberships,
            filter,
        }
    }
}

impl From<NativeInteractionGroups> for fyrox_lite::lite_physics::LiteInteractionGroups {
    fn from(__value: NativeInteractionGroups) -> Self {
        let memberships = __value.memberships.into();
        let filter = __value.filter.into();
        Self {
            memberships,
            filter,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInteractionGroups_optional {
    pub value: NativeInteractionGroups,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_physics::LiteInteractionGroups>>
    for NativeInteractionGroups_optional
{
    fn from(value: Option<fyrox_lite::lite_physics::LiteInteractionGroups>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeInteractionGroups_optional>
    for Option<fyrox_lite::lite_physics::LiteInteractionGroups>
{
    fn from(value: NativeInteractionGroups_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInteractionGroups_slice {
    pub begin: *mut NativeInteractionGroups,
    pub len: i32,
}

impl Default for NativeInteractionGroups_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_physics::LiteInteractionGroups>> for NativeInteractionGroups_slice {
    fn from(value: Vec<fyrox_lite::lite_physics::LiteInteractionGroups>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeInteractionGroups> =
            value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeInteractionGroups_slice> for Vec<fyrox_lite::lite_physics::LiteInteractionGroups> {
    fn from(value: NativeInteractionGroups_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_physics_LiteInteractionGroups_slice(
    data: NativeInteractionGroups_slice,
) -> NativeInteractionGroups_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeInteractionGroups_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInteractionGroups_result {
    pub ok: i32,
    pub value: NativeInteractionGroups_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeInteractionGroups_result_value {
    ok: NativeInteractionGroups,
    err: NativeString,
}

impl NativeInteractionGroups_result {
    pub fn into_result_shallow(self) -> Result<NativeInteractionGroups, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_physics::LiteInteractionGroups, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_physics::LiteInteractionGroups, crate::LangSpecificError>>
    for NativeInteractionGroups_result
{
    fn from(
        value: Result<fyrox_lite::lite_physics::LiteInteractionGroups, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeInteractionGroups_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeInteractionGroups_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeInteractionGroups_result>
    for Result<fyrox_lite::lite_physics::LiteInteractionGroups, crate::LangSpecificError>
{
    fn from(value: NativeInteractionGroups_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRigidBody {
    pub handle: NativeHandle,
}

impl From<fyrox_lite::lite_physics::LiteRigidBody> for NativeRigidBody {
    fn from(value: fyrox_lite::lite_physics::LiteRigidBody) -> Self {
        Self {
            handle: NativeHandle::from_u128(value.to_external()),
        }
    }
}

impl From<NativeRigidBody> for fyrox_lite::lite_physics::LiteRigidBody {
    fn from(value: NativeRigidBody) -> Self {
        Self::from_external(value.handle.as_u128())
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_physics_LiteRigidBody_apply_force(
    this: NativeRigidBody,
    force: NativeVector3,
) -> () {
    let force = force.into();
    let ret = fyrox_lite::lite_physics::LiteRigidBody::from(this).apply_force(force);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRigidBody_optional {
    pub value: NativeRigidBody,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_physics::LiteRigidBody>> for NativeRigidBody_optional {
    fn from(value: Option<fyrox_lite::lite_physics::LiteRigidBody>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeRigidBody_optional> for Option<fyrox_lite::lite_physics::LiteRigidBody> {
    fn from(value: NativeRigidBody_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRigidBody_result {
    pub ok: i32,
    pub value: NativeRigidBody_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeRigidBody_result_value {
    ok: NativeRigidBody,
    err: NativeString,
}

impl NativeRigidBody_result {
    pub fn into_result_shallow(self) -> Result<NativeRigidBody, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_physics::LiteRigidBody, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_physics::LiteRigidBody, crate::LangSpecificError>>
    for NativeRigidBody_result
{
    fn from(
        value: Result<fyrox_lite::lite_physics::LiteRigidBody, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeRigidBody_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeRigidBody_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeRigidBody_result>
    for Result<fyrox_lite::lite_physics::LiteRigidBody, crate::LangSpecificError>
{
    fn from(value: NativeRigidBody_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeRigidBody_optional_result {
    pub ok: i32,
    pub value: NativeRigidBody_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeRigidBody_optional_result_value {
    ok: NativeRigidBody_optional,
    err: NativeString,
}

impl NativeRigidBody_optional_result {
    pub fn into_result_shallow(self) -> Result<NativeRigidBody_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<Option<fyrox_lite::lite_physics::LiteRigidBody>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<fyrox_lite::lite_physics::LiteRigidBody>, crate::LangSpecificError>>
    for NativeRigidBody_optional_result
{
    fn from(
        value: Result<Option<fyrox_lite::lite_physics::LiteRigidBody>, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeRigidBody_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeRigidBody_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeRigidBody_optional_result>
    for Result<Option<fyrox_lite::lite_physics::LiteRigidBody>, crate::LangSpecificError>
{
    fn from(value: NativeRigidBody_optional_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNode {
    pub handle: NativeHandle,
}

impl From<fyrox_lite::lite_node::LiteNode> for NativeNode {
    fn from(value: fyrox_lite::lite_node::LiteNode) -> Self {
        Self {
            handle: NativeHandle::from_u128(value.to_external()),
        }
    }
}

impl From<NativeNode> for fyrox_lite::lite_node::LiteNode {
    fn from(value: NativeNode) -> Self {
        Self::from_external(value.handle.as_u128())
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_as_rigid_body(
    this: NativeNode,
) -> NativeRigidBody_optional_result {
    let ret =
        fyrox_lite::lite_node::LiteNode::from(this).as_rigid_body::<crate::UserScriptImpl>(());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_name(this: NativeNode) -> NativeString_result {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_name::<crate::UserScriptImpl>(());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_alive(this: NativeNode) -> NativeBool {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_alive();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_destroy(this: NativeNode) -> () {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).destroy();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_global_position(
    this: NativeNode,
) -> NativeVector3 {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_global_position();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_local_position(
    this: NativeNode,
) -> NativeVector3 {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_local_position();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_local_rotation(
    this: NativeNode,
) -> NativeQuaternion {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_local_rotation();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_send_hierarchical(
    this: NativeNode,
    routing: NativeRoutingStrategy,
    payload: UserScriptMessage,
) -> () {
    let routing = routing.into();
    let payload = payload.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this)
        .send_hierarchical::<crate::UserScriptImpl>(routing, payload);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_set_local_position(
    this: NativeNode,
    new_pos: NativeVector3,
) -> Unit_result {
    let new_pos = new_pos.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this)
        .set_local_position::<crate::UserScriptImpl>(new_pos, ());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_set_local_rotation(
    this: NativeNode,
    value: NativeQuaternion,
) -> Unit_result {
    let value = value.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this)
        .set_local_rotation::<crate::UserScriptImpl>(value, ());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_subscribe_to(this: NativeNode) -> () {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).subscribe_to::<crate::UserScriptImpl>(());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_find_collider_in_children(
    this: NativeNode,
) -> NativeNode_optional {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).find_collider_in_children();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_valid(this: NativeNode) -> NativeBool {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_valid();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_parent(this: NativeNode) -> NativeNode {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_parent();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_add_script(
    this: NativeNode,
    class_id: NativeClassId,
) -> NativeInstanceId_result {
    let class_id = class_id.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this)
        .add_script::<crate::UserScriptImpl>(class_id, ());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_find_script(
    this: NativeNode,
    class_id: NativeClassId,
) -> NativeInstanceId_optional_result {
    let class_id = class_id.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this)
        .find_script::<crate::UserScriptImpl>(class_id, ());
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_global_rotation(
    this: NativeNode,
) -> NativeQuaternion {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_global_rotation();
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_tag_is(
    this: NativeNode,
    tag: NativeString,
) -> NativeBool {
    let tag = tag.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this).tag_is(tag);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_set_tag(this: NativeNode, tag: NativeString) -> () {
    let tag = tag.into();
    let ret = fyrox_lite::lite_node::LiteNode::from(this).set_tag(tag);
    ret.into()
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_node_LiteNode_get_tag(this: NativeNode) -> NativeString {
    let ret = fyrox_lite::lite_node::LiteNode::from(this).get_tag();
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNode_optional {
    pub value: NativeNode,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_node::LiteNode>> for NativeNode_optional {
    fn from(value: Option<fyrox_lite::lite_node::LiteNode>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeNode_optional> for Option<fyrox_lite::lite_node::LiteNode> {
    fn from(value: NativeNode_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNode_result {
    pub ok: i32,
    pub value: NativeNode_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeNode_result_value {
    ok: NativeNode,
    err: NativeString,
}

impl NativeNode_result {
    pub fn into_result_shallow(self) -> Result<NativeNode, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<fyrox_lite::lite_node::LiteNode, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_node::LiteNode, crate::LangSpecificError>> for NativeNode_result {
    fn from(value: Result<fyrox_lite::lite_node::LiteNode, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeNode_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeNode_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeNode_result> for Result<fyrox_lite::lite_node::LiteNode, crate::LangSpecificError> {
    fn from(value: NativeNode_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeNode_optional_result {
    pub ok: i32,
    pub value: NativeNode_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeNode_optional_result_value {
    ok: NativeNode_optional,
    err: NativeString,
}

impl NativeNode_optional_result {
    pub fn into_result_shallow(self) -> Result<NativeNode_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<Option<fyrox_lite::lite_node::LiteNode>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<fyrox_lite::lite_node::LiteNode>, crate::LangSpecificError>>
    for NativeNode_optional_result
{
    fn from(
        value: Result<Option<fyrox_lite::lite_node::LiteNode>, crate::LangSpecificError>,
    ) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeNode_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeNode_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeNode_optional_result>
    for Result<Option<fyrox_lite::lite_node::LiteNode>, crate::LangSpecificError>
{
    fn from(value: NativeNode_optional_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NativeRoutingStrategy {
    Up,
    Down,
}

impl From<fyrox_lite::lite_node::LiteRoutingStrategy> for NativeRoutingStrategy {
    fn from(value: fyrox_lite::lite_node::LiteRoutingStrategy) -> Self {
        match value {
            fyrox_lite::lite_node::LiteRoutingStrategy::Up => NativeRoutingStrategy::Up,
            fyrox_lite::lite_node::LiteRoutingStrategy::Down => NativeRoutingStrategy::Down,
        }
    }
}

impl From<NativeRoutingStrategy> for fyrox_lite::lite_node::LiteRoutingStrategy {
    fn from(value: NativeRoutingStrategy) -> Self {
        match value {
            NativeRoutingStrategy::Up => fyrox_lite::lite_node::LiteRoutingStrategy::Up,
            NativeRoutingStrategy::Down => fyrox_lite::lite_node::LiteRoutingStrategy::Down,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWindow {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_window_LiteWindow_set_cursor_grab(
    mode: NativeCursorGrabMode,
) -> () {
    let mode = mode.into();
    let ret = fyrox_lite::lite_window::LiteWindow::set_cursor_grab(mode);
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NativeCursorGrabMode {
    None,
    Confined,
    Locked,
}

impl From<fyrox_lite::lite_window::LiteCursorGrabMode> for NativeCursorGrabMode {
    fn from(value: fyrox_lite::lite_window::LiteCursorGrabMode) -> Self {
        match value {
            fyrox_lite::lite_window::LiteCursorGrabMode::None => NativeCursorGrabMode::None,
            fyrox_lite::lite_window::LiteCursorGrabMode::Confined => NativeCursorGrabMode::Confined,
            fyrox_lite::lite_window::LiteCursorGrabMode::Locked => NativeCursorGrabMode::Locked,
        }
    }
}

impl From<NativeCursorGrabMode> for fyrox_lite::lite_window::LiteCursorGrabMode {
    fn from(value: NativeCursorGrabMode) -> Self {
        match value {
            NativeCursorGrabMode::None => fyrox_lite::lite_window::LiteCursorGrabMode::None,
            NativeCursorGrabMode::Confined => fyrox_lite::lite_window::LiteCursorGrabMode::Confined,
            NativeCursorGrabMode::Locked => fyrox_lite::lite_window::LiteCursorGrabMode::Locked,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePlugin {
    pub handle: NativeHandle,
}

#[no_mangle]
pub extern "C" fn fyrox_lite_lite_plugin_LitePlugin_get(
    class_id: NativeClassId,
) -> NativeInstanceId_result {
    let class_id = class_id.into();
    let ret = fyrox_lite::lite_plugin::LitePlugin::get::<crate::UserScriptImpl>(class_id, ());
    ret.into()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<fyrox_lite::lite_math::PodVector3> for NativeVector3 {
    fn from(__value: fyrox_lite::lite_math::PodVector3) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        let z = __value.z.into();
        Self { x, y, z }
    }
}

impl From<NativeVector3> for fyrox_lite::lite_math::PodVector3 {
    fn from(__value: NativeVector3) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        let z = __value.z.into();
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3_optional {
    pub value: NativeVector3,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_math::PodVector3>> for NativeVector3_optional {
    fn from(value: Option<fyrox_lite::lite_math::PodVector3>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeVector3_optional> for Option<fyrox_lite::lite_math::PodVector3> {
    fn from(value: NativeVector3_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3_slice {
    pub begin: *mut NativeVector3,
    pub len: i32,
}

impl Default for NativeVector3_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_math::PodVector3>> for NativeVector3_slice {
    fn from(value: Vec<fyrox_lite::lite_math::PodVector3>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeVector3> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeVector3_slice> for Vec<fyrox_lite::lite_math::PodVector3> {
    fn from(value: NativeVector3_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_math_PodVector3_slice(
    data: NativeVector3_slice,
) -> NativeVector3_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeVector3_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector3_result {
    pub ok: i32,
    pub value: NativeVector3_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeVector3_result_value {
    ok: NativeVector3,
    err: NativeString,
}

impl NativeVector3_result {
    pub fn into_result_shallow(self) -> Result<NativeVector3, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_math::PodVector3, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_math::PodVector3, crate::LangSpecificError>>
    for NativeVector3_result
{
    fn from(value: Result<fyrox_lite::lite_math::PodVector3, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeVector3_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeVector3_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeVector3_result>
    for Result<fyrox_lite::lite_math::PodVector3, crate::LangSpecificError>
{
    fn from(value: NativeVector3_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2 {
    pub x: f32,
    pub y: f32,
}

impl From<fyrox_lite::lite_math::PodVector2> for NativeVector2 {
    fn from(__value: fyrox_lite::lite_math::PodVector2) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        Self { x, y }
    }
}

impl From<NativeVector2> for fyrox_lite::lite_math::PodVector2 {
    fn from(__value: NativeVector2) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2_optional {
    pub value: NativeVector2,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_math::PodVector2>> for NativeVector2_optional {
    fn from(value: Option<fyrox_lite::lite_math::PodVector2>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeVector2_optional> for Option<fyrox_lite::lite_math::PodVector2> {
    fn from(value: NativeVector2_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2_slice {
    pub begin: *mut NativeVector2,
    pub len: i32,
}

impl Default for NativeVector2_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_math::PodVector2>> for NativeVector2_slice {
    fn from(value: Vec<fyrox_lite::lite_math::PodVector2>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeVector2> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeVector2_slice> for Vec<fyrox_lite::lite_math::PodVector2> {
    fn from(value: NativeVector2_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_math_PodVector2_slice(
    data: NativeVector2_slice,
) -> NativeVector2_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeVector2_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2_result {
    pub ok: i32,
    pub value: NativeVector2_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeVector2_result_value {
    ok: NativeVector2,
    err: NativeString,
}

impl NativeVector2_result {
    pub fn into_result_shallow(self) -> Result<NativeVector2, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_math::PodVector2, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_math::PodVector2, crate::LangSpecificError>>
    for NativeVector2_result
{
    fn from(value: Result<fyrox_lite::lite_math::PodVector2, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeVector2_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeVector2_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeVector2_result>
    for Result<fyrox_lite::lite_math::PodVector2, crate::LangSpecificError>
{
    fn from(value: NativeVector2_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2I {
    pub x: i32,
    pub y: i32,
}

impl From<fyrox_lite::lite_math::PodVector2I> for NativeVector2I {
    fn from(__value: fyrox_lite::lite_math::PodVector2I) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        Self { x, y }
    }
}

impl From<NativeVector2I> for fyrox_lite::lite_math::PodVector2I {
    fn from(__value: NativeVector2I) -> Self {
        let x = __value.x.into();
        let y = __value.y.into();
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2I_optional {
    pub value: NativeVector2I,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_math::PodVector2I>> for NativeVector2I_optional {
    fn from(value: Option<fyrox_lite::lite_math::PodVector2I>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeVector2I_optional> for Option<fyrox_lite::lite_math::PodVector2I> {
    fn from(value: NativeVector2I_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2I_slice {
    pub begin: *mut NativeVector2I,
    pub len: i32,
}

impl Default for NativeVector2I_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_math::PodVector2I>> for NativeVector2I_slice {
    fn from(value: Vec<fyrox_lite::lite_math::PodVector2I>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeVector2I> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeVector2I_slice> for Vec<fyrox_lite::lite_math::PodVector2I> {
    fn from(value: NativeVector2I_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_math_PodVector2I_slice(
    data: NativeVector2I_slice,
) -> NativeVector2I_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeVector2I_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeVector2I_result {
    pub ok: i32,
    pub value: NativeVector2I_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeVector2I_result_value {
    ok: NativeVector2I,
    err: NativeString,
}

impl NativeVector2I_result {
    pub fn into_result_shallow(self) -> Result<NativeVector2I, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_math::PodVector2I, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_math::PodVector2I, crate::LangSpecificError>>
    for NativeVector2I_result
{
    fn from(value: Result<fyrox_lite::lite_math::PodVector2I, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeVector2I_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeVector2I_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeVector2I_result>
    for Result<fyrox_lite::lite_math::PodVector2I, crate::LangSpecificError>
{
    fn from(value: NativeVector2I_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion {
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub w: f32,
}

impl From<fyrox_lite::lite_math::PodQuaternion> for NativeQuaternion {
    fn from(__value: fyrox_lite::lite_math::PodQuaternion) -> Self {
        let i = __value.i.into();
        let j = __value.j.into();
        let k = __value.k.into();
        let w = __value.w.into();
        Self { i, j, k, w }
    }
}

impl From<NativeQuaternion> for fyrox_lite::lite_math::PodQuaternion {
    fn from(__value: NativeQuaternion) -> Self {
        let i = __value.i.into();
        let j = __value.j.into();
        let k = __value.k.into();
        let w = __value.w.into();
        Self { i, j, k, w }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion_optional {
    pub value: NativeQuaternion,
    pub has_value: i32,
}

impl From<Option<fyrox_lite::lite_math::PodQuaternion>> for NativeQuaternion_optional {
    fn from(value: Option<fyrox_lite::lite_math::PodQuaternion>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeQuaternion_optional> for Option<fyrox_lite::lite_math::PodQuaternion> {
    fn from(value: NativeQuaternion_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion_slice {
    pub begin: *mut NativeQuaternion,
    pub len: i32,
}

impl Default for NativeQuaternion_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<fyrox_lite::lite_math::PodQuaternion>> for NativeQuaternion_slice {
    fn from(value: Vec<fyrox_lite::lite_math::PodQuaternion>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeQuaternion> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeQuaternion_slice> for Vec<fyrox_lite::lite_math::PodQuaternion> {
    fn from(value: NativeQuaternion_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_fyrox_lite_lite_math_PodQuaternion_slice(
    data: NativeQuaternion_slice,
) -> NativeQuaternion_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeQuaternion_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeQuaternion_result {
    pub ok: i32,
    pub value: NativeQuaternion_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeQuaternion_result_value {
    ok: NativeQuaternion,
    err: NativeString,
}

impl NativeQuaternion_result {
    pub fn into_result_shallow(self) -> Result<NativeQuaternion, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(
        self,
    ) -> Result<fyrox_lite::lite_math::PodQuaternion, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<fyrox_lite::lite_math::PodQuaternion, crate::LangSpecificError>>
    for NativeQuaternion_result
{
    fn from(value: Result<fyrox_lite::lite_math::PodQuaternion, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeQuaternion_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeQuaternion_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeQuaternion_result>
    for Result<fyrox_lite::lite_math::PodQuaternion, crate::LangSpecificError>
{
    fn from(value: NativeQuaternion_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_result {
    pub ok: i32,
    pub value: NativeBool_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeBool_result_value {
    ok: NativeBool,
    err: NativeString,
}

impl NativeBool_result {
    pub fn into_result_shallow(self) -> Result<NativeBool, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<bool, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<bool, crate::LangSpecificError>> for NativeBool_result {
    fn from(value: Result<bool, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeBool_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeBool_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeBool_result> for Result<bool, crate::LangSpecificError> {
    fn from(value: NativeBool_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_optional {
    pub value: NativeBool,
    pub has_value: i32,
}

impl From<Option<bool>> for NativeBool_optional {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeBool_optional> for Option<bool> {
    fn from(value: NativeBool_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_slice {
    pub begin: *mut NativeBool,
    pub len: i32,
}

impl Default for NativeBool_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<bool>> for NativeBool_slice {
    fn from(value: Vec<bool>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeBool> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeBool_slice> for Vec<bool> {
    fn from(value: NativeBool_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_bool_slice(data: NativeBool_slice) -> NativeBool_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeBool_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_result {
    pub ok: i32,
    pub value: u8_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union u8_result_value {
    ok: u8,
    err: NativeString,
}

impl u8_result {
    pub fn into_result_shallow(self) -> Result<u8, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<u8, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<u8, crate::LangSpecificError>> for u8_result {
    fn from(value: Result<u8, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: u8_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: u8_result_value { err: err.into() },
            },
        }
    }
}

impl From<u8_result> for Result<u8, crate::LangSpecificError> {
    fn from(value: u8_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_optional {
    pub value: u8,
    pub has_value: i32,
}

impl From<Option<u8>> for u8_optional {
    fn from(value: Option<u8>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<u8_optional> for Option<u8> {
    fn from(value: u8_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_slice {
    pub begin: *mut u8,
    pub len: i32,
}

impl Default for u8_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<u8>> for u8_slice {
    fn from(value: Vec<u8>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<u8> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<u8_slice> for Vec<u8> {
    fn from(value: u8_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_u8_slice(data: u8_slice) -> u8_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    u8_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_result {
    pub ok: i32,
    pub value: i32_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union i32_result_value {
    ok: i32,
    err: NativeString,
}

impl i32_result {
    pub fn into_result_shallow(self) -> Result<i32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<i32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<i32, crate::LangSpecificError>> for i32_result {
    fn from(value: Result<i32, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: i32_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: i32_result_value { err: err.into() },
            },
        }
    }
}

impl From<i32_result> for Result<i32, crate::LangSpecificError> {
    fn from(value: i32_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_optional {
    pub value: i32,
    pub has_value: i32,
}

impl From<Option<i32>> for i32_optional {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<i32_optional> for Option<i32> {
    fn from(value: i32_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_slice {
    pub begin: *mut i32,
    pub len: i32,
}

impl Default for i32_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<i32>> for i32_slice {
    fn from(value: Vec<i32>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<i32> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<i32_slice> for Vec<i32> {
    fn from(value: i32_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_i32_slice(data: i32_slice) -> i32_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    i32_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_result {
    pub ok: i32,
    pub value: i64_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union i64_result_value {
    ok: i64,
    err: NativeString,
}

impl i64_result {
    pub fn into_result_shallow(self) -> Result<i64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<i64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<i64, crate::LangSpecificError>> for i64_result {
    fn from(value: Result<i64, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: i64_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: i64_result_value { err: err.into() },
            },
        }
    }
}

impl From<i64_result> for Result<i64, crate::LangSpecificError> {
    fn from(value: i64_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_optional {
    pub value: i64,
    pub has_value: i32,
}

impl From<Option<i64>> for i64_optional {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<i64_optional> for Option<i64> {
    fn from(value: i64_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_slice {
    pub begin: *mut i64,
    pub len: i32,
}

impl Default for i64_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<i64>> for i64_slice {
    fn from(value: Vec<i64>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<i64> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<i64_slice> for Vec<i64> {
    fn from(value: i64_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_i64_slice(data: i64_slice) -> i64_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    i64_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_result {
    pub ok: i32,
    pub value: f32_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union f32_result_value {
    ok: f32,
    err: NativeString,
}

impl f32_result {
    pub fn into_result_shallow(self) -> Result<f32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<f32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<f32, crate::LangSpecificError>> for f32_result {
    fn from(value: Result<f32, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: f32_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: f32_result_value { err: err.into() },
            },
        }
    }
}

impl From<f32_result> for Result<f32, crate::LangSpecificError> {
    fn from(value: f32_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_optional {
    pub value: f32,
    pub has_value: i32,
}

impl From<Option<f32>> for f32_optional {
    fn from(value: Option<f32>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<f32_optional> for Option<f32> {
    fn from(value: f32_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_slice {
    pub begin: *mut f32,
    pub len: i32,
}

impl Default for f32_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<f32>> for f32_slice {
    fn from(value: Vec<f32>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<f32> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<f32_slice> for Vec<f32> {
    fn from(value: f32_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_f32_slice(data: f32_slice) -> f32_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    f32_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_result {
    pub ok: i32,
    pub value: f64_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union f64_result_value {
    ok: f64,
    err: NativeString,
}

impl f64_result {
    pub fn into_result_shallow(self) -> Result<f64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<f64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<f64, crate::LangSpecificError>> for f64_result {
    fn from(value: Result<f64, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: f64_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: f64_result_value { err: err.into() },
            },
        }
    }
}

impl From<f64_result> for Result<f64, crate::LangSpecificError> {
    fn from(value: f64_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_optional {
    pub value: f64,
    pub has_value: i32,
}

impl From<Option<f64>> for f64_optional {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<f64_optional> for Option<f64> {
    fn from(value: f64_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_slice {
    pub begin: *mut f64,
    pub len: i32,
}

impl Default for f64_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<f64>> for f64_slice {
    fn from(value: Vec<f64>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<f64> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<f64_slice> for Vec<f64> {
    fn from(value: f64_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_f64_slice(data: f64_slice) -> f64_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    f64_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_result {
    pub ok: i32,
    pub value: NativeString_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeString_result_value {
    ok: NativeString,
    err: NativeString,
}

impl NativeString_result {
    pub fn into_result_shallow(self) -> Result<NativeString, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<String, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<String, crate::LangSpecificError>> for NativeString_result {
    fn from(value: Result<String, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeString_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeString_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeString_result> for Result<String, crate::LangSpecificError> {
    fn from(value: NativeString_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_optional {
    pub value: NativeString,
    pub has_value: i32,
}

impl From<Option<String>> for NativeString_optional {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeString_optional> for Option<String> {
    fn from(value: NativeString_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_slice {
    pub begin: *mut NativeString,
    pub len: i32,
}

impl Default for NativeString_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<String>> for NativeString_slice {
    fn from(value: Vec<String>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeString> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeString_slice> for Vec<String> {
    fn from(value: NativeString_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_String_slice(data: NativeString_slice) -> NativeString_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeString_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_result {
    pub ok: i32,
    pub value: NativeInstanceId_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeInstanceId_result_value {
    ok: NativeInstanceId,
    err: NativeString,
}

impl NativeInstanceId_result {
    pub fn into_result_shallow(self) -> Result<NativeInstanceId, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<crate::UserScriptImpl, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<crate::UserScriptImpl, crate::LangSpecificError>> for NativeInstanceId_result {
    fn from(value: Result<crate::UserScriptImpl, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeInstanceId_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeInstanceId_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeInstanceId_result> for Result<crate::UserScriptImpl, crate::LangSpecificError> {
    fn from(value: NativeInstanceId_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_optional {
    pub value: NativeInstanceId,
    pub has_value: i32,
}

impl From<Option<crate::UserScriptImpl>> for NativeInstanceId_optional {
    fn from(value: Option<crate::UserScriptImpl>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeInstanceId_optional> for Option<crate::UserScriptImpl> {
    fn from(value: NativeInstanceId_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_slice {
    pub begin: *mut NativeInstanceId,
    pub len: i32,
}

impl Default for NativeInstanceId_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<crate::UserScriptImpl>> for NativeInstanceId_slice {
    fn from(value: Vec<crate::UserScriptImpl>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeInstanceId> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeInstanceId_slice> for Vec<crate::UserScriptImpl> {
    fn from(value: NativeInstanceId_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_crate_UserScriptImpl_slice(
    data: NativeInstanceId_slice,
) -> NativeInstanceId_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeInstanceId_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptMetadata_slice {
    pub begin: *mut NativeScriptMetadata,
    pub len: i32,
}

impl Default for NativeScriptMetadata_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeScriptMetadata>> for NativeScriptMetadata_slice {
    fn from(value: Vec<NativeScriptMetadata>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeScriptMetadata> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeScriptMetadata_slice> for Vec<NativeScriptMetadata> {
    fn from(value: NativeScriptMetadata_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeScriptMetadata_slice(
    data: NativeScriptMetadata_slice,
) -> NativeScriptMetadata_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeScriptMetadata_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptProperty_slice {
    pub begin: *mut NativeScriptProperty,
    pub len: i32,
}

impl Default for NativeScriptProperty_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeScriptProperty>> for NativeScriptProperty_slice {
    fn from(value: Vec<NativeScriptProperty>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeScriptProperty> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeScriptProperty_slice> for Vec<NativeScriptProperty> {
    fn from(value: NativeScriptProperty_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeScriptProperty_slice(
    data: NativeScriptProperty_slice,
) -> NativeScriptProperty_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeScriptProperty_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeValue_slice {
    pub begin: *mut NativeValue,
    pub len: i32,
}

impl Default for NativeValue_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeValue>> for NativeValue_slice {
    fn from(value: Vec<NativeValue>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeValue> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeValue_slice> for Vec<NativeValue> {
    fn from(value: NativeValue_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeValue_slice(
    data: NativeValue_slice,
) -> NativeValue_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeValue_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeHandle_optional {
    pub value: NativeHandle,
    pub has_value: i32,
}

impl From<Option<NativeHandle>> for NativeHandle_optional {
    fn from(value: Option<NativeHandle>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeHandle_optional> for Option<NativeHandle> {
    fn from(value: NativeHandle_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePropertyValue_slice {
    pub begin: *mut NativePropertyValue,
    pub len: i32,
}

impl Default for NativePropertyValue_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativePropertyValue>> for NativePropertyValue_slice {
    fn from(value: Vec<NativePropertyValue>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativePropertyValue> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativePropertyValue_slice> for Vec<NativePropertyValue> {
    fn from(value: NativePropertyValue_slice) -> Self {
        let mut vec = Vec::new();
        unsafe {
            for i in 0..value.len {
                let v = *value.begin.add(i as usize);
                vec.push(v.into());
            }
        }
        vec
    }
}

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativePropertyValue_slice(
    data: NativePropertyValue_slice,
) -> NativePropertyValue_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativePropertyValue_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_optional_result {
    pub ok: i32,
    pub value: NativeInstanceId_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeInstanceId_optional_result_value {
    ok: NativeInstanceId_optional,
    err: NativeString,
}

impl NativeInstanceId_optional_result {
    pub fn into_result_shallow(
        self,
    ) -> Result<NativeInstanceId_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<Option<crate::UserScriptImpl>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>>
    for NativeInstanceId_optional_result
{
    fn from(value: Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeInstanceId_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeInstanceId_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeInstanceId_optional_result>
    for Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>
{
    fn from(value: NativeInstanceId_optional_result) -> Self {
        value.into_result()
    }
}
