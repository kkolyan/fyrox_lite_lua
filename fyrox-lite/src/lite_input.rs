use std::cell::RefCell;
use std::collections::{HashSet};
use fyrox::event::{DeviceEvent, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use fyrox::keyboard::{KeyCode, PhysicalKey};
use lite_macro::lite_api;
use crate::lite_math::PodVector2;

#[derive(Debug, Clone, Default)]
pub struct Input {
    keys_up: HashSet<LiteKeyCode>,
    keys_down: HashSet<LiteKeyCode>,
    keys: HashSet<LiteKeyCode>,
    mouse_button_up: HashSet<i32>,
    mouse_button_down: HashSet<i32>,
    mouse_button: HashSet<i32>,
    mouse_move: PodVector2,
    mouse_scroll: PodVector2,
}

thread_local! {
    static INSTANCE: RefCell<Option<Input>> = const { RefCell::new(None) };
}

fn with_input<R: Sized>(f: &mut dyn FnMut(&mut Input) -> R) -> R {
    INSTANCE.with_borrow_mut(|it| {
        f(it.as_mut().expect("Input system wasn't initialized"))
    })
}

#[allow(non_upper_case_globals)]
#[lite_api]
impl Input {
    pub const MouseLeft: i32 = 0;
    pub const MouseRight: i32 = 1;
    pub const MouseMiddle: i32 = 2;
    pub const MouseBack: i32 = 3;
    pub const MouseForward: i32 = 4;

    pub fn is_mouse_button_down(button: i32) -> bool {
        with_input(&mut |input| input.mouse_button_down.contains(&button))
    }
    pub fn is_mouse_button_up(button: i32) -> bool {
        with_input(&mut |input| input.mouse_button_up.contains(&button))
    }
    pub fn is_mouse_button_pressed(button: i32) -> bool {
        with_input(&mut |input| input.mouse_button.contains(&button))
    }

    pub fn is_key_down(key: LiteKeyCode) -> bool {
        with_input(&mut |input| input.keys_down.contains(&key))
    }
    pub fn is_key_up(key: LiteKeyCode) -> bool {
        with_input(&mut |input| input.keys_up.contains(&key))
    }
    pub fn is_key_pressed(key: LiteKeyCode) -> bool {
        with_input(&mut |input| input.keys.contains(&key))
    }

    pub fn get_mouse_move() -> PodVector2 {
        with_input(&mut |input| input.mouse_move)
    }

    pub fn get_mouse_scroll() -> PodVector2 {
        with_input(&mut |input| input.mouse_scroll)
    }
}

/// Basically a copy of `winit`'s [`KeyCode`], which is mostly inspired by UI Events Specification’s [`KeyboardEvent.code`].
///
/// [`KeyCode`]: https://docs.rs/winit/0.29.15/i686-pc-windows-msvc/winit/keyboard/enum.KeyCode.html
/// [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[lite_api(class=KeyCode)]
pub enum LiteKeyCode {
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>0</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    A,
    /// <kbd>b</kbd> on a US keyboard.
    B,
    /// <kbd>c</kbd> on a US keyboard.
    C,
    /// <kbd>d</kbd> on a US keyboard.
    D,
    /// <kbd>e</kbd> on a US keyboard.
    E,
    /// <kbd>f</kbd> on a US keyboard.
    F,
    /// <kbd>g</kbd> on a US keyboard.
    G,
    /// <kbd>h</kbd> on a US keyboard.
    H,
    /// <kbd>i</kbd> on a US keyboard.
    I,
    /// <kbd>j</kbd> on a US keyboard.
    J,
    /// <kbd>k</kbd> on a US keyboard.
    K,
    /// <kbd>l</kbd> on a US keyboard.
    L,
    /// <kbd>m</kbd> on a US keyboard.
    M,
    /// <kbd>n</kbd> on a US keyboard.
    N,
    /// <kbd>o</kbd> on a US keyboard.
    O,
    /// <kbd>p</kbd> on a US keyboard.
    P,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    Q,
    /// <kbd>r</kbd> on a US keyboard.
    R,
    /// <kbd>s</kbd> on a US keyboard.
    S,
    /// <kbd>t</kbd> on a US keyboard.
    T,
    /// <kbd>u</kbd> on a US keyboard.
    U,
    /// <kbd>v</kbd> on a US keyboard.
    V,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    W,
    /// <kbd>x</kbd> on a US keyboard.
    X,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    Y,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    Z,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Quote,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
}

impl TryFrom<KeyCode> for LiteKeyCode {
    type Error = ();

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyCode::Backquote => LiteKeyCode::Backquote,
            KeyCode::Backslash => LiteKeyCode::Backslash,
            KeyCode::BracketLeft => LiteKeyCode::BracketLeft,
            KeyCode::BracketRight => LiteKeyCode::BracketRight,
            KeyCode::Comma => LiteKeyCode::Comma,
            KeyCode::Digit0 => LiteKeyCode::Digit0,
            KeyCode::Digit1 => LiteKeyCode::Digit1,
            KeyCode::Digit2 => LiteKeyCode::Digit2,
            KeyCode::Digit3 => LiteKeyCode::Digit3,
            KeyCode::Digit4 => LiteKeyCode::Digit4,
            KeyCode::Digit5 => LiteKeyCode::Digit5,
            KeyCode::Digit6 => LiteKeyCode::Digit6,
            KeyCode::Digit7 => LiteKeyCode::Digit7,
            KeyCode::Digit8 => LiteKeyCode::Digit8,
            KeyCode::Digit9 => LiteKeyCode::Digit9,
            KeyCode::Equal => LiteKeyCode::Equal,
            KeyCode::IntlBackslash => LiteKeyCode::IntlBackslash,
            KeyCode::IntlRo => LiteKeyCode::IntlRo,
            KeyCode::IntlYen => LiteKeyCode::IntlYen,
            KeyCode::KeyA => LiteKeyCode::A,
            KeyCode::KeyB => LiteKeyCode::B,
            KeyCode::KeyC => LiteKeyCode::C,
            KeyCode::KeyD => LiteKeyCode::D,
            KeyCode::KeyE => LiteKeyCode::E,
            KeyCode::KeyF => LiteKeyCode::F,
            KeyCode::KeyG => LiteKeyCode::G,
            KeyCode::KeyH => LiteKeyCode::H,
            KeyCode::KeyI => LiteKeyCode::I,
            KeyCode::KeyJ => LiteKeyCode::J,
            KeyCode::KeyK => LiteKeyCode::K,
            KeyCode::KeyL => LiteKeyCode::L,
            KeyCode::KeyM => LiteKeyCode::M,
            KeyCode::KeyN => LiteKeyCode::N,
            KeyCode::KeyO => LiteKeyCode::O,
            KeyCode::KeyP => LiteKeyCode::P,
            KeyCode::KeyQ => LiteKeyCode::Q,
            KeyCode::KeyR => LiteKeyCode::R,
            KeyCode::KeyS => LiteKeyCode::S,
            KeyCode::KeyT => LiteKeyCode::T,
            KeyCode::KeyU => LiteKeyCode::U,
            KeyCode::KeyV => LiteKeyCode::V,
            KeyCode::KeyW => LiteKeyCode::W,
            KeyCode::KeyX => LiteKeyCode::X,
            KeyCode::KeyY => LiteKeyCode::Y,
            KeyCode::KeyZ => LiteKeyCode::Z,
            KeyCode::Minus => LiteKeyCode::Minus,
            KeyCode::Period => LiteKeyCode::Period,
            KeyCode::Quote => LiteKeyCode::Quote,
            KeyCode::Semicolon => LiteKeyCode::Semicolon,
            KeyCode::Slash => LiteKeyCode::Slash,
            KeyCode::AltLeft => LiteKeyCode::AltLeft,
            KeyCode::AltRight => LiteKeyCode::AltRight,
            KeyCode::Backspace => LiteKeyCode::Backspace,
            KeyCode::CapsLock => LiteKeyCode::CapsLock,
            KeyCode::ContextMenu => LiteKeyCode::ContextMenu,
            KeyCode::ControlLeft => LiteKeyCode::ControlLeft,
            KeyCode::ControlRight => LiteKeyCode::ControlRight,
            KeyCode::Enter => LiteKeyCode::Enter,
            KeyCode::SuperLeft => LiteKeyCode::SuperLeft,
            KeyCode::SuperRight => LiteKeyCode::SuperRight,
            KeyCode::ShiftLeft => LiteKeyCode::ShiftLeft,
            KeyCode::ShiftRight => LiteKeyCode::ShiftRight,
            KeyCode::Space => LiteKeyCode::Space,
            KeyCode::Tab => LiteKeyCode::Tab,
            KeyCode::Convert => LiteKeyCode::Convert,
            KeyCode::KanaMode => LiteKeyCode::KanaMode,
            KeyCode::Lang1 => LiteKeyCode::Lang1,
            KeyCode::Lang2 => LiteKeyCode::Lang2,
            KeyCode::Lang3 => LiteKeyCode::Lang3,
            KeyCode::Lang4 => LiteKeyCode::Lang4,
            KeyCode::Lang5 => LiteKeyCode::Lang5,
            KeyCode::NonConvert => LiteKeyCode::NonConvert,
            KeyCode::Delete => LiteKeyCode::Delete,
            KeyCode::End => LiteKeyCode::End,
            KeyCode::Help => LiteKeyCode::Help,
            KeyCode::Home => LiteKeyCode::Home,
            KeyCode::Insert => LiteKeyCode::Insert,
            KeyCode::PageDown => LiteKeyCode::PageDown,
            KeyCode::PageUp => LiteKeyCode::PageUp,
            KeyCode::ArrowDown => LiteKeyCode::ArrowDown,
            KeyCode::ArrowLeft => LiteKeyCode::ArrowLeft,
            KeyCode::ArrowRight => LiteKeyCode::ArrowRight,
            KeyCode::ArrowUp => LiteKeyCode::ArrowUp,
            KeyCode::NumLock => LiteKeyCode::NumLock,
            KeyCode::Numpad0 => LiteKeyCode::Numpad0,
            KeyCode::Numpad1 => LiteKeyCode::Numpad1,
            KeyCode::Numpad2 => LiteKeyCode::Numpad2,
            KeyCode::Numpad3 => LiteKeyCode::Numpad3,
            KeyCode::Numpad4 => LiteKeyCode::Numpad4,
            KeyCode::Numpad5 => LiteKeyCode::Numpad5,
            KeyCode::Numpad6 => LiteKeyCode::Numpad6,
            KeyCode::Numpad7 => LiteKeyCode::Numpad7,
            KeyCode::Numpad8 => LiteKeyCode::Numpad8,
            KeyCode::Numpad9 => LiteKeyCode::Numpad9,
            KeyCode::NumpadAdd => LiteKeyCode::NumpadAdd,
            KeyCode::NumpadBackspace => LiteKeyCode::NumpadBackspace,
            KeyCode::NumpadClear => LiteKeyCode::NumpadClear,
            KeyCode::NumpadClearEntry => LiteKeyCode::NumpadClearEntry,
            KeyCode::NumpadComma => LiteKeyCode::NumpadComma,
            KeyCode::NumpadDecimal => LiteKeyCode::NumpadDecimal,
            KeyCode::NumpadDivide => LiteKeyCode::NumpadDivide,
            KeyCode::NumpadEnter => LiteKeyCode::NumpadEnter,
            KeyCode::NumpadEqual => LiteKeyCode::NumpadEqual,
            KeyCode::NumpadHash => LiteKeyCode::NumpadHash,
            KeyCode::NumpadMemoryAdd => LiteKeyCode::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear => LiteKeyCode::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall => LiteKeyCode::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore => LiteKeyCode::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract => LiteKeyCode::NumpadMemorySubtract,
            KeyCode::NumpadMultiply => LiteKeyCode::NumpadMultiply,
            KeyCode::NumpadParenLeft => LiteKeyCode::NumpadParenLeft,
            KeyCode::NumpadParenRight => LiteKeyCode::NumpadParenRight,
            KeyCode::NumpadStar => LiteKeyCode::NumpadStar,
            KeyCode::NumpadSubtract => LiteKeyCode::NumpadSubtract,
            KeyCode::Escape => LiteKeyCode::Escape,
            KeyCode::Fn => LiteKeyCode::Fn,
            KeyCode::FnLock => LiteKeyCode::FnLock,
            KeyCode::PrintScreen => LiteKeyCode::PrintScreen,
            KeyCode::ScrollLock => LiteKeyCode::ScrollLock,
            KeyCode::Pause => LiteKeyCode::Pause,
            KeyCode::BrowserBack => LiteKeyCode::BrowserBack,
            KeyCode::BrowserFavorites => LiteKeyCode::BrowserFavorites,
            KeyCode::BrowserForward => LiteKeyCode::BrowserForward,
            KeyCode::BrowserHome => LiteKeyCode::BrowserHome,
            KeyCode::BrowserRefresh => LiteKeyCode::BrowserRefresh,
            KeyCode::BrowserSearch => LiteKeyCode::BrowserSearch,
            KeyCode::BrowserStop => LiteKeyCode::BrowserStop,
            KeyCode::Eject => LiteKeyCode::Eject,
            KeyCode::LaunchApp1 => LiteKeyCode::LaunchApp1,
            KeyCode::LaunchApp2 => LiteKeyCode::LaunchApp2,
            KeyCode::LaunchMail => LiteKeyCode::LaunchMail,
            KeyCode::MediaPlayPause => LiteKeyCode::MediaPlayPause,
            KeyCode::MediaSelect => LiteKeyCode::MediaSelect,
            KeyCode::MediaStop => LiteKeyCode::MediaStop,
            KeyCode::MediaTrackNext => LiteKeyCode::MediaTrackNext,
            KeyCode::MediaTrackPrevious => LiteKeyCode::MediaTrackPrevious,
            KeyCode::Power => LiteKeyCode::Power,
            KeyCode::Sleep => LiteKeyCode::Sleep,
            KeyCode::AudioVolumeDown => LiteKeyCode::AudioVolumeDown,
            KeyCode::AudioVolumeMute => LiteKeyCode::AudioVolumeMute,
            KeyCode::AudioVolumeUp => LiteKeyCode::AudioVolumeUp,
            KeyCode::WakeUp => LiteKeyCode::WakeUp,
            KeyCode::Meta => LiteKeyCode::Meta,
            KeyCode::Hyper => LiteKeyCode::Hyper,
            KeyCode::Turbo => LiteKeyCode::Turbo,
            KeyCode::Abort => LiteKeyCode::Abort,
            KeyCode::Resume => LiteKeyCode::Resume,
            KeyCode::Suspend => LiteKeyCode::Suspend,
            KeyCode::Again => LiteKeyCode::Again,
            KeyCode::Copy => LiteKeyCode::Copy,
            KeyCode::Cut => LiteKeyCode::Cut,
            KeyCode::Find => LiteKeyCode::Find,
            KeyCode::Open => LiteKeyCode::Open,
            KeyCode::Paste => LiteKeyCode::Paste,
            KeyCode::Props => LiteKeyCode::Props,
            KeyCode::Select => LiteKeyCode::Select,
            KeyCode::Undo => LiteKeyCode::Undo,
            KeyCode::Hiragana => LiteKeyCode::Hiragana,
            KeyCode::Katakana => LiteKeyCode::Katakana,
            KeyCode::F1 => LiteKeyCode::F1,
            KeyCode::F2 => LiteKeyCode::F2,
            KeyCode::F3 => LiteKeyCode::F3,
            KeyCode::F4 => LiteKeyCode::F4,
            KeyCode::F5 => LiteKeyCode::F5,
            KeyCode::F6 => LiteKeyCode::F6,
            KeyCode::F7 => LiteKeyCode::F7,
            KeyCode::F8 => LiteKeyCode::F8,
            KeyCode::F9 => LiteKeyCode::F9,
            KeyCode::F10 => LiteKeyCode::F10,
            KeyCode::F11 => LiteKeyCode::F11,
            KeyCode::F12 => LiteKeyCode::F12,
            KeyCode::F13 => LiteKeyCode::F13,
            KeyCode::F14 => LiteKeyCode::F14,
            KeyCode::F15 => LiteKeyCode::F15,
            KeyCode::F16 => LiteKeyCode::F16,
            KeyCode::F17 => LiteKeyCode::F17,
            KeyCode::F18 => LiteKeyCode::F18,
            KeyCode::F19 => LiteKeyCode::F19,
            KeyCode::F20 => LiteKeyCode::F20,
            KeyCode::F21 => LiteKeyCode::F21,
            KeyCode::F22 => LiteKeyCode::F22,
            KeyCode::F23 => LiteKeyCode::F23,
            KeyCode::F24 => LiteKeyCode::F24,
            KeyCode::F25 => LiteKeyCode::F25,
            KeyCode::F26 => LiteKeyCode::F26,
            KeyCode::F27 => LiteKeyCode::F27,
            KeyCode::F28 => LiteKeyCode::F28,
            KeyCode::F29 => LiteKeyCode::F29,
            KeyCode::F30 => LiteKeyCode::F30,
            KeyCode::F31 => LiteKeyCode::F31,
            KeyCode::F32 => LiteKeyCode::F32,
            KeyCode::F33 => LiteKeyCode::F33,
            KeyCode::F34 => LiteKeyCode::F34,
            KeyCode::F35 => LiteKeyCode::F35,
            _ => return Err(()),
        })
    }
}

