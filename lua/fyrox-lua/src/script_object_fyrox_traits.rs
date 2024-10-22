use std::any::TypeId;

use fyrox::{asset::Resource, core::{algebra::{UnitQuaternion, Vector3}, pool::Handle, reflect::{FieldInfo, Reflect}}, gui::UiNode, resource::model::Model, scene::node::Node};

use crate::{reflect_base, script_metadata::ScriptFieldValueType, script_object::{ScriptFieldValue, ScriptObject}};


impl Reflect for ScriptObject {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let def = self.def.clone();
        let fields = def
            .metadata
            .fields
            .iter()
            .filter(|it| it.ty != ScriptFieldValueType::RuntimePin)
            .enumerate()
            .filter(|(_i, it)| !it.private)
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<ScriptObject>(),
                name: it.name.as_str(),
                display_name: it.title.as_str(),
                description: it.name.as_str(),
                type_name: match it.ty {
                    ScriptFieldValueType::Number => std::any::type_name::<f32>(),
                    ScriptFieldValueType::String => std::any::type_name::<mlua::String>(),
                    ScriptFieldValueType::Bool => std::any::type_name::<bool>(),
                    ScriptFieldValueType::Node => std::any::type_name::<Handle<Node>>(),
                    ScriptFieldValueType::UiNode => std::any::type_name::<Handle<UiNode>>(),
                    ScriptFieldValueType::Prefab => {
                        std::any::type_name::<Option<Resource<Model>>>()
                    }
                    ScriptFieldValueType::Vector3 => std::any::type_name::<Vector3<f32>>(),
                    ScriptFieldValueType::Quaternion => {
                        std::any::type_name::<UnitQuaternion<f32>>()
                    }
                    ScriptFieldValueType::RuntimePin => panic!("WTF, it's excluded above"),
                },
                doc: it.description.unwrap_or(""),
                value: match self.values.get(i).unwrap() {
                    ScriptFieldValue::Number(it) => it,
                    ScriptFieldValue::String(it) => it,
                    ScriptFieldValue::Bool(it) => it,
                    ScriptFieldValue::Node(it) => it,
                    ScriptFieldValue::UiNode(it) => it,
                    ScriptFieldValue::Prefab(it) => it,
                    ScriptFieldValue::Vector3(it) => it,
                    ScriptFieldValue::Quaternion(it) => it,
                    ScriptFieldValue::RuntimePin(_it) => panic!("WTF, it's excluded above"),
                },
                reflect_value: self.values.get(i).unwrap().as_reflect(),
                read_only: false,
                immutable_collection: false,
                min_value: None,
                max_value: None,
                step: None,
                precision: None,
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        let fields = self
            .values
            .iter()
            .enumerate()
            .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
            .map(|(_i, it)| {
                let it: &dyn Reflect = it.as_reflect();
                it
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        let mut fields = self
            .values
            .iter_mut()
            .enumerate()
            .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
            .map(|(_i, it)| {
                let it: &mut dyn Reflect = it.as_reflect_mut();
                it
            })
            .collect::<Vec<_>>();
        func(&mut fields)
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &dyn Reflect = it.as_reflect();
            x
        }))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get_mut(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &mut dyn Reflect = it.as_reflect_mut();
            x
        }))
    }
}