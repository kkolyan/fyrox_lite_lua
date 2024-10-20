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
    fn drop_script_object_to_prevent_delayed_destructor(_script: &mut Self::UnpackedScriptObject) {}
    fn id_of(script: &Self::UnpackedScriptObject) -> Uuid;
    fn unpack_script(script: &ScriptObject<Self>) -> Result<Self::UnpackedScriptObject, String>;
}

#[allow(non_camel_case_types)]
pub enum ScriptFieldValue<T: Lang> {
    bool(bool),
    f32(f32),
    f64(f64),
    i16(i16),
    i32(i32),
    i64(i64),
    String(String),
    Node(Handle<Node>),
    UiNode(Handle<UiNode>),
    Prefab(Option<Resource<Model>>),
    Vector3(Vector3<f32>),
    Quaternion(UnitQuaternion<f32>),
    // global key of the value. is useful for hot-reload only, because in persistent data it's always None
    RuntimePin(T::RuntimePin),
}

impl<T: Lang> Debug for ScriptObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_name = self.def.metadata.class.clone();
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
                T::drop_runtime_pin(it);
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
                    ScriptFieldValueType::String => ScriptFieldValue::String(Default::default()),
                    ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Prefab => ScriptFieldValue::Prefab(Default::default()),
                    ScriptFieldValueType::UiNode => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Vector3 => ScriptFieldValue::Vector3(Default::default()),
                    ScriptFieldValueType::Quaternion => {
                        ScriptFieldValue::Quaternion(Default::default())
                    }
                    ScriptFieldValueType::RuntimePin => ScriptFieldValue::RuntimePin(T::RuntimePin::default()),
                    ScriptFieldValueType::bool => ScriptFieldValue::bool(Default::default()),
                    ScriptFieldValueType::f32 => ScriptFieldValue::f32(Default::default()),
                    ScriptFieldValueType::f64 => ScriptFieldValue::f64(Default::default()),
                    ScriptFieldValueType::i16 => ScriptFieldValue::i16(Default::default()),
                    ScriptFieldValueType::i32 => ScriptFieldValue::i32(Default::default()),
                    ScriptFieldValueType::i64 => ScriptFieldValue::i64(Default::default()),
                })
                .collect(),
        }
    }
}

impl<T: Lang> Clone for ScriptFieldValue<T> {
    fn clone(&self) -> Self {
        match self {
            ScriptFieldValue::String(it) => Self::String(it.clone()),
            ScriptFieldValue::Node(it) => Self::Node(it.clone()),
            ScriptFieldValue::UiNode(it) => Self::UiNode(it.clone()),
            ScriptFieldValue::Prefab(it) => Self::Prefab(it.clone()),
            ScriptFieldValue::Vector3(it) => Self::Vector3(it.clone()),
            ScriptFieldValue::Quaternion(it) => Self::Quaternion(it.clone()),
            ScriptFieldValue::RuntimePin(it) => {
                let new = T::clone_runtime_pin(it);
                ScriptFieldValue::RuntimePin(new)
            },
            ScriptFieldValue::bool(it) => ScriptFieldValue::bool(*it),
            ScriptFieldValue::f32(it) => ScriptFieldValue::f32(*it),
            ScriptFieldValue::f64(it) => ScriptFieldValue::f64(*it),
            ScriptFieldValue::i16(it) => ScriptFieldValue::i16(*it),
            ScriptFieldValue::i32(it) => ScriptFieldValue::i32(*it),
            ScriptFieldValue::i64(it) => ScriptFieldValue::i64(*it),
        }
    }
}

impl<T: Lang> ScriptFieldValue<T> {
    pub fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        match self {
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
            ScriptFieldValue::bool(it) => it,
            ScriptFieldValue::f32(it) => it,
            ScriptFieldValue::f64(it) => it,
            ScriptFieldValue::i16(it) => it,
            ScriptFieldValue::i32(it) => it,
            ScriptFieldValue::i64(it) => it,
        }
    }
    pub fn as_reflect(&self) -> &dyn Reflect {
        match self {
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
            ScriptFieldValue::bool(it) => it,
            ScriptFieldValue::f32(it) => it,
            ScriptFieldValue::f64(it) => it,
            ScriptFieldValue::i16(it) => it,
            ScriptFieldValue::i32(it) => it,
            ScriptFieldValue::i64(it) => it,
        }
    }
}

impl<T: Lang> Debug for ScriptFieldValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptFieldValue::String(it) => it.fmt(f),
            ScriptFieldValue::Node(it) => it.fmt(f),
            ScriptFieldValue::UiNode(it) => it.fmt(f),
            ScriptFieldValue::Prefab(it) => it.fmt(f),
            ScriptFieldValue::Vector3(it) => it.fmt(f),
            ScriptFieldValue::Quaternion(it) => it.fmt(f),
            ScriptFieldValue::RuntimePin(it) => it.fmt(f),
            ScriptFieldValue::bool(it) => it.fmt(f),
            ScriptFieldValue::f32(it) => it.fmt(f),
            ScriptFieldValue::f64(it) => it.fmt(f),
            ScriptFieldValue::i16(it) => it.fmt(f),
            ScriptFieldValue::i32(it) => it.fmt(f),
            ScriptFieldValue::i64(it) => it.fmt(f),
        }
    }
}
