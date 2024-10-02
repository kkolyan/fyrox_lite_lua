
#![allow(unused_variables)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};

impl FyroxUserData for lite_physics::LitePhysics {
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
            |lua, this, (opts): (TypedUserData<Traitor<lite_physics::LiteRayCastOptions>>)| {
                let opts = opts.borrow()?.inner().clone().into();

                let ret = lite_physics::LitePhysics::cast_ray(opts);
                let ret = lua.create_table_from(
                    ret.into_iter()
                        .map(|it| Traitor::new(lite_physics::LiteIntersection::from(it)))
                        .enumerate(),
                );
                Ok(ret)
            },
        );
    }
}
