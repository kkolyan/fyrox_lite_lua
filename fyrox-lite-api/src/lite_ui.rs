use fyrox::{
    core::pool::Handle,
    gui::{message::MessageDirection, text::{TextBuilder, TextMessage}, UiNode},
};

use crate::script_context::{with_script_context, OptionaUiContainerExt};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteUiNode {
    handle: Handle<UiNode>,
}

impl LiteUiNode {
    pub fn new(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }

    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LiteText {
    handle: Handle<UiNode>,
}

impl LiteText {
    pub fn from(handle: Handle<UiNode>) -> Self {
        Self { handle }
    }

    pub fn new(builder: TextBuilder) -> LiteText {
        with_script_context(|ctx| LiteText {
            handle: builder.build(&mut ctx.ui().first_mut().build_ctx()),
        })
    }

    pub fn inner(&self) -> Handle<UiNode> {
        self.handle
    }

    pub fn set_text_async(&self, text: String) {
        with_script_context(|ctx| {
            ctx.ui().first_mut().send_message(TextMessage::text(
                self.handle,
                MessageDirection::ToWidget,
                text,
            ));
        })
    }
}
