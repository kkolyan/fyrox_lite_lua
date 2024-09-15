use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    scene::node::Node,
    script::{RoutingStrategy, ScriptMessagePayload, ScriptTrait},
};

use crate::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_physics::LiteRigidBody,
    script_context::with_script_context,
};
use fyrox::graph::BaseSceneGraph;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiteNode {
    handle: Handle<Node>,
}

impl From<Handle<Node>> for LiteNode {
    fn from(value: Handle<Node>) -> Self {
        LiteNode { handle: value }
    }
}

impl From<LiteNode> for Handle<Node> {
    fn from(value: LiteNode) -> Self {
        value.handle
    }
}

impl LiteNode {
    pub fn as_rigid_body(&mut self) -> Option<LiteRigidBody> {
        with_script_context(|ctx| {
            if ctx.scene.graph[self.handle].is_rigid_body() {
                Some(LiteRigidBody {
                    handle: self.handle,
                })
            } else {
                None
            }
        })
    }

    pub fn destroy(&mut self) {
        with_script_context(|ctx| ctx.scene.graph.remove_node(self.handle));
    }

    pub fn global_position(&self) -> LiteVector3 {
        with_script_context(|ctx| ctx.scene.graph[self.handle].global_position().into())
    }

    pub fn local_position(&self) -> LiteVector3 {
        with_script_context(|ctx| {
            (*ctx.scene.graph[self.handle]
                .local_transform()
                .position()
                .get_value_ref())
            .into()
        })
    }

    pub fn local_rotation(&self) -> LiteQuaternion {
        with_script_context(|ctx| {
            (*ctx.scene.graph[self.handle]
                .local_transform()
                .rotation()
                .get_value_ref())
            .into()
        })
    }

    /// Sends a hierarchical script message with the given payload.
    pub fn send_hierarchical<T>(&self, routing: RoutingStrategy, payload: T)
    where
        T: ScriptMessagePayload,
    {
        with_script_context(|ctx| {
            ctx.message_sender
                .send_hierarchical(self.handle, routing, payload);
        });
    }

    pub fn set_local_position(&self, new_pos: LiteVector3) {
        with_script_context(|ctx| {
            ctx.scene.graph[self.handle]
                .local_transform_mut()
                .set_position(new_pos.into());
        });
    }

    pub fn set_local_rotation(&self, value: LiteQuaternion) {
        with_script_context(|ctx| {
            ctx.scene.graph[self.handle]
                .local_transform_mut()
                .set_rotation(value.into());
        });
    }

    pub fn subscribe_to<T: 'static>(&self) {
        with_script_context(|ctx| {
            ctx.message_dispatcher.as_mut()
            .expect("cannot subscribe from on_message callback. do it in on_init, on_start or on_update")
            .subscribe_to::<T>(self.handle);
        });
    }

    pub fn try_get_collider(&self) -> Option<LiteNode> {
        with_script_context(|ctx| {
            ctx.scene.graph[self.handle]
                .children()
                .iter()
                .copied()
                .find(|it| ctx.scene.graph[*it].is_collider())
                .map(|it| it.into())
        })
    }

    pub fn is_valid(&self) -> bool {
        self.handle.is_some()
    }

    pub fn has_script<T: ScriptTrait>(&self) -> bool {
        with_script_context(|ctx| ctx.scene.graph[self.handle].has_script::<T>())
    }

    pub fn parent(&self) -> LiteNode {
        with_script_context(|ctx| ctx.scene.graph[self.handle].parent().into())
    }

    pub fn with_script<T: ScriptTrait>(&self, f: impl FnOnce(&mut T)) {
        with_script_context(|ctx| {
            let node = &mut ctx.scene.graph[self.handle];
            f(node.try_get_script_mut::<T>().unwrap());
        })
    }

    pub fn global_rotation(&self) -> LiteQuaternion {
        with_script_context(|ctx| {
            let camera_global_transform = ctx.scene.graph[self.handle].global_transform();

            let rot = camera_global_transform.fixed_view::<3, 3>(0, 0);
            UnitQuaternion::from_matrix(&rot.into()).into()
        })
    }
}
