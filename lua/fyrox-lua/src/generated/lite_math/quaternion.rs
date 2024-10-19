
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

impl FyroxUserData for fyrox_lite_math::lite_math::LiteQuaternion {
    const CLASS_NAME: &'static str = "Quaternion";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });

        methods.add_method_mut(
            "mul_vec",
            |lua, this, (o): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>)| {
                let o = o.borrow()?.inner().clone().into();

                let ret = this.mul_vec(o);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(ret));
                Ok(ret)
            },
        );

        methods.add_method_mut(
                    "mul_quat",
                    |lua, this, (rot_delta): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteQuaternion>>)| {
            

                        let rot_delta = rot_delta.borrow()?.inner().clone().into();
                

                        let ret = this.mul_quat(rot_delta);
                        let ret = Traitor::new(fyrox_lite_math::lite_math::LiteQuaternion::from(ret));
                        Ok(ret)
                    },
                );
    }

    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut(
            "face_towards",
            |lua,
             this,
             (dir, up): (
                TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>,
                TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>,
            )| {
                let dir = dir.borrow()?.inner().clone().into();

                let up = up.borrow()?.inner().clone().into();

                let ret = fyrox_lite_math::lite_math::LiteQuaternion::face_towards(dir, up);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteQuaternion::from(ret));
                Ok(ret)
            },
        );

        methods.add_method_mut(
            "from_axis_angle",
            |lua,
             this,
             (axis, angle): (
                TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>,
                f32,
            )| {
                let axis = axis.borrow()?.inner().clone().into();

                let angle = angle;

                let ret = fyrox_lite_math::lite_math::LiteQuaternion::from_axis_angle(axis, angle);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteQuaternion::from(ret));
                Ok(ret)
            },
        );
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}

    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}
}
