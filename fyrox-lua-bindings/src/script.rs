use crate::fyrox_utils::PluginsRefMut_Ext;
use crate::lua_reflect_bindings::LuaTableKey;
use crate::reflect_base;
use super::script_data::ScriptData;
use fyrox::core::algebra::UnitQuaternion;
use fyrox::core::algebra::Vector3;
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::scene::node::Node;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use fyrox_lite_api::script_context::without_script_context;
use mlua::prelude::LuaResult;
use mlua::FromLua;
use mlua::UserDataRefMut;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Debug, Clone, ComponentProvider)]
pub struct LuaScript {
    pub name: String,
    pub data: ScriptData,
}

impl ScriptTrait for LuaScript {
    fn on_update(&mut self, #[allow(unused_variables)] sc: &mut ScriptContext) {
        let plugin = sc.plugins.lua_mut();
        if plugin.failed {
            // don't spam logs, though, plugin is completely broken at this point
            return;
        }
        if self.data.is_packed() {
            // script was just loaded from the scene file or safe game. unpack it!
            self.data = match &self.data {
                ScriptData::Packed(it) => {
                    let so = plugin
                        .vm
                        .create_userdata(it.clone())
                        .and_then(|it| UserDataRefMut::from_lua(Value::UserData(it), plugin.vm))
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
        let lua = plugin.vm;
        without_script_context(sc, || {
        });
    }
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
        self.data.as_script_object().def.metadata.uuid
    }
}

impl Visit for LuaScript {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        let def = self.data.as_script_object().def.clone();
        for (i, field) in def.metadata.fields.iter().enumerate() {
            match &mut self.data.as_script_object_mut().values[i] {
                ScriptFieldValue::Number(it) => it.visit(&field.name, &mut guard),
                ScriptFieldValue::String(it) => it.visit(&field.name, &mut guard),
                ScriptFieldValue::Bool(it) => it.visit(&field.name, &mut guard),
                ScriptFieldValue::Node(it) => it.visit(&field.name, &mut guard),
                ScriptFieldValue::Vector3(it) => it.visit(&field.name, &mut guard),
                ScriptFieldValue::Quaternion(it) => it.visit(&field.name, &mut guard),
            }?;
        }

        Ok(())
    }
}

impl Reflect for LuaScript {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        self.data.as_script_object().fields_info(func)
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        self.data.as_script_object().fields(func)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        self.data.as_script_object_mut().fields_mut(func)
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        self.data.as_script_object().field(name, func)
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        self.data.as_script_object_mut().field_mut(name, func)
    }
}

pub struct LuaScriptBasedExpr {}

#[derive(Clone)]
pub enum ScriptFieldValue {
    Number(f64),
    String(String),
    Bool(bool),
    Node(Handle<Node>),
    Vector3(Vector3<f32>),
    Quaternion(UnitQuaternion<f32>),
}

impl ScriptFieldValue {
    pub fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        match self {
            ScriptFieldValue::Number(it) => it,
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Bool(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
        }
    }
    pub fn as_reflect(&self) -> &dyn Reflect {
        match self {
            ScriptFieldValue::Number(it) => it,
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Bool(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
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
            ScriptFieldValue::Vector3(it) => it.fmt(f),
            ScriptFieldValue::Quaternion(it) => it.fmt(f),
        }
    }
}