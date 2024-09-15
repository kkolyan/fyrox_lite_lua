use fyrox::{core::pool::Handle, scene::node::Node};

pub struct LiteMessages;

impl LiteMessages {
    pub fn send_hierarchical<T>(root: Handle<Node>, routing: RoutingStrategy, payload: T)
    where
        T: ScriptMessagePayload,
    {
		
    }
}
