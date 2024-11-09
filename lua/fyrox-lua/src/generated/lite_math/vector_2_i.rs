
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

impl FyroxUserData for fyrox_lite_math::lite_math::LiteVector2I {
    const CLASS_NAME: &'static str = "Vector2I";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        methods.add_method_mut("mul", |lua, this, (o): (i32)| {
            let o = o;
            let ret = this.mul(o);
            let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2I::from(ret));
            Ok(ret)
        });
        methods.add_method_mut(
            "add",
            |lua, this, (o): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2I>>)| {
                let o = o.borrow()?.inner().clone().into();
                let ret = this.add(o);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2I::from(ret));
                Ok(ret)
            },
        );
        methods.add_method_mut(
            "sub",
            |lua, this, (o): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2I>>)| {
                let o = o.borrow()?.inner().clone().into();
                let ret = this.sub(o);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2I::from(ret));
                Ok(ret)
            },
        );
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("new", |lua, this, (x, y): (i32, i32)| {
            let x = x;
            let y = y;
            let ret = fyrox_lite_math::lite_math::LiteVector2I::new(x, y);
            let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2I::from(ret));
            Ok(ret)
        });
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| {
            let value = this.get_x();
            Ok(value)
        });
        fields.add_field_method_get("y", |lua, this| {
            let value = this.get_y();
            Ok(value)
        });
        fields.add_field_method_set("x", |lua, this, value: i32| {
            this.set_x(value);
            Ok(())
        });
        fields.add_field_method_set("y", |lua, this, value: i32| {
            this.set_y(value);
            Ok(())
        });
    }
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("ZERO", |lua, this| {
            let value = fyrox_lite_math::lite_math::LiteVector2I::get_ZERO();
            Ok(Traitor::new(
                fyrox_lite_math::lite_math::LiteVector2I::from(value),
            ))
        });
    }
}
