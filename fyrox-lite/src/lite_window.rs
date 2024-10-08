use std::fmt::Debug;

use fyrox::{core::log::Log, window::CursorGrabMode};
use lite_macro::lite_api;

use crate::script_context::with_script_context;

#[derive(Debug, Clone, Copy)]
pub struct LiteWindow;

#[lite_api(class=Window)]
impl LiteWindow {
    pub fn set_cursor_grab(mode: LiteCursorGrabMode) {
        with_script_context(|ctx| {
            let result = ctx
                .graphics_context
                .as_initialized_mut()
                .window
                .set_cursor_grab(match mode {
                    LiteCursorGrabMode::None => CursorGrabMode::None,
                    LiteCursorGrabMode::Confined => CursorGrabMode::Confined,
                    LiteCursorGrabMode::Locked => CursorGrabMode::Locked,
                });
            if let Err(err) = result {
                Log::err(format!("set_cursor_grab failed: {}", err));
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[lite_api(class=CursorGrabMode)]
pub enum LiteCursorGrabMode {
    /// No grabbing of the cursor is performed.
    None,

    /// The cursor is confined to the window area.
    ///
    /// There's no guarantee that the cursor will be hidden. You should hide it by yourself if you
    /// want to do so.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** Not implemented. Always returns [`ExternalError::NotSupported`] for now.
    /// - **iOS / Android / Web:** Always returns an [`ExternalError::NotSupported`].
    Confined,

    /// The cursor is locked inside the window area to the certain position.
    ///
    /// There's no guarantee that the cursor will be hidden. You should hide it by yourself if you
    /// want to do so.
    ///
    /// ## Platform-specific
    ///
    /// - **X11 / Windows:** Not implemented. Always returns [`ExternalError::NotSupported`] for now.
    /// - **iOS / Android:** Always returns an [`ExternalError::NotSupported`].
    Locked,
}
