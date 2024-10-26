
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

impl FyroxUserData for fyrox_lite::lite_physics::LitePhysics {
    const CLASS_NAME: &'static str = "Physics";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut(
            "cast_ray",
            |lua, this, (opts): (Traitor<fyrox_lite::lite_physics::LiteRayCastOptions>)| {
                let opts = opts.inner().clone().into();
                let ret = fyrox_lite::lite_physics::LitePhysics::cast_ray(opts);
                let ret = lua.create_table_from(
                    ret.into_iter()
                        .map(|it| {
                            Traitor::new(fyrox_lite::lite_physics::LiteIntersection::from(it))
                        })
                        .enumerate()
                        .map(|(i, v)| (i + 1, v)),
                )?;
                Ok(ret)
            },
        );
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("EXCLUDE_FIXED", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::EXCLUDE_FIXED;
            Ok(value)
        });
        fields.add_field_method_get("EXCLUDE_KINEMATIC", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::EXCLUDE_KINEMATIC;
            Ok(value)
        });
        fields.add_field_method_get("EXCLUDE_DYNAMIC", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::EXCLUDE_DYNAMIC;
            Ok(value)
        });
        fields.add_field_method_get("EXCLUDE_SENSORS", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::EXCLUDE_SENSORS;
            Ok(value)
        });
        fields.add_field_method_get("EXCLUDE_SOLIDS", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::EXCLUDE_SOLIDS;
            Ok(value)
        });
        fields.add_field_method_get("ONLY_DYNAMIC", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::ONLY_DYNAMIC;
            Ok(value)
        });
        fields.add_field_method_get("ONLY_KINEMATIC", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::ONLY_KINEMATIC;
            Ok(value)
        });
        fields.add_field_method_get("ONLY_FIXED", |lua, this| {
            let value = fyrox_lite::lite_physics::LitePhysics::ONLY_FIXED;
            Ok(value)
        });
    }
}
