#![allow(unused_variables)]

use std::{borrow::Borrow, mem};

use crate::{
    debug::VerboseLuaValue,
    fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
    fyrox_utils::PluginsRefMut_Ext,
    lua_error,
    script::LuaScript,
    script_class::ScriptClass,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
};
use fyrox::core::{
        algebra::{UnitQuaternion, Vector3},
        log::Log,
    };
use fyrox_lite::{
    lite_node::{LiteNode, LiteRoutingStrategy},
    lite_physics::{LitePhysics, LiteRayCastOptions, LiteRigidBody},
    lite_prefab::LitePrefab,
    lite_scene::LiteScene,
    lite_ui::{Brush, Color, LiteText, LiteUiNode},
    lite_window::{LiteCursorGrabMode, LiteWindow},
    script_context::with_script_context,
    spi::UserScript,
    LiteDataType,
};
use fyrox_lite_math::{quat::LiteQuaternion, vec::LiteVector3};
use mlua::{
    AnyUserData, Lua, MetaMethod, MultiValue, Table, UserDataFields, UserDataMethods, UserDataRef,
    UserDataRefMut, Value,
};
use send_wrapper::SendWrapper;

type UserScriptImpl<'a> = TypedUserData<'a, ScriptObject>;


impl FyroxUserData for LiteVector3 {
    const CLASS_NAME: &'static str = "Vector3";

    #[rustfmt::skip]
    fn add_instance_fields<'lua, F: UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| Ok(this.inner().clone().get_x()));
        fields.add_field_method_get("y", |lua, this| Ok(this.inner().clone().get_y()));
        fields.add_field_method_get("z", |lua, this| Ok(this.inner().clone().get_z()));

        fields.add_field_method_set("x", |lua, this, value: f32| { this.inner_mut().set_x(value); Ok(()) });
        fields.add_field_method_set("y", |lua, this, value: f32| { this.inner_mut().set_y(value); Ok(()) });
        fields.add_field_method_set("z", |lua, this, value: f32| { this.inner_mut().set_z(value); Ok(()) });
    }

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor::new(this.mul(o)))
            },
        );

        methods.add_function(
            "add",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor::new(this.add(*o.inner()))) },
        );
        methods.add_function(
            "sub",
            |lua,
             (this, o): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| { Ok(Traitor::new(this.sub(*o.inner()))) },
        );
        methods.add_function(
            "normalize",
            |lua, this: UserDataRef<Traitor<LiteVector3>>| Ok(Traitor::new(this.normalize())),
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

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method("new", |lua, cls, (x, y, z): (f32, f32, f32)| {
            Ok(Traitor::new(LiteVector3::new(x, y, z)))
        });
    }

    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_function_get("X", |lua, this| Ok(Traitor::new(LiteVector3::x_axis())));
        fields.add_field_function_get("Y", |lua, this| Ok(Traitor::new(LiteVector3::y_axis())));
        fields.add_field_function_get("Z", |lua, this| Ok(Traitor::new(LiteVector3::z_axis())));

        fields.add_field_function_get("ZERO", |lua, this| Ok(Traitor::new(LiteVector3::zero())));
    }
}

impl FyroxUserData for LiteQuaternion {
    const CLASS_NAME: &'static str = "Quaternion";

    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_function(
            "mul",
            |lua, (this, o): (UserDataRef<Traitor<LiteQuaternion>>, AnyUserData)| {
                if let Ok(o) = o.borrow::<Traitor<LiteVector3>>() {
                    return Ok(lua.create_userdata(Traitor::new(this.mul__LiteVector(*o.inner()))));
                }
                if let Ok(o) = o.borrow::<Traitor<LiteQuaternion>>() {
                    return Ok(
                        lua.create_userdata(Traitor::new(this.mul__LiteQuaternion(*o.inner())))
                    );
                }
                Err(mlua::Error::runtime("invalid operand type"))
            },
        );
    }

    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
        methods.add_method(
            "face_towards",
            |lua,
             cls,
             (dir, up): (
                UserDataRef<Traitor<LiteVector3>>,
                UserDataRef<Traitor<LiteVector3>>,
            )| {
                Ok(Traitor::new(LiteQuaternion::face_towards(
                    *dir.inner(),
                    *up.inner(),
                )))
            },
        );
        methods.add_method(
            "from_axis_angle",
            |lua, cls, (axis, angle): (UserDataRef<Traitor<LiteVector3>>, f32)| {
                Ok(Traitor::new(LiteQuaternion::from_axis_angle(
                    *axis.inner(),
                    angle,
                )))
            },
        );
    }
}

impl<'a> UserScript for TypedUserData<'a, ScriptObject> {
    type ProxyScript = LuaScript;

    type LangSpecificError = mlua::Error;
    
    type UserScriptMessage = Traitor<SendWrapper<Value<'static>>>;

    type UserScriptGenericStub = ();

    fn extract_from(proxy: &Self::ProxyScript, class_name: &str) -> Option<Self> {
        if proxy.name == class_name {
            let script_data = proxy.data.inner_unpacked();
            return Some(script_data.expect("expected to be unpacked here"));
        }
        None
    }

    fn into_proxy_script(self) -> mlua::Result<Self::ProxyScript> {
        let name = self.borrow()?.def.metadata.class.to_string();
        // it's sound, because Lua outlives a process
        let ud: TypedUserData<'static, ScriptObject> = unsafe { mem::transmute(self) };
        let data = crate::script_data::ScriptData::Unpacked(SendWrapper::new(ud));
        Ok(LuaScript { name, data })
    }
}

impl LiteDataType for Traitor<SendWrapper<Value<'static>>> {}

impl<'a> LiteDataType for TypedUserData<'a, ScriptObject> {}