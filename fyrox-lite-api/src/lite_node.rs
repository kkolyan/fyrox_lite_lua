use crate::wrapper_reflect;
use std::{any::Any, fmt::Debug};

use fyrox::{
    core::{algebra::UnitQuaternion, pool::Handle, reflect::*, visitor::Visit},
    scene::node::Node,
    script::{RoutingStrategy, ScriptMessagePayload, ScriptTrait},
};

use crate::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_physics::LiteRigidBody,
    script_context::with_script_context,
};
use fyrox::graph::BaseSceneGraph;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct LiteNode {
    handle: Handle<Node>,
}

impl Debug for LiteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.handle, f)
    }
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
            if ctx.scene.as_ref().expect("scene unavailable").graph[self.handle].is_rigid_body() {
                Some(LiteRigidBody {
                    handle: self.handle,
                })
            } else {
                None
            }
        })
    }

    pub fn name(&self) -> Option<String> {
        with_script_context(|ctx| {
            ctx.scene
                .as_ref()
                .expect("scene unavailable")
                .graph
                .try_get(self.handle)
                .map(|it| it.name_owned())
        })
    }

    pub fn is_alive(&self) -> bool {
        with_script_context(|ctx| {
            ctx.scene
                .as_ref()
                .expect("scene unavailable")
                .graph
                .try_get(self.handle)
                .is_some()
        })
    }

    pub fn destroy(&mut self) {
        with_script_context(|ctx| {
            ctx.scene
                .as_mut()
                .expect("scene unavailable")
                .graph
                .remove_node(self.handle)
        });
    }

    pub fn global_position(&self) -> LiteVector3 {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .global_position()
                .into()
        })
    }

    pub fn local_position(&self) -> LiteVector3 {
        with_script_context(|ctx| {
            (*ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .local_transform()
                .position()
                .get_value_ref())
            .into()
        })
    }

    pub fn local_rotation(&self) -> LiteQuaternion {
        with_script_context(|ctx| {
            (*ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .local_transform()
                .rotation()
                .get_value_ref())
            .into()
        })
    }

    /// Sends a hierarchical script message with the given payload.
    pub fn send_hierarchical<T>(&self, routing: LiteRoutingStrategy, payload: T)
    where
        T: ScriptMessagePayload,
        T: Debug,
    {
        with_script_context(|ctx| {
            let routing = match routing {
                LiteRoutingStrategy::Up => RoutingStrategy::Up,
                LiteRoutingStrategy::Down => RoutingStrategy::Down,
            };
            ctx.message_sender
                .as_mut()
                .expect("message sender unavailable")
                .send_hierarchical(self.handle, routing, payload);
        });
    }

    pub fn set_local_position(&self, new_pos: LiteVector3) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .local_transform_mut()
                .set_position(new_pos.into());
        });
    }

    pub fn set_local_rotation(&self, value: LiteQuaternion) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
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
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .children()
                .iter()
                .copied()
                .find(|it| ctx.scene.as_ref().expect("scene unavailable").graph[*it].is_collider())
                .map(|it| it.into())
        })
    }

    pub fn is_valid(&self) -> bool {
        self.handle.is_some()
    }

    pub fn parent(&self) -> LiteNode {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .parent()
                .into()
        })
    }

    pub fn with_script<T: ScriptTrait>(&self, f: impl FnOnce(&mut T)) {
        with_script_context(|ctx| {
            let node = &mut ctx.scene.as_mut().expect("scene unavailable").graph[self.handle];
            f(node.try_get_script_mut::<T>().unwrap());
        })
    }

    pub fn add_script<T: ScriptTrait>(&self, state: T) {
        with_script_context(|ctx| {
            let node = &mut ctx.scene.as_mut().expect("scene unavailable").graph[self.handle];
            node.add_script(state);
        })
    }

    pub fn find_script<T: ScriptTrait, R>(&self, f: impl Fn(&mut T) -> Option<R>) -> Option<R> {
        with_script_context(|ctx| {
            let node = &mut ctx.scene.as_mut().expect("scene unavailable").graph[self.handle];
            for script in node.try_get_scripts_mut::<T>() {
                if let Some(r) = f(script) {
                    return Some(r);
                }
            }
            None
        })
    }

    pub fn global_rotation(&self) -> LiteQuaternion {
        with_script_context(|ctx| {
            let camera_global_transform = ctx.scene.as_mut().expect("scene unavailable").graph
                [self.handle]
                .global_transform();

            let rot = camera_global_transform.fixed_view::<3, 3>(0, 0);
            UnitQuaternion::from_matrix(&rot.into()).into()
        })
    }

    pub fn tag_is(&self, tag: &str) -> bool {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle].tag() == tag
        })
    }

    pub fn set_tag(&self, tag: &str) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .set_tag(tag.to_string());
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LiteRoutingStrategy {
    /// An message will be passed to the specified root node and then to every node up in the hierarchy.
    Up,
    /// An message will be passed to every node down the tree in the hierarchy.
    Down,
}

impl Visit for LiteNode {
    fn visit(
        &mut self,
        name: &str,
        visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        self.handle.visit(name, visitor)
    }
}

impl Reflect for LiteNode {
    wrapper_reflect!(handle);

    fn source_path() -> &'static str
    where
        Self: Sized,
    {
        file!()
    }

    fn assembly_name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn type_assembly_name() -> &'static str
    where
        Self: Sized,
    {
        env!("CARGO_PKG_NAME")
    }
}

#[macro_export]
macro_rules! wrapper_reflect {
    ($ident:tt) => {
        fn type_name(&self) -> &'static str {
            self.$ident.type_name()
        }

        fn doc(&self) -> &'static str {
            self.$ident.doc()
        }

        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            self.$ident.fields_info(func)
        }

        fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }

        fn as_any(&self, func: &mut dyn FnMut(&dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any(&self.$ident, func)
        }

        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any_mut(&mut self.$ident, func)
        }

        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            self.$ident.as_reflect(func)
        }

        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            self.$ident.as_reflect_mut(func)
        }

        fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            self.$ident.set(value)
        }

        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            self.$ident.field(name, func)
        }

        fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
            self.$ident.field_mut(name, func)
        }

        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            self.$ident.as_array(func)
        }

        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            self.$ident.as_array_mut(func)
        }

        fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
            self.$ident.as_list(func)
        }

        fn as_list_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectList>)) {
            self.$ident.as_list_mut(func)
        }

        fn as_inheritable_variable(
            &self,
            func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable(func)
        }

        fn as_inheritable_variable_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable_mut(func)
        }

        fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
            self.$ident.as_hash_map(func)
        }

        fn as_hash_map_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>)) {
            self.$ident.as_hash_map_mut(func)
        }
    };
}
