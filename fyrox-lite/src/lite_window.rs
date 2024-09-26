use fyrox::{error::ExternalError, window::CursorGrabMode};

use crate::script_context::with_script_context;

pub struct LiteWindow;

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