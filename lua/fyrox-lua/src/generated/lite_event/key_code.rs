
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for fyrox_lite::lite_event::KeyCode {
    const CLASS_NAME: &'static str = "KeyCode";

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Backquote", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Backquote))
        });

        fields.add_field_method_get("Backslash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Backslash))
        });

        fields.add_field_method_get("BracketLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BracketLeft))
        });

        fields.add_field_method_get("BracketRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BracketRight))
        });

        fields.add_field_method_get("Comma", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Comma))
        });

        fields.add_field_method_get("Digit0", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit0))
        });

        fields.add_field_method_get("Digit1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit1))
        });

        fields.add_field_method_get("Digit2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit2))
        });

        fields.add_field_method_get("Digit3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit3))
        });

        fields.add_field_method_get("Digit4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit4))
        });

        fields.add_field_method_get("Digit5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit5))
        });

        fields.add_field_method_get("Digit6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit6))
        });

        fields.add_field_method_get("Digit7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit7))
        });

        fields.add_field_method_get("Digit8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit8))
        });

        fields.add_field_method_get("Digit9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Digit9))
        });

        fields.add_field_method_get("Equal", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Equal))
        });

        fields.add_field_method_get("IntlBackslash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::IntlBackslash))
        });

        fields.add_field_method_get("IntlRo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::IntlRo))
        });

        fields.add_field_method_get("IntlYen", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::IntlYen))
        });

        fields.add_field_method_get("KeyA", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyA))
        });

        fields.add_field_method_get("KeyB", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyB))
        });

        fields.add_field_method_get("KeyC", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyC))
        });

        fields.add_field_method_get("KeyD", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyD))
        });

        fields.add_field_method_get("KeyE", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyE))
        });

        fields.add_field_method_get("KeyF", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyF))
        });

        fields.add_field_method_get("KeyG", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyG))
        });

        fields.add_field_method_get("KeyH", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyH))
        });

        fields.add_field_method_get("KeyI", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyI))
        });

        fields.add_field_method_get("KeyJ", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyJ))
        });

        fields.add_field_method_get("KeyK", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyK))
        });

        fields.add_field_method_get("KeyL", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyL))
        });

        fields.add_field_method_get("KeyM", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyM))
        });

        fields.add_field_method_get("KeyN", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyN))
        });

        fields.add_field_method_get("KeyO", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyO))
        });

        fields.add_field_method_get("KeyP", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyP))
        });

        fields.add_field_method_get("KeyQ", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyQ))
        });

        fields.add_field_method_get("KeyR", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyR))
        });

        fields.add_field_method_get("KeyS", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyS))
        });

        fields.add_field_method_get("KeyT", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyT))
        });

        fields.add_field_method_get("KeyU", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyU))
        });

        fields.add_field_method_get("KeyV", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyV))
        });

        fields.add_field_method_get("KeyW", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyW))
        });

        fields.add_field_method_get("KeyX", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyX))
        });

        fields.add_field_method_get("KeyY", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyY))
        });

        fields.add_field_method_get("KeyZ", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KeyZ))
        });

        fields.add_field_method_get("Minus", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Minus))
        });

        fields.add_field_method_get("Period", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Period))
        });

        fields.add_field_method_get("Quote", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Quote))
        });

        fields.add_field_method_get("Semicolon", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Semicolon))
        });

        fields.add_field_method_get("Slash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Slash))
        });

        fields.add_field_method_get("AltLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::AltLeft))
        });

        fields.add_field_method_get("AltRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::AltRight))
        });

        fields.add_field_method_get("Backspace", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Backspace))
        });

        fields.add_field_method_get("CapsLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::CapsLock))
        });

        fields.add_field_method_get("ContextMenu", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ContextMenu))
        });

        fields.add_field_method_get("ControlLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ControlLeft))
        });

        fields.add_field_method_get("ControlRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ControlRight))
        });

        fields.add_field_method_get("Enter", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Enter))
        });

        fields.add_field_method_get("SuperLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::SuperLeft))
        });

        fields.add_field_method_get("SuperRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::SuperRight))
        });

        fields.add_field_method_get("ShiftLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ShiftLeft))
        });

        fields.add_field_method_get("ShiftRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ShiftRight))
        });

        fields.add_field_method_get("Space", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Space))
        });

        fields.add_field_method_get("Tab", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Tab))
        });

        fields.add_field_method_get("Convert", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Convert))
        });

        fields.add_field_method_get("KanaMode", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::KanaMode))
        });

        fields.add_field_method_get("Lang1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Lang1))
        });

        fields.add_field_method_get("Lang2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Lang2))
        });

        fields.add_field_method_get("Lang3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Lang3))
        });

        fields.add_field_method_get("Lang4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Lang4))
        });

        fields.add_field_method_get("Lang5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Lang5))
        });

        fields.add_field_method_get("NonConvert", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NonConvert))
        });

        fields.add_field_method_get("Delete", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Delete))
        });

        fields.add_field_method_get("End", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::End))
        });

        fields.add_field_method_get("Help", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Help))
        });

        fields.add_field_method_get("Home", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Home))
        });

        fields.add_field_method_get("Insert", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Insert))
        });

        fields.add_field_method_get("PageDown", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::PageDown))
        });

        fields.add_field_method_get("PageUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::PageUp))
        });

        fields.add_field_method_get("ArrowDown", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ArrowDown))
        });

        fields.add_field_method_get("ArrowLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ArrowLeft))
        });

        fields.add_field_method_get("ArrowRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ArrowRight))
        });

        fields.add_field_method_get("ArrowUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ArrowUp))
        });

        fields.add_field_method_get("NumLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumLock))
        });

        fields.add_field_method_get("Numpad0", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad0))
        });

        fields.add_field_method_get("Numpad1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad1))
        });

        fields.add_field_method_get("Numpad2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad2))
        });

        fields.add_field_method_get("Numpad3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad3))
        });

        fields.add_field_method_get("Numpad4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad4))
        });

        fields.add_field_method_get("Numpad5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad5))
        });

        fields.add_field_method_get("Numpad6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad6))
        });

        fields.add_field_method_get("Numpad7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad7))
        });

        fields.add_field_method_get("Numpad8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad8))
        });

        fields.add_field_method_get("Numpad9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Numpad9))
        });

        fields.add_field_method_get("NumpadAdd", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadAdd))
        });

        fields.add_field_method_get("NumpadBackspace", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadBackspace,
            ))
        });

        fields.add_field_method_get("NumpadClear", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadClear))
        });

        fields.add_field_method_get("NumpadClearEntry", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadClearEntry,
            ))
        });

        fields.add_field_method_get("NumpadComma", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadComma))
        });

        fields.add_field_method_get("NumpadDecimal", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadDecimal))
        });

        fields.add_field_method_get("NumpadDivide", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadDivide))
        });

        fields.add_field_method_get("NumpadEnter", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadEnter))
        });

        fields.add_field_method_get("NumpadEqual", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadEqual))
        });

        fields.add_field_method_get("NumpadHash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadHash))
        });

        fields.add_field_method_get("NumpadMemoryAdd", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMemoryAdd,
            ))
        });

        fields.add_field_method_get("NumpadMemoryClear", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMemoryClear,
            ))
        });

        fields.add_field_method_get("NumpadMemoryRecall", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMemoryRecall,
            ))
        });

        fields.add_field_method_get("NumpadMemoryStore", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMemoryStore,
            ))
        });

        fields.add_field_method_get("NumpadMemorySubtract", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMemorySubtract,
            ))
        });

        fields.add_field_method_get("NumpadMultiply", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadMultiply,
            ))
        });

        fields.add_field_method_get("NumpadParenLeft", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadParenLeft,
            ))
        });

        fields.add_field_method_get("NumpadParenRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadParenRight,
            ))
        });

        fields.add_field_method_get("NumpadStar", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::NumpadStar))
        });

        fields.add_field_method_get("NumpadSubtract", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::NumpadSubtract,
            ))
        });

        fields.add_field_method_get("Escape", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Escape))
        });

        fields.add_field_method_get("Fn", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Fn))
        });

        fields.add_field_method_get("FnLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::FnLock))
        });

        fields.add_field_method_get("PrintScreen", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::PrintScreen))
        });

        fields.add_field_method_get("ScrollLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::ScrollLock))
        });

        fields.add_field_method_get("Pause", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Pause))
        });

        fields.add_field_method_get("BrowserBack", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BrowserBack))
        });

        fields.add_field_method_get("BrowserFavorites", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::BrowserFavorites,
            ))
        });

        fields.add_field_method_get("BrowserForward", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::BrowserForward,
            ))
        });

        fields.add_field_method_get("BrowserHome", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BrowserHome))
        });

        fields.add_field_method_get("BrowserRefresh", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::BrowserRefresh,
            ))
        });

        fields.add_field_method_get("BrowserSearch", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BrowserSearch))
        });

        fields.add_field_method_get("BrowserStop", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::BrowserStop))
        });

        fields.add_field_method_get("Eject", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Eject))
        });

        fields.add_field_method_get("LaunchApp1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::LaunchApp1))
        });

        fields.add_field_method_get("LaunchApp2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::LaunchApp2))
        });

        fields.add_field_method_get("LaunchMail", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::LaunchMail))
        });

        fields.add_field_method_get("MediaPlayPause", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::MediaPlayPause,
            ))
        });

        fields.add_field_method_get("MediaSelect", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::MediaSelect))
        });

        fields.add_field_method_get("MediaStop", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::MediaStop))
        });

        fields.add_field_method_get("MediaTrackNext", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::MediaTrackNext,
            ))
        });

        fields.add_field_method_get("MediaTrackPrevious", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::MediaTrackPrevious,
            ))
        });

        fields.add_field_method_get("Power", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Power))
        });

        fields.add_field_method_get("Sleep", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Sleep))
        });

        fields.add_field_method_get("AudioVolumeDown", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::AudioVolumeDown,
            ))
        });

        fields.add_field_method_get("AudioVolumeMute", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_event::KeyCode::AudioVolumeMute,
            ))
        });

        fields.add_field_method_get("AudioVolumeUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::AudioVolumeUp))
        });

        fields.add_field_method_get("WakeUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::WakeUp))
        });

        fields.add_field_method_get("Meta", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Meta))
        });

        fields.add_field_method_get("Hyper", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Hyper))
        });

        fields.add_field_method_get("Turbo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Turbo))
        });

        fields.add_field_method_get("Abort", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Abort))
        });

        fields.add_field_method_get("Resume", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Resume))
        });

        fields.add_field_method_get("Suspend", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Suspend))
        });

        fields.add_field_method_get("Again", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Again))
        });

        fields.add_field_method_get("Copy", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Copy))
        });

        fields.add_field_method_get("Cut", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Cut))
        });

        fields.add_field_method_get("Find", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Find))
        });

        fields.add_field_method_get("Open", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Open))
        });

        fields.add_field_method_get("Paste", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Paste))
        });

        fields.add_field_method_get("Props", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Props))
        });

        fields.add_field_method_get("Select", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Select))
        });

        fields.add_field_method_get("Undo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Undo))
        });

        fields.add_field_method_get("Hiragana", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Hiragana))
        });

        fields.add_field_method_get("Katakana", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::Katakana))
        });

        fields.add_field_method_get("F1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F1))
        });

        fields.add_field_method_get("F2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F2))
        });

        fields.add_field_method_get("F3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F3))
        });

        fields.add_field_method_get("F4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F4))
        });

        fields.add_field_method_get("F5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F5))
        });

        fields.add_field_method_get("F6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F6))
        });

        fields.add_field_method_get("F7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F7))
        });

        fields.add_field_method_get("F8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F8))
        });

        fields.add_field_method_get("F9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F9))
        });

        fields.add_field_method_get("F10", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F10))
        });

        fields.add_field_method_get("F11", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F11))
        });

        fields.add_field_method_get("F12", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F12))
        });

        fields.add_field_method_get("F13", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F13))
        });

        fields.add_field_method_get("F14", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F14))
        });

        fields.add_field_method_get("F15", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F15))
        });

        fields.add_field_method_get("F16", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F16))
        });

        fields.add_field_method_get("F17", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F17))
        });

        fields.add_field_method_get("F18", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F18))
        });

        fields.add_field_method_get("F19", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F19))
        });

        fields.add_field_method_get("F20", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F20))
        });

        fields.add_field_method_get("F21", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F21))
        });

        fields.add_field_method_get("F22", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F22))
        });

        fields.add_field_method_get("F23", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F23))
        });

        fields.add_field_method_get("F24", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F24))
        });

        fields.add_field_method_get("F25", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F25))
        });

        fields.add_field_method_get("F26", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F26))
        });

        fields.add_field_method_get("F27", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F27))
        });

        fields.add_field_method_get("F28", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F28))
        });

        fields.add_field_method_get("F29", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F29))
        });

        fields.add_field_method_get("F30", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F30))
        });

        fields.add_field_method_get("F31", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F31))
        });

        fields.add_field_method_get("F32", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F32))
        });

        fields.add_field_method_get("F33", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F33))
        });

        fields.add_field_method_get("F34", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F34))
        });

        fields.add_field_method_get("F35", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_event::KeyCode::F35))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Backquote", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Backquote = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Backslash", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Backslash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BracketLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BracketLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BracketRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BracketRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Comma", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Comma = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit0", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit0 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit1", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit2", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit3", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit4", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit5", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit6", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit7", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit8", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Digit9", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Digit9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Equal", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Equal = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("IntlBackslash", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::IntlBackslash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("IntlRo", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::IntlRo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("IntlYen", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::IntlYen = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyA", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyA = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyB", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyB = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyC", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyC = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyD", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyD = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyE", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyE = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyF", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyF = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyG", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyG = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyH", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyH = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyI", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyI = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyJ", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyJ = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyK", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyK = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyL", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyL = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyM", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyM = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyN", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyN = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyO", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyO = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyP", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyP = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyQ", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyQ = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyR", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyR = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyS", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyS = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyT", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyT = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyU", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyU = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyV", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyV = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyW", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyW = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyX", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyX = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyY", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyY = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KeyZ", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KeyZ = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Minus", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Minus = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Period", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Period = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Quote", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Quote = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Semicolon", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Semicolon = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Slash", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Slash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AltLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::AltLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AltRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::AltRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Backspace", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Backspace = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("CapsLock", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::CapsLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ContextMenu", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ContextMenu = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ControlLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ControlLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ControlRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ControlRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Enter", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Enter = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("SuperLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::SuperLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("SuperRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::SuperRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ShiftLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ShiftLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ShiftRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ShiftRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Space", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Space = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Tab", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Tab = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Convert", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Convert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("KanaMode", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::KanaMode = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Lang1", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Lang1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Lang2", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Lang2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Lang3", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Lang3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Lang4", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Lang4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Lang5", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Lang5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NonConvert", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NonConvert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Delete", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Delete = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("End", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::End = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Help", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Help = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Home", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Home = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Insert", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Insert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("PageDown", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::PageDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("PageUp", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::PageUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ArrowDown", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ArrowDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ArrowLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ArrowLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ArrowRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ArrowRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ArrowUp", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ArrowUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumLock", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad0", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad0 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad1", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad2", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad3", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad4", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad5", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad6", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad7", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad8", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Numpad9", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Numpad9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadAdd", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadAdd = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadBackspace", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadBackspace = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadClear", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadClear = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadClearEntry", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadClearEntry = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadComma", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadComma = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadDecimal", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadDecimal = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadDivide", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadDivide = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadEnter", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadEnter = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadEqual", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadEqual = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadHash", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadHash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMemoryAdd", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMemoryAdd = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMemoryClear", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMemoryClear = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMemoryRecall", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMemoryRecall = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMemoryStore", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMemoryStore = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMemorySubtract", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMemorySubtract = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadMultiply", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadMultiply = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadParenLeft", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadParenLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadParenRight", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadParenRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadStar", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadStar = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("NumpadSubtract", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::NumpadSubtract = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Escape", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Escape = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Fn", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Fn = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("FnLock", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::FnLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("PrintScreen", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::PrintScreen = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("ScrollLock", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::ScrollLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Pause", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Pause = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserBack", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserBack = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserFavorites", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserFavorites = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserForward", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserForward = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserHome", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserHome = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserRefresh", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserRefresh = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserSearch", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserSearch = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("BrowserStop", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::BrowserStop = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Eject", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Eject = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("LaunchApp1", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::LaunchApp1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("LaunchApp2", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::LaunchApp2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("LaunchMail", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::LaunchMail = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MediaPlayPause", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::MediaPlayPause = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MediaSelect", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::MediaSelect = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MediaStop", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::MediaStop = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MediaTrackNext", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::MediaTrackNext = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("MediaTrackPrevious", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::MediaTrackPrevious = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Power", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Power = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Sleep", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Sleep = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AudioVolumeDown", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::AudioVolumeDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AudioVolumeMute", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::AudioVolumeMute = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("AudioVolumeUp", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::AudioVolumeUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("WakeUp", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::WakeUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Meta", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Meta = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Hyper", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Hyper = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Turbo", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Turbo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Abort", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Abort = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Resume", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Resume = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Suspend", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Suspend = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Again", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Again = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Copy", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Copy = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Cut", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Cut = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Find", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Find = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Open", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Open = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Paste", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Paste = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Props", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Props = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Select", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Select = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Undo", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Undo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Hiragana", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Hiragana = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("Katakana", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::Katakana = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F1", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F2", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F3", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F4", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F5", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F6", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F7", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F8", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F9", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F10", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F10 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F11", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F11 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F12", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F12 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F13", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F13 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F14", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F14 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F15", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F15 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F16", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F16 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F17", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F17 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F18", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F18 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F19", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F19 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F20", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F20 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F21", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F21 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F22", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F22 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F23", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F23 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F24", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F24 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F25", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F25 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F26", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F26 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F27", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F27 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F28", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F28 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F29", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F29 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F30", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F30 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F31", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F31 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F32", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F32 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F33", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F33 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F34", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F34 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });

        fields.add_field_method_get("F35", |lua, this| {
            let fyrox_lite::lite_event::KeyCode::F35 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
