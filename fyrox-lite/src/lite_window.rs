use fyrox::{error::ExternalError, window::CursorGrabMode};
use fyrox_lite_macro::fyrox_lite_engine_class;

use crate::script_context::with_script_context;

pub struct LiteWindow;

#[fyrox_lite_engine_class("Window")]
impl LiteWindow {
	pub fn set_cursor_grab(mode: CursorGrabMode) -> Result<(), ExternalError> {
		with_script_context(|ctx| {
			ctx
				.graphics_context
				.as_initialized_mut()
				.window
				.set_cursor_grab(mode)
		})
	}
}