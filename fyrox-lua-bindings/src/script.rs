use super::script_data::ScriptData;
use crate::fyrox_lite::Traitor;
use crate::fyrox_utils::PluginsRefMut_Ext;
use crate::lua_reflect_bindings::LuaTableKey;
use crate::plugin::LuaPlugin;
use crate::reflect_base;
use crate::script_class::ScriptClass;
use crate::script_def::ScriptKind;
use crate::script_object::ScriptObject;
use crate::typed_userdata::TypedUserData;
use fyrox::asset::Resource;
use fyrox::core::algebra::UnitQuaternion;
use fyrox::core::algebra::Vector3;
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::gui::UiNode;
use fyrox::resource::model::Model;
use fyrox::scene::node::Node;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use fyrox_lite_api::script_context::without_script_context;
use fyrox_lite_api::script_context::UnsafeAsUnifiedContext;
use mlua::prelude::LuaResult;
use mlua::AsChunk;
use mlua::FromLua;
use mlua::Function;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::Table;
use mlua::UserDataRef;
use mlua::UserDataRefMut;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::process::exit;

#[derive(Debug, Clone, ComponentProvider)]
pub struct LuaScript {
    pub name: String,
    pub data: ScriptData,
}

impl ScriptTrait for LuaScript {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
        invoke_callback(
            &mut self.data,
            ctx.plugins.lua().vm,
            ctx,
            "on_init",
            |_lua| Ok(()),
        );
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
        invoke_callback(
            &mut self.data,
            ctx.plugins.lua().vm,
            ctx,
            "on_start",
            |_lua| Ok(()),
        );
    }

    fn on_deinit(&mut self, ctx: &mut fyrox::script::ScriptDeinitContext) {
        uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
        invoke_callback(
            &mut self.data,
            ctx.plugins.lua().vm,
            ctx,
            "on_deinit",
            |_lua| Ok(()),
        );
    }

    fn on_os_event(&mut self, event: &fyrox::event::Event<()>, ctx: &mut ScriptContext) {
        uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
        invoke_callback(
            &mut self.data,
            ctx.plugins.lua().vm,
            ctx,
            "on_os_event",
            |_lua| Ok(Traitor::new(event.clone())),
        );
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
        invoke_callback(
            &mut self.data,
            ctx.plugins.lua().vm,
            ctx,
            "on_update",
            |_lua| Ok(()),
        );
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(lua_message) = message.downcast_ref::<SendWrapper<Value>>() {
            uppack_script_if_necessary(ctx.plugins.lua_mut(), &mut self.data);
            invoke_callback(
                &mut self.data,
                ctx.plugins.lua().vm,
                ctx,
                "on_init",
                |_lua| Ok(Value::clone(lua_message)),
            );
        } else {
            panic!("non-lua messages not supported by lua scripts")
        }
    }
}

fn uppack_script_if_necessary(plugin: &mut LuaPlugin, data: &mut ScriptData) {
    if plugin.failed {
        // don't spam logs, though, plugin is completely broken at this point
        return;
    }
    if data.is_packed() {
        // script was just loaded from the scene file or safe game. unpack it!
        *data = match data {
            ScriptData::Packed(it) => {
                let so = plugin
                    .vm
                    .create_userdata(it.clone())
                    .map(TypedUserData::<ScriptObject>::new)
                    .map(SendWrapper::new)
                    .map(ScriptData::Unpacked);
                match so {
                    Ok(it) => it,
                    Err(err) => {
                        Log::err(format!("failed to unpack LuaScript: {:?}", err));
                        plugin.failed = true;
                        return;
                    }
                }
            }
            ScriptData::Unpacked(_) => panic!("WTF?"),
        }
    }
}

pub fn invoke_callback<'a, 'b, 'c, 'lua, A: IntoLuaMulti<'lua>>(
    data: &mut ScriptData,
    lua: &'static Lua,
    ctx: &mut dyn UnsafeAsUnifiedContext<'a, 'b, 'c>,
    callback_name: &str,
    args: impl FnOnce(&'lua Lua) -> mlua::Result<A>,
) {
    without_script_context(ctx, || {
        let script_object_ud = data
            .inner_unpacked()
            .expect("WTF, it's guaranteed to be unpacked here");

        let class_name = script_object_ud.borrow().unwrap().def.metadata.class;
        // TODO optimize me
        let class = lua
            .globals()
            .get::<_, UserDataRef<ScriptClass>>(class_name)
            .unwrap_or_else(|err| panic!("class not found: {}. error: {}", class_name, err));

        let callback = class.table.get(callback_name);

        if let Some(Value::Function(callback)) = callback {
            let args = args(lua).unwrap();
            match callback.call::<_, ()>((script_object_ud, args)) {
                Ok(_) => {},
                Err(err) => {
                    Log::err(format!("callback \"{}:{}\" failed with Lua error:\n{}", class_name, callback_name, err));
                    Log::warn("exiting to prevent error spamming (change this behavior in future)");
                    exit(123);
                },
            };
        }
    });
}

impl BaseScript for LuaScript {
    fn clone_box(&self) -> Box<dyn ScriptTrait> {
        Box::new(self.clone())
    }
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn id(&self) -> Uuid {
        self.data
            .with_script_object(|it| match it.def.metadata.kind {
                ScriptKind::Script(uuid) => uuid,
                ScriptKind::Plugin => panic!("not expected to be called for Plugin scripts"),
            })
    }
}

impl Visit for LuaScript {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.data.visit(name, visitor)
    }
}

impl Reflect for LuaScript {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        self.data.with_script_object(|it| it.fields_info(func))
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        self.data.with_script_object(|it| it.fields(func))
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        self.data.with_script_object_mut(|it| it.fields_mut(func))
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        self.data.with_script_object(|it| it.field(name, func))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        self.data
            .with_script_object_mut(|it| it.field_mut(name, func))
    }
}

pub struct LuaScriptBasedExpr {}

#[derive(Clone)]
pub enum ScriptFieldValue {
    Number(f64),
    String(String),
    Bool(bool),
    Node(Handle<Node>),
    UiNode(Handle<UiNode>),
    Prefab(Option<Resource<Model>>),
    Vector3(Vector3<f32>),
    Quaternion(UnitQuaternion<f32>),
    RawLuaValue(SendWrapper<Value<'static>>),
}

impl ScriptFieldValue {
    pub fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        match self {
            ScriptFieldValue::Number(it) => it,
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Bool(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RawLuaValue(_it) => panic!("WTF, it shouldn't be reachable"),
        }
    }
    pub fn as_reflect(&self) -> &dyn Reflect {
        match self {
            ScriptFieldValue::Number(it) => it,
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Bool(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RawLuaValue(_it) => panic!("WTF, it shouldn't be reachable"),
        }
    }
}

impl Debug for ScriptFieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptFieldValue::Number(it) => it.fmt(f),
            ScriptFieldValue::String(it) => it.fmt(f),
            ScriptFieldValue::Bool(it) => it.fmt(f),
            ScriptFieldValue::Node(it) => it.fmt(f),
            ScriptFieldValue::UiNode(it) => it.fmt(f),
            ScriptFieldValue::Prefab(it) => it.fmt(f),
            ScriptFieldValue::Vector3(it) => it.fmt(f),
            ScriptFieldValue::Quaternion(it) => it.fmt(f),
            ScriptFieldValue::RawLuaValue(it) => it.fmt(f),
        }
    }
}
