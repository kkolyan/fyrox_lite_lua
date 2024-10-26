
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

impl FyroxUserData for fyrox_lite::lite_prefab::LitePrefab {
    const CLASS_NAME: &'static str = "Prefab";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        methods.add_method_mut(
            "instantiate_at",
            |lua,
             this,
             (position, orientation): (
                TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>,
                TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteQuaternion>>,
            )| {
                let position = position.borrow()?.inner().clone().into();
                let orientation = orientation.borrow()?.inner().clone().into();
                let ret = this.instantiate_at(position, orientation);
                let ret = Traitor::new(fyrox_lite::lite_node::LiteNode::from(ret));
                Ok(ret)
            },
        );
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}
}
