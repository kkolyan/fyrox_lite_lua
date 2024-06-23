use fyrox::{
    core::pool::Handle,
    scene::{node::Node, Scene},
    script::ScriptContext,
};

pub fn print_ancestors(ctx: &mut ScriptContext, mut handle: Handle<Node>) {
    println!("Node hierarchy:");
    loop {
        let mut node = &mut ctx.scene.graph.try_get_mut(handle);
        let Some(node) = node else {
            break;
        };
        println!("  {}: {}", node.name(), node.type_name());
        handle = node.parent();
    }
}

#[extend::ext]
pub impl Handle<Node> {
    fn try_get_collider(&self, scene: &Scene) -> Option<Handle<Node>> {
        scene.graph[*self]
            .children()
            .iter()
            .copied()
            .find(|it| scene.graph[*it].is_collider())
    }
}
