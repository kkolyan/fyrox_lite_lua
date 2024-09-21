use std::mem;

use fyrox::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use mlua::{IntoLua, Lua, MetaMethod, UserData, Value};

use crate::{fyrox_lite::Traitor, lua_error};

#[derive(Clone)]
pub struct RustEnum {
    discriminant: &'static str,
    fields: Fields,
}

#[derive(Clone)]
pub enum Fields {
    Unit,
    Named(Vec<(&'static str, Value<'static>)>),
    Positional(Vec<Value<'static>>),
}

impl RustEnum {
    pub fn unit(discriminant: &'static str) -> Self {
        Self {
            discriminant,
            fields: Fields::Unit,
        }
    }

    pub fn struct_like(
        discriminant: &'static str,
        properties: Vec<(&'static str, Value<'_>)>,
    ) -> Self {
        Self {
            discriminant,
            fields: Fields::Named(
                properties
                    .into_iter()
                    .map(|(k, value)| {
                        // Safety: it's sound, because we use Lua during the whole life of application
                        let value: Value<'static> = unsafe { mem::transmute(value) };
                        (k, value)
                    })
                    .collect(),
            ),
        }
    }
    pub fn tuple_like(discriminant: &'static str, properties: Vec<Value<'_>>) -> Self {
        Self {
            discriminant,
            fields: Fields::Positional(
                properties
                    .into_iter()
                    .map(|value| {
                        // Safety: it's sound, because we use Lua during the whole life of application
                        let value: Value<'static> = unsafe { mem::transmute(value) };
                        value
                    })
                    .collect(),
            ),
        }
    }
}

impl UserData for RustEnum {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is", |_lua, this, discriminant: mlua::String| {
            Ok(this.discriminant == discriminant.to_str().unwrap())
        });

        methods.add_method(MetaMethod::Index.name(), |_lua, this, key: mlua::Value| {
            match &this.fields {
                Fields::Unit => {
					Err(lua_error!("constant-like enum {} doesn't have any data. attempted to access with key {:?}", this.discriminant, key))
				}
                Fields::Named(fields) => {
					let key_str = key.as_str().ok_or_else(|| lua_error!("struct-like enum {} fields cannot be accessed with non-string key {:?}", this.discriminant, key))?;
					for (k, value) in fields.iter() {
						if *k == key_str {
							return Ok(Value::clone(value));
						}
					}
					Err(lua_error!("struct-like enum {} doesn't have a value for key {:?}", this.discriminant, key))
				}
                Fields::Positional(fields) => {
                    let index = key.as_usize().ok_or_else(|| lua_error!("tuple-like enum {} fields cannot be accessed with non-index key {:?}", this.discriminant, key))?;
                    Ok(fields.get(index - 1).ok_or_else(|| lua_error!("index out of bounds: {}", index))?.clone())
                }
            }
        });
    }
}

