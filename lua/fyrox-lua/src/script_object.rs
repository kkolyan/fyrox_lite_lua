use std::{fmt::{Debug, Formatter}, sync::Arc};

use fyrox::{
    asset::Resource,
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
        reflect::Reflect,
        Uuid,
    },
    gui::UiNode,
    resource::model::Model,
    scene::node::Node,
};
use mlua::{Table, Value};

use crate::lua_lifecycle::lua_vm;

use super::script_metadata::{ScriptDefinition, ScriptFieldValueType};

/// This object is what passed to mlua to represent a script
#[derive(Clone)]
pub struct ScriptObject {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue>,
}

pub enum ScriptFieldValue {
    Number(f64),
    String(String),
    Bool(bool),
    Node(Handle<Node>),
    UiNode(Handle<UiNode>),
    Prefab(Option<Resource<Model>>),
    Vector3(Vector3<f32>),
    Quaternion(UnitQuaternion<f32>),
    // global key of the value. is useful for hot-reload only, because in persistent data it's always None
    // Option is necessary to avoid creation of SendWrapper outside of main
    RuntimePin(Option<String>),
}

impl Debug for ScriptObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_name = self.def.metadata.class;
        let mut fields = Vec::new();
        for (i, field) in self.def.metadata.fields.iter().enumerate() {
            fields.push(format!(
                "{} ({:?}): {:?}",
                field.name, field.ty, self.values[i]
            ));
        }
        write!(f, "{}{{ {} }}", class_name, fields.join(", "))
    }
}

impl Drop for ScriptObject {
    fn drop(&mut self) {
        for it in self.values.iter_mut() {
            if let ScriptFieldValue::RuntimePin(it) = it {
                if let Some(key) = it {
                    lua_vm()
                        .globals()
                        .get::<_, Table>("PINS")
                        .unwrap()
                        .set(key.as_str(), Value::Nil)
                        .unwrap();
                }
            }
        }
    }
}

impl ScriptObject {
    pub(crate) fn new(def: &Arc<ScriptDefinition>) -> Self {
        ScriptObject {
            def: def.clone(),
            values: def
                .metadata
                .fields
                .iter()
                .map(|it| match &it.ty {
                    ScriptFieldValueType::Number => ScriptFieldValue::Number(Default::default()),
                    ScriptFieldValueType::String => ScriptFieldValue::String(Default::default()),
                    ScriptFieldValueType::Bool => ScriptFieldValue::Bool(Default::default()),
                    ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Prefab => ScriptFieldValue::Prefab(Default::default()),
                    ScriptFieldValueType::UiNode => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Vector3 => ScriptFieldValue::Vector3(Default::default()),
                    ScriptFieldValueType::Quaternion => {
                        ScriptFieldValue::Quaternion(Default::default())
                    }
                    ScriptFieldValueType::RuntimePin => ScriptFieldValue::RuntimePin(None),
                })
                .collect(),
        }
    }
}



impl Clone for ScriptFieldValue {
    fn clone(&self) -> Self {
        match self {
            Self::Number(it) => Self::Number(it.clone()),
            Self::String(it) => Self::String(it.clone()),
            Self::Bool(it) => Self::Bool(it.clone()),
            Self::Node(it) => Self::Node(it.clone()),
            Self::UiNode(it) => Self::UiNode(it.clone()),
            Self::Prefab(it) => Self::Prefab(it.clone()),
            Self::Vector3(it) => Self::Vector3(it.clone()),
            Self::Quaternion(it) => Self::Quaternion(it.clone()),
            Self::RuntimePin(it) => match it {
                Some(existing) => {
                    let new = Uuid::new_v4().to_string();
                    let ex_value = lua_vm()
                        .globals()
                        .get::<_, Table>("PINS")
                        .unwrap()
                        .get::<_, mlua::Value>(existing.as_str())
                        .unwrap();
                    lua_vm()
                        .globals()
                        .get::<_, Table>("PINS")
                        .unwrap()
                        .set(new.as_str(), ex_value)
                        .unwrap();
                    Self::RuntimePin(Some(new))
                },
                None => Self::RuntimePin(None),
            },
        }
    }
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
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
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
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
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
            ScriptFieldValue::RuntimePin(it) => it.fmt(f),
        }
    }
}
