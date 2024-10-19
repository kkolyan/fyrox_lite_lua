use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use fyrox::{
    asset::Resource,
    core::{
        algebra::{UnitQuaternion, Vector3}, pool::Handle, reflect::Reflect, visitor::Visit, Uuid
    },
    gui::UiNode,
    resource::model::Model,
    scene::node::Node,
};

use super::script_metadata::{ScriptDefinition, ScriptFieldValueType};

/// Useful for persisting script data, but for some languages could be used as a runtime type
#[derive(Clone)]
pub struct ScriptObject<T: Lang> {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue<T>>,
}

pub trait Lang: Debug + Clone + 'static {
    type String<'a>;
    type RuntimePin: Clone + Debug + Visit + Default;
    type UnpackedScriptObject: Visit;

    fn drop_runtime_pin(runtime_pin: &mut Self::RuntimePin);
    fn clone_runtime_pin(runtime_pin: &Self::RuntimePin) -> Self::RuntimePin;
    fn drop_script_object_to_prevent_delayed_destructor(script: &mut Self::UnpackedScriptObject) {}
    fn id_of(script: &Self::UnpackedScriptObject) -> Uuid;
}

pub enum ScriptFieldValue<T: Lang> {
    Number(f64),
    String(String),
    Bool(bool),
    Node(Handle<Node>),
    UiNode(Handle<UiNode>),
    Prefab(Option<Resource<Model>>),
    Vector3(Vector3<f32>),
    Quaternion(UnitQuaternion<f32>),
    // global key of the value. is useful for hot-reload only, because in persistent data it's always None
    RuntimePin(Option<T::RuntimePin>),
}

impl<T: Lang> Debug for ScriptObject<T> {
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

impl<T: Lang> Drop for ScriptObject<T> {
    fn drop(&mut self) {
        for it in self.values.iter_mut() {
            if let ScriptFieldValue::RuntimePin(it) = it {
                if let Some(key) = it {
                    T::drop_runtime_pin(key);
                }
            }
        }
    }
}

impl<T: Lang> ScriptObject<T> {
    pub fn new(def: &Arc<ScriptDefinition>) -> Self {
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

impl<T: Lang> Clone for ScriptFieldValue<T> {
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
                    let new = T::clone_runtime_pin(existing);
                    
                    let new = Some(new);
                    Self::RuntimePin(new)
                }
                None => Self::RuntimePin(None),
            },
        }
    }
}

impl<T: Lang> ScriptFieldValue<T> {
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

impl<T: Lang> Debug for ScriptFieldValue<T> {
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