impl<'lua, T> IntoLua<'lua> for Traitor<Event<T>> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
            Event::NewEvents(_it) => RustEnum::unit("NewEvents"),
            Event::WindowEvent { window_id: _, event } => RustEnum::struct_like(
                "WindowEvent",
                vec![("event", Traitor::new(event.clone()).into_lua(lua)?)],
            ),
            Event::DeviceEvent { device_id: _, event } => RustEnum::struct_like(
                "DeviceEvent",
                vec![
					("event", Traitor::new(event.clone()).into_lua(lua)?)
				],
            ),
            Event::UserEvent(_it) => RustEnum::unit("UserEvent"),
            Event::Suspended => RustEnum::unit("Suspended"),
            Event::Resumed => RustEnum::unit("Resumed"),
            Event::AboutToWait => RustEnum::unit("AboutToWait"),
            Event::LoopExiting => RustEnum::unit("LoopExiting"),
            Event::MemoryWarning => RustEnum::unit("MemoryWarning"),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<WindowEvent> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			WindowEvent::ActivationTokenDone { serial: _, token: _ } => RustEnum::unit("ActivationTokenDone"),
			WindowEvent::Resized(_physical_size) => RustEnum::unit("Resized"),
			WindowEvent::Moved(_physical_position) => RustEnum::unit("Moved"),
			WindowEvent::CloseRequested => RustEnum::unit("CloseRequested"),
			WindowEvent::Destroyed => RustEnum::unit("Destroyed"),
			WindowEvent::DroppedFile(_path_buf) => RustEnum::unit("DroppedFile"),
			WindowEvent::HoveredFile(_path_buf) => RustEnum::unit("HoveredFile"),
			WindowEvent::HoveredFileCancelled => RustEnum::unit("HoveredFileCancelled"),
			WindowEvent::Focused(_) => RustEnum::unit("Focused"),
			WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => RustEnum::struct_like(
				"KeyboardInput",
				vec![
					("event", Traitor::new(event.clone()).into_lua(lua)?)
				]
			),
			WindowEvent::ModifiersChanged(_modifiers) => RustEnum::unit("ModifiersChanged"),
			WindowEvent::Ime(_ime) => RustEnum::unit("Ime"),
			WindowEvent::CursorMoved { device_id: _, position: _ } => RustEnum::unit("CursorMoved"),
			WindowEvent::CursorEntered { device_id: _ } => RustEnum::unit("CursorEntered"),
			WindowEvent::CursorLeft { device_id: _ } => RustEnum::unit("CursorLeft"),
			WindowEvent::MouseWheel { device_id: _, delta: _, phase: _ } => RustEnum::unit("MouseWheel"),
			WindowEvent::MouseInput { device_id: _, state, button } => RustEnum::struct_like(
				"MouseInput", 
				vec![
					("state", Traitor::new(*state).into_lua(lua)?),
					("button", Traitor::new(*button).into_lua(lua)?),
				]
			),
			WindowEvent::TouchpadMagnify { device_id: _, delta: _, phase: _ } => RustEnum::unit("TouchpadMagnify"),
			WindowEvent::SmartMagnify { device_id: _ } => RustEnum::unit("SmartMagnify"),
			WindowEvent::TouchpadRotate { device_id: _, delta: _, phase: _ } => RustEnum::unit("TouchpadRotate"),
			WindowEvent::TouchpadPressure { device_id: _, pressure: _, stage: _ } => RustEnum::unit("TouchpadPressure"),
			WindowEvent::AxisMotion { device_id: _, axis: _, value: _ } => RustEnum::unit("AxisMotion"),
			WindowEvent::Touch(_touch) => RustEnum::unit("Touch"),
			WindowEvent::ScaleFactorChanged { scale_factor: _, inner_size_writer: _ } => RustEnum::unit("ScaleFactorChanged"),
			WindowEvent::ThemeChanged(_theme) => RustEnum::unit("ThemeChanged"),
			WindowEvent::Occluded(_) => RustEnum::unit("Occluded"),
			WindowEvent::RedrawRequested => RustEnum::unit("RedrawRequested"),
		};
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<DeviceEvent> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			DeviceEvent::Added => RustEnum::unit("Added"),
			DeviceEvent::Removed => RustEnum::unit("Removed"),
			DeviceEvent::MouseMotion { delta } => {
				let delta_as_t = lua.create_table()?;
				delta_as_t.set(1, delta.0)?;
				delta_as_t.set(2, delta.1)?;
				RustEnum::struct_like(
					"MouseMotion", 
					vec![
						("delta", Value::Table(delta_as_t))
					]
				)
			},
			DeviceEvent::MouseWheel { delta: _ } => RustEnum::unit("MouseWheel"),
			DeviceEvent::Motion { axis: _, value: _ } => RustEnum::unit("Motion"),
			DeviceEvent::Button { button: _, state: _ } => RustEnum::unit("Button"),
			DeviceEvent::Key(_raw_key_event) => RustEnum::unit("Key"),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<ElementState> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			ElementState::Pressed => RustEnum::unit("Pressed"),
			ElementState::Released => RustEnum::unit("Released"),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<PhysicalKey> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			PhysicalKey::Code(key_code) => RustEnum::tuple_like(
				"Code", 
				vec![
				Traitor::new(*key_code).into_lua(lua)?
			]),
			PhysicalKey::Unidentified(_native_key_code) => RustEnum::unit("Unidentified"),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<KeyEvent> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let t = lua.create_table()?;
        t.set("physical_key", Traitor::new(self.physical_key))?;
        t.set("state", Traitor::new(self.state))?;
        Ok(Value::Table(t))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<MouseButton> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			MouseButton::Left => RustEnum::unit("Left"),
			MouseButton::Right => RustEnum::unit("Right"),
			MouseButton::Middle => RustEnum::unit("Middle"),
			MouseButton::Back => RustEnum::unit("Back"),
			MouseButton::Forward => RustEnum::unit("Forward"),
			MouseButton::Other(it) => RustEnum::tuple_like("Other", vec![Value::Number(*it as f64)]),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}

