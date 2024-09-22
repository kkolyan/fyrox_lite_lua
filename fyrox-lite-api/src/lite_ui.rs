use fyrox::{core::pool::Handle, gui::UiNode};

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