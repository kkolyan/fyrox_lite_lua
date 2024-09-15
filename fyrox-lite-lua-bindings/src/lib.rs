use std::ops::{Deref, DerefMut};

use fyrox_lite_api::{lite_math::LiteVector3, lite_node::LiteNode, lite_physics::LiteRigidBody};
use mlua::{IntoLuaMulti, UserData, Value::Nil};

struct Traitor<T>(pub T);

impl<T> Deref for Traitor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Traitor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UserData for Traitor<LiteRigidBody> {}

impl UserData for Traitor<LiteVector3> {}


impl UserData for Traitor<LiteNode> {

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("parent", |a, b, args: ()| Ok(Traitor(b.parent())));
        methods.add_method_mut("as_rigid_body", |a, b, args: ()| {
            Ok(b.as_rigid_body().map(Traitor))
        });
        methods.add_method_mut("destroy", |a, b, args: ()| {
            b.destroy();
            Ok(())
        });
        methods.add_method_mut("global_position", |a, b, args: ()| {
            Ok(Traitor(b.global_position()))
        });
    }
}

fn sa(s: &mut LiteNode) {
    s.destroy()
}

/*

pub fn global_position(&self) -> Vector3<f32> {
    with_script_context(|ctx| ctx.scene.graph[self.handle].global_position())
}

pub fn local_position(&self) -> Vector3<f32> {
    with_script_context(|ctx| {
        *ctx.scene.graph[self.handle]
            .local_transform()
            .position()
            .get_value_ref()
    })
}

pub fn local_rotation(&self) -> UnitQuaternion<f32> {
    with_script_context(|ctx| {
        *ctx.scene.graph[self.handle]
            .local_transform()
            .rotation()
            .get_value_ref()
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

pub fn set_local_position(&self, new_pos: Vector3<f32>) {
    with_script_context(|ctx| {
        ctx.scene.graph[self.handle]
            .local_transform_mut()
            .set_position(new_pos);
    });
}

pub fn set_local_rotation(&self, value: UnitQuaternion<f32>) {
    with_script_context(|ctx| {
        ctx.scene.graph[self.handle]
            .local_transform_mut()
            .set_rotation(value);
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

pub fn global_rotation(&self) -> UnitQuaternion<f32> { */
