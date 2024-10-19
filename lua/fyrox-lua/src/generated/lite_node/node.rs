
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite::lite_node::LiteNode {
    const CLASS_NAME: &'static str = "Node";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });

        methods.add_meta_method(
            mlua::MetaMethod::Eq.name(),
            |lua, this, args: TypedUserData<Traitor<Self>>| {
                Ok(<Self as PartialEq>::eq(
                    this.inner(),
                    args.borrow()?.inner(),
                ))
            },
        );

        methods.add_method_mut("as_rigid_body", |lua, this, (): ()| {
            let ret = this.as_rigid_body();
            let ret = if let Some(ret) = ret {
                Some(Traitor::new(fyrox_lite::lite_physics::LiteRigidBody::from(
                    ret,
                )))
            } else {
                None
            };
            Ok(ret)
        });

        methods.add_method_mut("destroy", |lua, this, (): ()| {
            let ret = this.destroy();
            let ret = ret;
            Ok(ret)
        });

        methods.add_method_mut(
            "send_hierarchical",
            |lua,
             this,
             (routing, payload): (
                TypedUserData<Traitor<fyrox_lite::lite_node::LiteRoutingStrategy>>,
                mlua::Value,
            )| {
                let routing = routing.borrow()?.inner().clone().into();

                let payload = Traitor::new(send_wrapper::SendWrapper::new(unsafe {
                    std::mem::transmute::<mlua::Value<'_>, mlua::Value<'static>>(payload)
                }));

                let ret = this
                    .send_hierarchical::<TypedUserData<Traitor<ScriptObject>>>(routing, payload);
                let ret = ret;
                Ok(ret)
            },
        );

        methods.add_method_mut("subscribe_to", |lua, this, (): ()| {
            let _stub = Default::default();

            let ret = this.subscribe_to::<TypedUserData<Traitor<ScriptObject>>>(_stub);
            let ret = ret;
            Ok(ret)
        });

        methods.add_method_mut("find_collider_in_children", |lua, this, (): ()| {
            let ret = this.find_collider_in_children();
            let ret = if let Some(ret) = ret {
                Some(Traitor::new(fyrox_lite::lite_node::LiteNode::from(ret)))
            } else {
                None
            };
            Ok(ret)
        });

        methods.add_method_mut("add_script", |lua, this, (class_name): (mlua::String)| {
            let class_name = class_name.to_str()?.to_string();

            let _stub = Default::default();

            let ret = this.add_script::<TypedUserData<Traitor<ScriptObject>>>(class_name, _stub);
            let ret = match ret {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
            Ok(ret)
        });

        methods.add_method_mut("find_script", |lua, this, (class_name): (mlua::String)| {
            let class_name = class_name.to_str()?.to_string();

            let _stub = Default::default();

            let ret = this.find_script::<TypedUserData<Traitor<ScriptObject>>>(class_name, _stub);
            let ret = match ret {
                Ok(ret) => {
                    if let Some(ret) = ret {
                        Some(ret)
                    } else {
                        None
                    }
                }
                Err(err) => return Err(err),
            };
            Ok(ret)
        });

        methods.add_method_mut("tag_is", |lua, this, (tag): (mlua::String)| {
            let tag = tag.to_str()?.to_string();

            let ret = this.tag_is(tag);
            let ret = ret;
            Ok(ret)
        });
    }

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("name", |lua, this| {
            let value = this.get_name();
            Ok(if let Some(value) = value {
                Some(value)
            } else {
                None
            })
        });

        fields.add_field_method_get("alive", |lua, this| {
            let value = this.get_alive();
            Ok(value)
        });

        fields.add_field_method_get("global_position", |lua, this| {
            let value = this.get_global_position();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(
                value,
            )))
        });

        fields.add_field_method_get("local_position", |lua, this| {
            let value = this.get_local_position();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(
                value,
            )))
        });

        fields.add_field_method_get("local_rotation", |lua, this| {
            let value = this.get_local_rotation();
            Ok(Traitor::new(
                fyrox_lite_math::lite_math::LiteQuaternion::from(value),
            ))
        });

        fields.add_field_method_get("valid", |lua, this| {
            let value = this.get_valid();
            Ok(value)
        });

        fields.add_field_method_get("parent", |lua, this| {
            let value = this.get_parent();
            Ok(Traitor::new(fyrox_lite::lite_node::LiteNode::from(value)))
        });

        fields.add_field_method_get("global_rotation", |lua, this| {
            let value = this.get_global_rotation();
            Ok(Traitor::new(
                fyrox_lite_math::lite_math::LiteQuaternion::from(value),
            ))
        });

        fields.add_field_method_get("tag", |lua, this| {
            let value = this.get_tag();
            Ok(value)
        });

        fields.add_field_method_set(
            "local_position",
            |lua, this, value: TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>| {
                this.set_local_position(value.borrow()?.inner().clone().into());
                Ok(())
            },
        );

        fields.add_field_method_set("local_rotation", |lua, this, value: TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteQuaternion>>| {
                    this.set_local_rotation(value.borrow()?.inner().clone().into());
                    Ok(())
                });

        fields.add_field_method_set("tag", |lua, this, value: mlua::String| {
            this.set_tag(value.to_str()?.to_string());
            Ok(())
        });
    }

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}
}