impl Input {

    /// should be called after fixed frame update
    pub fn post_fixed_update() {
        with_input(&mut |it| {
            it.mouse_move = Default::default();
            it.mouse_scroll = Default::default();
            it.mouse_button_down.clear();
            it.mouse_button_up.clear();
            it.keys_up.clear();
            it.keys_down.clear();
        })
    }

    /// should be called at the beginning.
    pub fn init_thread_local_state() {
        INSTANCE.set(Some(Input::default()));
    }

    pub fn on_os_event(event: &Event<()>) {
        with_input(&mut |input| {
            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::KeyboardInput { event, .. } => {
                            match event.physical_key {
                                PhysicalKey::Code(key_code) => {
                                    if let Ok(key_code) = key_code.try_into() {
                                        match event.state {
                                            ElementState::Pressed => {
                                                input.keys_down.insert(key_code);
                                                input.keys.insert(key_code);
                                            }
                                            ElementState::Released => {
                                                input.keys_up.insert(key_code);
                                                input.keys.remove(&key_code);
                                            }
                                        }
                                    }
                                }
                                PhysicalKey::Unidentified(_) => {}
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let button = match button {
                                MouseButton::Left => Input::MouseLeft,
                                MouseButton::Right => Input::MouseRight,
                                MouseButton::Middle => Input::MouseMiddle,
                                MouseButton::Back => Input::MouseBack,
                                MouseButton::Forward => Input::MouseForward,
                                // TODO does it continue numeration of known buttons or overlap with them?
                                MouseButton::Other(it) => *it as i32,
                            };
                            match state {
                                ElementState::Pressed => {
                                    input.mouse_button_down.insert(button);
                                    input.mouse_button.insert(button);
                                }
                                ElementState::Released => {
                                    input.mouse_button_up.insert(button);
                                    input.mouse_button.remove(&button);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { event,.. } => {
                    match event {
                        DeviceEvent::Added => {}
                        DeviceEvent::Removed => {}
                        DeviceEvent::MouseMotion { delta: (x, y),.. } => {
                            input.mouse_move.x += *x as f32;
                            input.mouse_move.y += *y as f32;
                        }
                        DeviceEvent::MouseWheel { delta,.. } => {
                            match delta {
                                MouseScrollDelta::LineDelta(lite_macro, _) => {}
                                MouseScrollDelta::PixelDelta(_) => {}
                            }
                        }
                        DeviceEvent::Motion { .. } => {}
                        DeviceEvent::Button { .. } => {}
                        DeviceEvent::Key(_) => {}
                    }
                }
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::AboutToWait => {}
                Event::LoopExiting => {}
                Event::MemoryWarning => {}
            }
        });
    }
}