impl<'lua> IntoLua<'lua> for Traitor<KeyCode> {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        #[rustfmt::skip]
        let re = match self.inner() {
			KeyCode::Backquote => RustEnum::unit("Backquote"),
			KeyCode::Backslash => RustEnum::unit("Backslash"),
			KeyCode::BracketLeft => RustEnum::unit("BracketLeft"),
			KeyCode::BracketRight => RustEnum::unit("BracketRight"),
			KeyCode::Comma => RustEnum::unit("Comma"),
			KeyCode::Digit0 => RustEnum::unit("Digit0"),
			KeyCode::Digit1 => RustEnum::unit("Digit1"),
			KeyCode::Digit2 => RustEnum::unit("Digit2"),
			KeyCode::Digit3 => RustEnum::unit("Digit3"),
			KeyCode::Digit4 => RustEnum::unit("Digit4"),
			KeyCode::Digit5 => RustEnum::unit("Digit5"),
			KeyCode::Digit6 => RustEnum::unit("Digit6"),
			KeyCode::Digit7 => RustEnum::unit("Digit7"),
			KeyCode::Digit8 => RustEnum::unit("Digit8"),
			KeyCode::Digit9 => RustEnum::unit("Digit9"),
			KeyCode::Equal => RustEnum::unit("Equal"),
			KeyCode::IntlBackslash => RustEnum::unit("IntlBackslash"),
			KeyCode::IntlRo => RustEnum::unit("IntlRo"),
			KeyCode::IntlYen => RustEnum::unit("IntlYen"),
			KeyCode::KeyA => RustEnum::unit("KeyA"),
			KeyCode::KeyB => RustEnum::unit("KeyB"),
			KeyCode::KeyC => RustEnum::unit("KeyC"),
			KeyCode::KeyD => RustEnum::unit("KeyD"),
			KeyCode::KeyE => RustEnum::unit("KeyE"),
			KeyCode::KeyF => RustEnum::unit("KeyF"),
			KeyCode::KeyG => RustEnum::unit("KeyG"),
			KeyCode::KeyH => RustEnum::unit("KeyH"),
			KeyCode::KeyI => RustEnum::unit("KeyI"),
			KeyCode::KeyJ => RustEnum::unit("KeyJ"),
			KeyCode::KeyK => RustEnum::unit("KeyK"),
			KeyCode::KeyL => RustEnum::unit("KeyL"),
			KeyCode::KeyM => RustEnum::unit("KeyM"),
			KeyCode::KeyN => RustEnum::unit("KeyN"),
			KeyCode::KeyO => RustEnum::unit("KeyO"),
			KeyCode::KeyP => RustEnum::unit("KeyP"),
			KeyCode::KeyQ => RustEnum::unit("KeyQ"),
			KeyCode::KeyR => RustEnum::unit("KeyR"),
			KeyCode::KeyS => RustEnum::unit("KeyS"),
			KeyCode::KeyT => RustEnum::unit("KeyT"),
			KeyCode::KeyU => RustEnum::unit("KeyU"),
			KeyCode::KeyV => RustEnum::unit("KeyV"),
			KeyCode::KeyW => RustEnum::unit("KeyW"),
			KeyCode::KeyX => RustEnum::unit("KeyX"),
			KeyCode::KeyY => RustEnum::unit("KeyY"),
			KeyCode::KeyZ => RustEnum::unit("KeyZ"),
			KeyCode::Minus => RustEnum::unit("Minus"),
			KeyCode::Period => RustEnum::unit("Period"),
			KeyCode::Quote => RustEnum::unit("Quote"),
			KeyCode::Semicolon => RustEnum::unit("Semicolon"),
			KeyCode::Slash => RustEnum::unit("Slash"),
			KeyCode::AltLeft => RustEnum::unit("AltLeft"),
			KeyCode::AltRight => RustEnum::unit("AltRight"),
			KeyCode::Backspace => RustEnum::unit("Backspace"),
			KeyCode::CapsLock => RustEnum::unit("CapsLock"),
			KeyCode::ContextMenu => RustEnum::unit("ContextMenu"),
			KeyCode::ControlLeft => RustEnum::unit("ControlLeft"),
			KeyCode::ControlRight => RustEnum::unit("ControlRight"),
			KeyCode::Enter => RustEnum::unit("Enter"),
			KeyCode::SuperLeft => RustEnum::unit("SuperLeft"),
			KeyCode::SuperRight => RustEnum::unit("SuperRight"),
			KeyCode::ShiftLeft => RustEnum::unit("ShiftLeft"),
			KeyCode::ShiftRight => RustEnum::unit("ShiftRight"),
			KeyCode::Space => RustEnum::unit("Space"),
			KeyCode::Tab => RustEnum::unit("Tab"),
			KeyCode::Convert => RustEnum::unit("Convert"),
			KeyCode::KanaMode => RustEnum::unit("KanaMode"),
			KeyCode::Lang1 => RustEnum::unit("Lang1"),
			KeyCode::Lang2 => RustEnum::unit("Lang2"),
			KeyCode::Lang3 => RustEnum::unit("Lang3"),
			KeyCode::Lang4 => RustEnum::unit("Lang4"),
			KeyCode::Lang5 => RustEnum::unit("Lang5"),
			KeyCode::NonConvert => RustEnum::unit("NonConvert"),
			KeyCode::Delete => RustEnum::unit("Delete"),
			KeyCode::End => RustEnum::unit("End"),
			KeyCode::Help => RustEnum::unit("Help"),
			KeyCode::Home => RustEnum::unit("Home"),
			KeyCode::Insert => RustEnum::unit("Insert"),
			KeyCode::PageDown => RustEnum::unit("PageDown"),
			KeyCode::PageUp => RustEnum::unit("PageUp"),
			KeyCode::ArrowDown => RustEnum::unit("ArrowDown"),
			KeyCode::ArrowLeft => RustEnum::unit("ArrowLeft"),
			KeyCode::ArrowRight => RustEnum::unit("ArrowRight"),
			KeyCode::ArrowUp => RustEnum::unit("ArrowUp"),
			KeyCode::NumLock => RustEnum::unit("NumLock"),
			KeyCode::Numpad0 => RustEnum::unit("Numpad0"),
			KeyCode::Numpad1 => RustEnum::unit("Numpad1"),
			KeyCode::Numpad2 => RustEnum::unit("Numpad2"),
			KeyCode::Numpad3 => RustEnum::unit("Numpad3"),
			KeyCode::Numpad4 => RustEnum::unit("Numpad4"),
			KeyCode::Numpad5 => RustEnum::unit("Numpad5"),
			KeyCode::Numpad6 => RustEnum::unit("Numpad6"),
			KeyCode::Numpad7 => RustEnum::unit("Numpad7"),
			KeyCode::Numpad8 => RustEnum::unit("Numpad8"),
			KeyCode::Numpad9 => RustEnum::unit("Numpad9"),
			KeyCode::NumpadAdd => RustEnum::unit("NumpadAdd"),
			KeyCode::NumpadBackspace => RustEnum::unit("NumpadBackspace"),
			KeyCode::NumpadClear => RustEnum::unit("NumpadClear"),
			KeyCode::NumpadClearEntry => RustEnum::unit("NumpadClearEntry"),
			KeyCode::NumpadComma => RustEnum::unit("NumpadComma"),
			KeyCode::NumpadDecimal => RustEnum::unit("NumpadDecimal"),
			KeyCode::NumpadDivide => RustEnum::unit("NumpadDivide"),
			KeyCode::NumpadEnter => RustEnum::unit("NumpadEnter"),
			KeyCode::NumpadEqual => RustEnum::unit("NumpadEqual"),
			KeyCode::NumpadHash => RustEnum::unit("NumpadHash"),
			KeyCode::NumpadMemoryAdd => RustEnum::unit("NumpadMemoryAdd"),
			KeyCode::NumpadMemoryClear => RustEnum::unit("NumpadMemoryClear"),
			KeyCode::NumpadMemoryRecall => RustEnum::unit("NumpadMemoryRecall"),
			KeyCode::NumpadMemoryStore => RustEnum::unit("NumpadMemoryStore"),
			KeyCode::NumpadMemorySubtract => RustEnum::unit("NumpadMemorySubtract"),
			KeyCode::NumpadMultiply => RustEnum::unit("NumpadMultiply"),
			KeyCode::NumpadParenLeft => RustEnum::unit("NumpadParenLeft"),
			KeyCode::NumpadParenRight => RustEnum::unit("NumpadParenRight"),
			KeyCode::NumpadStar => RustEnum::unit("NumpadStar"),
			KeyCode::NumpadSubtract => RustEnum::unit("NumpadSubtract"),
			KeyCode::Escape => RustEnum::unit("Escape"),
			KeyCode::Fn => RustEnum::unit("Fn"),
			KeyCode::FnLock => RustEnum::unit("FnLock"),
			KeyCode::PrintScreen => RustEnum::unit("PrintScreen"),
			KeyCode::ScrollLock => RustEnum::unit("ScrollLock"),
			KeyCode::Pause => RustEnum::unit("Pause"),
			KeyCode::BrowserBack => RustEnum::unit("BrowserBack"),
			KeyCode::BrowserFavorites => RustEnum::unit("BrowserFavorites"),
			KeyCode::BrowserForward => RustEnum::unit("BrowserForward"),
			KeyCode::BrowserHome => RustEnum::unit("BrowserHome"),
			KeyCode::BrowserRefresh => RustEnum::unit("BrowserRefresh"),
			KeyCode::BrowserSearch => RustEnum::unit("BrowserSearch"),
			KeyCode::BrowserStop => RustEnum::unit("BrowserStop"),
			KeyCode::Eject => RustEnum::unit("Eject"),
			KeyCode::LaunchApp1 => RustEnum::unit("LaunchApp1"),
			KeyCode::LaunchApp2 => RustEnum::unit("LaunchApp2"),
			KeyCode::LaunchMail => RustEnum::unit("LaunchMail"),
			KeyCode::MediaPlayPause => RustEnum::unit("MediaPlayPause"),
			KeyCode::MediaSelect => RustEnum::unit("MediaSelect"),
			KeyCode::MediaStop => RustEnum::unit("MediaStop"),
			KeyCode::MediaTrackNext => RustEnum::unit("MediaTrackNext"),
			KeyCode::MediaTrackPrevious => RustEnum::unit("MediaTrackPrevious"),
			KeyCode::Power => RustEnum::unit("Power"),
			KeyCode::Sleep => RustEnum::unit("Sleep"),
			KeyCode::AudioVolumeDown => RustEnum::unit("AudioVolumeDown"),
			KeyCode::AudioVolumeMute => RustEnum::unit("AudioVolumeMute"),
			KeyCode::AudioVolumeUp => RustEnum::unit("AudioVolumeUp"),
			KeyCode::WakeUp => RustEnum::unit("WakeUp"),
			KeyCode::Meta => RustEnum::unit("Meta"),
			KeyCode::Hyper => RustEnum::unit("Hyper"),
			KeyCode::Turbo => RustEnum::unit("Turbo"),
			KeyCode::Abort => RustEnum::unit("Abort"),
			KeyCode::Resume => RustEnum::unit("Resume"),
			KeyCode::Suspend => RustEnum::unit("Suspend"),
			KeyCode::Again => RustEnum::unit("Again"),
			KeyCode::Copy => RustEnum::unit("Copy"),
			KeyCode::Cut => RustEnum::unit("Cut"),
			KeyCode::Find => RustEnum::unit("Find"),
			KeyCode::Open => RustEnum::unit("Open"),
			KeyCode::Paste => RustEnum::unit("Paste"),
			KeyCode::Props => RustEnum::unit("Props"),
			KeyCode::Select => RustEnum::unit("Select"),
			KeyCode::Undo => RustEnum::unit("Undo"),
			KeyCode::Hiragana => RustEnum::unit("Hiragana"),
			KeyCode::Katakana => RustEnum::unit("Katakana"),
			KeyCode::F1 => RustEnum::unit("F1"),
			KeyCode::F2 => RustEnum::unit("F2"),
			KeyCode::F3 => RustEnum::unit("F3"),
			KeyCode::F4 => RustEnum::unit("F4"),
			KeyCode::F5 => RustEnum::unit("F5"),
			KeyCode::F6 => RustEnum::unit("F6"),
			KeyCode::F7 => RustEnum::unit("F7"),
			KeyCode::F8 => RustEnum::unit("F8"),
			KeyCode::F9 => RustEnum::unit("F9"),
			KeyCode::F10 => RustEnum::unit("F10"),
			KeyCode::F11 => RustEnum::unit("F11"),
			KeyCode::F12 => RustEnum::unit("F12"),
			KeyCode::F13 => RustEnum::unit("F13"),
			KeyCode::F14 => RustEnum::unit("F14"),
			KeyCode::F15 => RustEnum::unit("F15"),
			KeyCode::F16 => RustEnum::unit("F16"),
			KeyCode::F17 => RustEnum::unit("F17"),
			KeyCode::F18 => RustEnum::unit("F18"),
			KeyCode::F19 => RustEnum::unit("F19"),
			KeyCode::F20 => RustEnum::unit("F20"),
			KeyCode::F21 => RustEnum::unit("F21"),
			KeyCode::F22 => RustEnum::unit("F22"),
			KeyCode::F23 => RustEnum::unit("F23"),
			KeyCode::F24 => RustEnum::unit("F24"),
			KeyCode::F25 => RustEnum::unit("F25"),
			KeyCode::F26 => RustEnum::unit("F26"),
			KeyCode::F27 => RustEnum::unit("F27"),
			KeyCode::F28 => RustEnum::unit("F28"),
			KeyCode::F29 => RustEnum::unit("F29"),
			KeyCode::F30 => RustEnum::unit("F30"),
			KeyCode::F31 => RustEnum::unit("F31"),
			KeyCode::F32 => RustEnum::unit("F32"),
			KeyCode::F33 => RustEnum::unit("F33"),
			KeyCode::F34 => RustEnum::unit("F34"),
			KeyCode::F35 => RustEnum::unit("F35"),
			_ => RustEnum::unit("_"),
        };
        Ok(Value::UserData(lua.create_userdata(re)?))
    }
}
