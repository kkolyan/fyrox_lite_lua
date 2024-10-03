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
    lite_physics::{LiteIntersection, LitePhysics, LiteRayCastOptions, LiteRigidBody},
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
    AnyUserData, IntoLua, Lua, MetaMethod, MultiValue, Table, UserDataFields, UserDataMethods, UserDataRef, UserDataRefMut, Value
};
use send_wrapper::SendWrapper;

type UserScriptImpl<'a> = TypedUserData<'a, ScriptObject>;

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