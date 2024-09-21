use std::{
    mem,
    ops::{Deref, DerefMut},
};

use crate::script::LuaScript;
use fyrox_lite_api::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_node::{LiteNode, LiteRoutingStrategy},
    lite_physics::LiteRigidBody,
};
use mlua::{AnyUserData, UserData, UserDataRef, UserDataRefMut, Value};
use send_wrapper::SendWrapper;

#[derive(Clone, Copy, Debug)]
pub struct Traitor<T>(T);

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

impl<T> Traitor<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }
    
    pub fn inner(&self) -> &T {
        &self.0
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteRigidBody> {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut(
            "name",
            |lua, this, force: UserDataRef<Traitor<LiteVector3>>| {
                this.apply_force(*force.inner());
                Ok(())
            },
        );
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteVector3> {
    #[rustfmt::skip]
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| Ok(this.0.get_x()));
        fields.add_field_method_get("y", |lua, this| Ok(this.0.get_y()));
        fields.add_field_method_get("z", |lua, this| Ok(this.0.get_z()));

        fields.add_field_method_set("x", |lua, this, value: f32| { this.0.set_x(value); Ok(()) });
        fields.add_field_method_set("y", |lua, this, value: f32| { this.0.set_y(value); Ok(()) });
        fields.add_field_method_set("z", |lua, this, value: f32| { this.0.set_z(value); Ok(()) });

        fields.add_field_function_get("X", |lua, args| Ok(Traitor(LiteVector3::x_axis())));
        fields.add_field_function_get("Y", |lua, args| Ok(Traitor(LiteVector3::y_axis())));
        fields.add_field_function_get("X", |lua, args| Ok(Traitor(LiteVector3::z_axis())));

        fields.add_field_function_get("ZERO", |lua, args| Ok(Traitor(LiteVector3::zero())));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |lua, (x, y, z): (f32, f32, f32)| {
            Ok(Traitor(LiteVector3::new(x, y, z)))
        });

        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteVector3>>, f32)| Ok(Traitor(this.mul(o))),
        );

        methods.add_function(
            "add",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor(this.add(*o.inner()))) },
        );
        methods.add_function(
            "sub",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor(this.sub(*o.inner()))) },
        );
        methods.add_function(
            "normalize",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(Traitor(this.normalize())),
        );
        methods.add_function(
            "magnitude",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(this.magnitude()),
        );
        methods.add_function_mut(
            "normalize_inplace",
            |lua, mut this: UserDataRefMut<Traitor<LiteVector3>>| {
                this.normalize_inplace();
                Ok(())
            },
        );
    }
}

#[allow(unused_variables)]
impl UserData for Traitor<LiteQuaternion> {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "face_towards",
            |lua,
             (dir, up): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| {
                Ok(Traitor(LiteQuaternion::face_towards(
                    *dir.inner(),
                    *up.inner(),
                )))
            },
        );
        methods.add_function(
            "from_axis_angle",
            |lua, (axis, angle): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor(LiteQuaternion::from_axis_angle(
                    *axis.inner(),
                    angle,
                )))
            },
        );
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteQuaternion>>, AnyUserData)| {
                if let Ok(o) = o.borrow::<Traitor<LiteVector3>>() {
                    return Ok(lua.create_userdata(Traitor(this.mul__LiteVector(o.0))));
                }
                if let Ok(o) = o.borrow::<Traitor<LiteQuaternion>>() {
                    return Ok(lua.create_any_userdata(Traitor(this.mul__LiteQuaternion(o.0))));
                }
                Err(mlua::Error::runtime("invalid operand type"))
            },
        );
    }
}

impl UserData for Traitor<LiteRoutingStrategy> {}

#[allow(unused_variables)]
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
        methods.add_method_mut("global_rotation", |a, b, args: ()| {
            Ok(Traitor(b.global_rotation()))
        });
        methods.add_method_mut("local_position", |a, b, args: ()| {
            Ok(Traitor(b.local_position()))
        });
        methods.add_method_mut("local_rotation", |a, b, args: ()| {
            Ok(Traitor(b.local_rotation()))
        });
        methods.add_method_mut(
            "send_hierarchical",
            |a, b, (routing_strategy, value): (UserDataRef<Traitor<LiteRoutingStrategy>>, Value)| {
                // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
                let value: Value<'static> = unsafe { mem::transmute(value) };
                b.send_hierarchical(routing_strategy.0, SendWrapper::new(value));
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_position",
            |a, b, value: UserDataRef<Traitor<LiteVector3>>| {
                b.set_local_position(value.0);
                Ok(())
            },
        );
        methods.add_method_mut(
            "set_local_rotation",
            |a, b, value: UserDataRef<Traitor<LiteQuaternion>>| {
                b.set_local_rotation(value.0);
                Ok(())
            },
        );
        methods.add_method_mut("subscribe_to", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            b.subscribe_to::<SendWrapper<Value>>();
            Ok(())
        });
        methods.add_method("find_collider_in_children", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.try_get_collider().map(Traitor))
        });
        methods.add_method("is_valid", |a, b, args: ()| {
            // TODO contribute to Fyrox a way to identify subscription by a value
            Ok(b.is_valid())
        });
        methods.add_method("get_script", |a, b, name: mlua::String| {
            b.find_script::<LuaScript, _>(|it| {
                if it.name == name.to_string_lossy() {
                    let script_data = it.data.clone();
                    return Some(script_data);
                }
                None
            });
            Ok(())
        });
    }
}
