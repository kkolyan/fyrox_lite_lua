
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
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite::lite_input::LiteKeyCode {
    const CLASS_NAME: &'static str = "KeyCode";
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Backquote", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Backquote))
        });
        fields.add_field_method_get("Backslash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Backslash))
        });
        fields.add_field_method_get("BracketLeft", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BracketLeft,
            ))
        });
        fields.add_field_method_get("BracketRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BracketRight,
            ))
        });
        fields.add_field_method_get("Comma", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Comma))
        });
        fields.add_field_method_get("Digit0", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit0))
        });
        fields.add_field_method_get("Digit1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit1))
        });
        fields.add_field_method_get("Digit2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit2))
        });
        fields.add_field_method_get("Digit3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit3))
        });
        fields.add_field_method_get("Digit4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit4))
        });
        fields.add_field_method_get("Digit5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit5))
        });
        fields.add_field_method_get("Digit6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit6))
        });
        fields.add_field_method_get("Digit7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit7))
        });
        fields.add_field_method_get("Digit8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit8))
        });
        fields.add_field_method_get("Digit9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Digit9))
        });
        fields.add_field_method_get("Equal", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Equal))
        });
        fields.add_field_method_get("IntlBackslash", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::IntlBackslash,
            ))
        });
        fields.add_field_method_get("IntlRo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::IntlRo))
        });
        fields.add_field_method_get("IntlYen", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::IntlYen))
        });
        fields.add_field_method_get("A", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::A))
        });
        fields.add_field_method_get("B", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::B))
        });
        fields.add_field_method_get("C", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::C))
        });
        fields.add_field_method_get("D", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::D))
        });
        fields.add_field_method_get("E", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::E))
        });
        fields.add_field_method_get("F", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F))
        });
        fields.add_field_method_get("G", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::G))
        });
        fields.add_field_method_get("H", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::H))
        });
        fields.add_field_method_get("I", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::I))
        });
        fields.add_field_method_get("J", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::J))
        });
        fields.add_field_method_get("K", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::K))
        });
        fields.add_field_method_get("L", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::L))
        });
        fields.add_field_method_get("M", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::M))
        });
        fields.add_field_method_get("N", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::N))
        });
        fields.add_field_method_get("O", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::O))
        });
        fields.add_field_method_get("P", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::P))
        });
        fields.add_field_method_get("Q", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Q))
        });
        fields.add_field_method_get("R", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::R))
        });
        fields.add_field_method_get("S", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::S))
        });
        fields.add_field_method_get("T", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::T))
        });
        fields.add_field_method_get("U", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::U))
        });
        fields.add_field_method_get("V", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::V))
        });
        fields.add_field_method_get("W", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::W))
        });
        fields.add_field_method_get("X", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::X))
        });
        fields.add_field_method_get("Y", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Y))
        });
        fields.add_field_method_get("Z", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Z))
        });
        fields.add_field_method_get("Minus", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Minus))
        });
        fields.add_field_method_get("Period", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Period))
        });
        fields.add_field_method_get("Quote", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Quote))
        });
        fields.add_field_method_get("Semicolon", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Semicolon))
        });
        fields.add_field_method_get("Slash", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Slash))
        });
        fields.add_field_method_get("AltLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::AltLeft))
        });
        fields.add_field_method_get("AltRight", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::AltRight))
        });
        fields.add_field_method_get("Backspace", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Backspace))
        });
        fields.add_field_method_get("CapsLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::CapsLock))
        });
        fields.add_field_method_get("ContextMenu", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ContextMenu,
            ))
        });
        fields.add_field_method_get("ControlLeft", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ControlLeft,
            ))
        });
        fields.add_field_method_get("ControlRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ControlRight,
            ))
        });
        fields.add_field_method_get("Enter", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Enter))
        });
        fields.add_field_method_get("SuperLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::SuperLeft))
        });
        fields.add_field_method_get("SuperRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::SuperRight,
            ))
        });
        fields.add_field_method_get("ShiftLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::ShiftLeft))
        });
        fields.add_field_method_get("ShiftRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ShiftRight,
            ))
        });
        fields.add_field_method_get("Space", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Space))
        });
        fields.add_field_method_get("Tab", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Tab))
        });
        fields.add_field_method_get("Convert", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Convert))
        });
        fields.add_field_method_get("KanaMode", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::KanaMode))
        });
        fields.add_field_method_get("Lang1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Lang1))
        });
        fields.add_field_method_get("Lang2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Lang2))
        });
        fields.add_field_method_get("Lang3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Lang3))
        });
        fields.add_field_method_get("Lang4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Lang4))
        });
        fields.add_field_method_get("Lang5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Lang5))
        });
        fields.add_field_method_get("NonConvert", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NonConvert,
            ))
        });
        fields.add_field_method_get("Delete", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Delete))
        });
        fields.add_field_method_get("End", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::End))
        });
        fields.add_field_method_get("Help", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Help))
        });
        fields.add_field_method_get("Home", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Home))
        });
        fields.add_field_method_get("Insert", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Insert))
        });
        fields.add_field_method_get("PageDown", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::PageDown))
        });
        fields.add_field_method_get("PageUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::PageUp))
        });
        fields.add_field_method_get("ArrowDown", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::ArrowDown))
        });
        fields.add_field_method_get("ArrowLeft", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::ArrowLeft))
        });
        fields.add_field_method_get("ArrowRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ArrowRight,
            ))
        });
        fields.add_field_method_get("ArrowUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::ArrowUp))
        });
        fields.add_field_method_get("NumLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::NumLock))
        });
        fields.add_field_method_get("Numpad0", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad0))
        });
        fields.add_field_method_get("Numpad1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad1))
        });
        fields.add_field_method_get("Numpad2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad2))
        });
        fields.add_field_method_get("Numpad3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad3))
        });
        fields.add_field_method_get("Numpad4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad4))
        });
        fields.add_field_method_get("Numpad5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad5))
        });
        fields.add_field_method_get("Numpad6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad6))
        });
        fields.add_field_method_get("Numpad7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad7))
        });
        fields.add_field_method_get("Numpad8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad8))
        });
        fields.add_field_method_get("Numpad9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Numpad9))
        });
        fields.add_field_method_get("NumpadAdd", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::NumpadAdd))
        });
        fields.add_field_method_get("NumpadBackspace", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadBackspace,
            ))
        });
        fields.add_field_method_get("NumpadClear", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadClear,
            ))
        });
        fields.add_field_method_get("NumpadClearEntry", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadClearEntry,
            ))
        });
        fields.add_field_method_get("NumpadComma", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadComma,
            ))
        });
        fields.add_field_method_get("NumpadDecimal", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadDecimal,
            ))
        });
        fields.add_field_method_get("NumpadDivide", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadDivide,
            ))
        });
        fields.add_field_method_get("NumpadEnter", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadEnter,
            ))
        });
        fields.add_field_method_get("NumpadEqual", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadEqual,
            ))
        });
        fields.add_field_method_get("NumpadHash", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadHash,
            ))
        });
        fields.add_field_method_get("NumpadMemoryAdd", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryAdd,
            ))
        });
        fields.add_field_method_get("NumpadMemoryClear", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryClear,
            ))
        });
        fields.add_field_method_get("NumpadMemoryRecall", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryRecall,
            ))
        });
        fields.add_field_method_get("NumpadMemoryStore", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryStore,
            ))
        });
        fields.add_field_method_get("NumpadMemorySubtract", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMemorySubtract,
            ))
        });
        fields.add_field_method_get("NumpadMultiply", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadMultiply,
            ))
        });
        fields.add_field_method_get("NumpadParenLeft", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadParenLeft,
            ))
        });
        fields.add_field_method_get("NumpadParenRight", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadParenRight,
            ))
        });
        fields.add_field_method_get("NumpadStar", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadStar,
            ))
        });
        fields.add_field_method_get("NumpadSubtract", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::NumpadSubtract,
            ))
        });
        fields.add_field_method_get("Escape", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Escape))
        });
        fields.add_field_method_get("Fn", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Fn))
        });
        fields.add_field_method_get("FnLock", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::FnLock))
        });
        fields.add_field_method_get("PrintScreen", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::PrintScreen,
            ))
        });
        fields.add_field_method_get("ScrollLock", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::ScrollLock,
            ))
        });
        fields.add_field_method_get("Pause", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Pause))
        });
        fields.add_field_method_get("BrowserBack", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserBack,
            ))
        });
        fields.add_field_method_get("BrowserFavorites", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserFavorites,
            ))
        });
        fields.add_field_method_get("BrowserForward", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserForward,
            ))
        });
        fields.add_field_method_get("BrowserHome", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserHome,
            ))
        });
        fields.add_field_method_get("BrowserRefresh", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserRefresh,
            ))
        });
        fields.add_field_method_get("BrowserSearch", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserSearch,
            ))
        });
        fields.add_field_method_get("BrowserStop", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::BrowserStop,
            ))
        });
        fields.add_field_method_get("Eject", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Eject))
        });
        fields.add_field_method_get("LaunchApp1", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::LaunchApp1,
            ))
        });
        fields.add_field_method_get("LaunchApp2", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::LaunchApp2,
            ))
        });
        fields.add_field_method_get("LaunchMail", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::LaunchMail,
            ))
        });
        fields.add_field_method_get("MediaPlayPause", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::MediaPlayPause,
            ))
        });
        fields.add_field_method_get("MediaSelect", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::MediaSelect,
            ))
        });
        fields.add_field_method_get("MediaStop", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::MediaStop))
        });
        fields.add_field_method_get("MediaTrackNext", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::MediaTrackNext,
            ))
        });
        fields.add_field_method_get("MediaTrackPrevious", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::MediaTrackPrevious,
            ))
        });
        fields.add_field_method_get("Power", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Power))
        });
        fields.add_field_method_get("Sleep", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Sleep))
        });
        fields.add_field_method_get("AudioVolumeDown", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::AudioVolumeDown,
            ))
        });
        fields.add_field_method_get("AudioVolumeMute", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::AudioVolumeMute,
            ))
        });
        fields.add_field_method_get("AudioVolumeUp", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_input::LiteKeyCode::AudioVolumeUp,
            ))
        });
        fields.add_field_method_get("WakeUp", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::WakeUp))
        });
        fields.add_field_method_get("Meta", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Meta))
        });
        fields.add_field_method_get("Hyper", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Hyper))
        });
        fields.add_field_method_get("Turbo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Turbo))
        });
        fields.add_field_method_get("Abort", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Abort))
        });
        fields.add_field_method_get("Resume", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Resume))
        });
        fields.add_field_method_get("Suspend", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Suspend))
        });
        fields.add_field_method_get("Again", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Again))
        });
        fields.add_field_method_get("Copy", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Copy))
        });
        fields.add_field_method_get("Cut", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Cut))
        });
        fields.add_field_method_get("Find", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Find))
        });
        fields.add_field_method_get("Open", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Open))
        });
        fields.add_field_method_get("Paste", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Paste))
        });
        fields.add_field_method_get("Props", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Props))
        });
        fields.add_field_method_get("Select", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Select))
        });
        fields.add_field_method_get("Undo", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Undo))
        });
        fields.add_field_method_get("Hiragana", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Hiragana))
        });
        fields.add_field_method_get("Katakana", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::Katakana))
        });
        fields.add_field_method_get("F1", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F1))
        });
        fields.add_field_method_get("F2", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F2))
        });
        fields.add_field_method_get("F3", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F3))
        });
        fields.add_field_method_get("F4", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F4))
        });
        fields.add_field_method_get("F5", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F5))
        });
        fields.add_field_method_get("F6", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F6))
        });
        fields.add_field_method_get("F7", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F7))
        });
        fields.add_field_method_get("F8", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F8))
        });
        fields.add_field_method_get("F9", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F9))
        });
        fields.add_field_method_get("F10", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F10))
        });
        fields.add_field_method_get("F11", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F11))
        });
        fields.add_field_method_get("F12", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F12))
        });
        fields.add_field_method_get("F13", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F13))
        });
        fields.add_field_method_get("F14", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F14))
        });
        fields.add_field_method_get("F15", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F15))
        });
        fields.add_field_method_get("F16", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F16))
        });
        fields.add_field_method_get("F17", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F17))
        });
        fields.add_field_method_get("F18", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F18))
        });
        fields.add_field_method_get("F19", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F19))
        });
        fields.add_field_method_get("F20", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F20))
        });
        fields.add_field_method_get("F21", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F21))
        });
        fields.add_field_method_get("F22", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F22))
        });
        fields.add_field_method_get("F23", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F23))
        });
        fields.add_field_method_get("F24", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F24))
        });
        fields.add_field_method_get("F25", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F25))
        });
        fields.add_field_method_get("F26", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F26))
        });
        fields.add_field_method_get("F27", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F27))
        });
        fields.add_field_method_get("F28", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F28))
        });
        fields.add_field_method_get("F29", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F29))
        });
        fields.add_field_method_get("F30", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F30))
        });
        fields.add_field_method_get("F31", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F31))
        });
        fields.add_field_method_get("F32", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F32))
        });
        fields.add_field_method_get("F33", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F33))
        });
        fields.add_field_method_get("F34", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F34))
        });
        fields.add_field_method_get("F35", |lua, this| {
            Ok(Traitor::new(fyrox_lite::lite_input::LiteKeyCode::F35))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Backquote", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Backquote = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Backslash", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Backslash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BracketLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BracketLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BracketRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BracketRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Comma", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Comma = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit0", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit0 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit1", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit2", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit3", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit4", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit5", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit6", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit7", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit8", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Digit9", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Digit9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Equal", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Equal = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("IntlBackslash", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::IntlBackslash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("IntlRo", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::IntlRo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("IntlYen", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::IntlYen = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("A", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::A = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("B", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::B = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("C", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::C = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("D", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::D = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("E", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::E = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("G", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::G = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("H", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::H = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("I", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::I = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("J", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::J = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("K", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::K = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("L", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::L = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("M", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::M = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("N", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::N = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("O", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::O = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("P", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::P = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Q", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Q = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("R", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::R = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("S", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::S = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("T", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::T = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("U", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::U = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("V", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::V = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("W", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::W = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("X", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::X = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Y", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Y = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Z", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Z = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Minus", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Minus = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Period", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Period = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Quote", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Quote = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Semicolon", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Semicolon = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Slash", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Slash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("AltLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::AltLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("AltRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::AltRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Backspace", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Backspace = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("CapsLock", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::CapsLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ContextMenu", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ContextMenu = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ControlLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ControlLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ControlRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ControlRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Enter", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Enter = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("SuperLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::SuperLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("SuperRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::SuperRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ShiftLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ShiftLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ShiftRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ShiftRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Space", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Space = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Tab", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Tab = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Convert", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Convert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("KanaMode", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::KanaMode = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Lang1", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Lang1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Lang2", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Lang2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Lang3", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Lang3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Lang4", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Lang4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Lang5", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Lang5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NonConvert", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NonConvert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Delete", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Delete = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("End", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::End = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Help", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Help = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Home", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Home = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Insert", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Insert = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("PageDown", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::PageDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("PageUp", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::PageUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ArrowDown", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ArrowDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ArrowLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ArrowLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ArrowRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ArrowRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ArrowUp", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ArrowUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumLock", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad0", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad0 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad1", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad2", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad3", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad4", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad5", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad6", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad7", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad8", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Numpad9", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Numpad9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadAdd", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadAdd = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadBackspace", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadBackspace = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadClear", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadClear = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadClearEntry", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadClearEntry = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadComma", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadComma = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadDecimal", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadDecimal = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadDivide", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadDivide = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadEnter", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadEnter = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadEqual", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadEqual = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadHash", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadHash = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMemoryAdd", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryAdd = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMemoryClear", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryClear = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMemoryRecall", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryRecall = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMemoryStore", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMemoryStore = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMemorySubtract", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMemorySubtract = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadMultiply", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadMultiply = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadParenLeft", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadParenLeft = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadParenRight", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadParenRight = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadStar", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadStar = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("NumpadSubtract", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::NumpadSubtract = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Escape", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Escape = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Fn", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Fn = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("FnLock", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::FnLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("PrintScreen", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::PrintScreen = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("ScrollLock", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::ScrollLock = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Pause", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Pause = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserBack", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserBack = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserFavorites", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserFavorites = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserForward", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserForward = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserHome", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserHome = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserRefresh", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserRefresh = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserSearch", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserSearch = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("BrowserStop", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::BrowserStop = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Eject", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Eject = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("LaunchApp1", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::LaunchApp1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("LaunchApp2", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::LaunchApp2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("LaunchMail", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::LaunchMail = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("MediaPlayPause", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::MediaPlayPause = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("MediaSelect", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::MediaSelect = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("MediaStop", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::MediaStop = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("MediaTrackNext", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::MediaTrackNext = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("MediaTrackPrevious", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::MediaTrackPrevious = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Power", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Power = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Sleep", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Sleep = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("AudioVolumeDown", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeDown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("AudioVolumeMute", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeMute = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("AudioVolumeUp", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::AudioVolumeUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("WakeUp", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::WakeUp = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Meta", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Meta = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Hyper", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Hyper = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Turbo", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Turbo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Abort", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Abort = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Resume", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Resume = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Suspend", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Suspend = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Again", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Again = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Copy", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Copy = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Cut", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Cut = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Find", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Find = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Open", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Open = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Paste", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Paste = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Props", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Props = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Select", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Select = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Undo", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Undo = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Hiragana", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Hiragana = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Katakana", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::Katakana = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F1", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F1 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F2", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F2 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F3", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F3 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F4", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F4 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F5", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F5 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F6", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F6 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F7", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F7 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F8", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F8 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F9", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F9 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F10", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F10 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F11", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F11 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F12", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F12 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F13", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F13 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F14", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F14 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F15", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F15 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F16", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F16 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F17", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F17 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F18", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F18 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F19", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F19 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F20", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F20 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F21", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F21 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F22", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F22 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F23", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F23 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F24", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F24 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F25", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F25 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F26", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F26 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F27", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F27 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F28", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F28 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F29", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F29 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F30", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F30 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F31", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F31 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F32", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F32 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F33", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F33 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F34", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F34 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("F35", |lua, this| {
            let fyrox_lite::lite_input::LiteKeyCode::F35 = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
