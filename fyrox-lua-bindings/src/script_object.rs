use std::{any::TypeId, ops::Deref, sync::Arc};

use fyrox::core::reflect::{FieldInfo, FieldValue, Reflect};
use fyrox_lite_api::{lite_math::{LiteQuaternion, LiteVector3}, lite_node::LiteNode};
use mlua::{MetaMethod, String, Table, UserData, UserDataRef, UserDataRefMut, Value};

use crate::{fyrox_lite::Traitor, lua_error, lua_utils::OptionX, reflect_base};

use super::{
    script::{ ScriptFieldValue},
    script_def::{ScriptDefinition, ScriptFieldValueType},
};

#[derive(Debug, Clone)]
pub struct ScriptObject {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue>,
}

impl ScriptObject {
    pub(crate) fn new(def: &Arc<ScriptDefinition>) -> Self {
        ScriptObject {
            def: def.clone(),
            values: def
                .metadata
                .fields
                .iter()
                .map(|it| match it.ty {
                    ScriptFieldValueType::Number => ScriptFieldValue::Number(Default::default()),
                    ScriptFieldValueType::String => ScriptFieldValue::String(Default::default()),
                    ScriptFieldValueType::Bool => ScriptFieldValue::Bool(Default::default()),
                    ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Vector3 => ScriptFieldValue::Vector3(Default::default()),
                })
                .collect(),
        }
    }
}

impl UserData for ScriptObject {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(MetaMethod::Index.name(), |lua, (this, key): (UserDataRef<ScriptObject>, String)| {
            let field_name = key.to_string_lossy();
            let field_index = this.def.metadata.field_name_to_index.get(field_name.as_ref())
                .lua_ok_or_else(|| lua_error!("unknown field {:?}.{:?}", this.def.metadata.class, key.to_str()))?;
            let value = &this.values[*field_index];
            let result = match value {
                ScriptFieldValue::Number(it) => Value::Number(*it),
                ScriptFieldValue::String(it) => Value::String(lua.create_string(it)?),
                ScriptFieldValue::Bool(it) => Value::Boolean(*it),
                ScriptFieldValue::Node(it) => Value::UserData(lua.create_any_userdata(Traitor(LiteNode::from(*it)))?),
                ScriptFieldValue::Vector3(it) => Value::UserData(lua.create_userdata(Traitor(LiteVector3::from(*it)))?),
                ScriptFieldValue::Quaternion(it) => Value::UserData(lua.create_userdata(Traitor(LiteQuaternion::from(*it)))?),
            };
            Ok(result)
        });
        methods.add_meta_function_mut(MetaMethod::NewIndex.name(), |lua, (mut this, key, value): (UserDataRefMut<ScriptObject>, String, Value)| {
            let field_name = key.to_string_lossy();
            let class = this.def.metadata.class;
            let field_index = *this.def.metadata.field_name_to_index.get(field_name.as_ref())
                .lua_ok_or_else(|| lua_error!("unknown field {}.`{}`", class, field_name))?;
            let value_storage = &mut this.values[field_index];
            match value_storage {
                ScriptFieldValue::Number(it) => *it = value.as_f64().unwrap(),
                ScriptFieldValue::String(it) => *it = value.as_string_lossy().unwrap().to_string(),
                ScriptFieldValue::Bool(it) => *it = value.as_boolean().unwrap(),
                ScriptFieldValue::Node(it) => *it = match value {
                    Value::Nil => Default::default(),
                    _ => extract_userdata_value::<Traitor<LiteNode>>(value, class, &field_name)?.inner().clone().into(),
                },
                ScriptFieldValue::Vector3(it) => *it = extract_userdata_value::<Traitor<LiteVector3>>(value, class, &field_name)?.inner().clone().into(),
                ScriptFieldValue::Quaternion(it) => *it = extract_userdata_value::<Traitor<LiteQuaternion>>(value, class, &field_name)?.inner().clone().into(),
            };
            Ok(())
        });
    }
}

fn extract_userdata_value<T: UserData +'static + Clone>(value: Value, class: &str, field: &str) -> mlua::Result<T> {
    if let Value::UserData(value) = value {
        match value.borrow::<T>() {
            Ok(it) => return Ok(it.clone()),
            Err(err) => match err {
                mlua::Error::UserDataBorrowError => panic!("immutable borrow failed while getting {}.`{}`", class, field),
                err => return Err(err),
            },
        }
    }
    Err(lua_error!("cannot assign {}.`{}` with {:?}", class, field, value))
}

impl Reflect for ScriptObject {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let def = self.def.clone();
        let fields = def
            .metadata
            .fields
            .iter()
            .enumerate()
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<ScriptObject>(),
                name: it.name.as_str(),
                display_name: it.name.as_str(),
                description: it.name.as_str(),
                type_name: match it.ty {
                    ScriptFieldValueType::Number => "f32",
                    ScriptFieldValueType::String => "alloc::string::String",
                    ScriptFieldValueType::Bool => "bool",
                    ScriptFieldValueType::Node => "Node",
                    ScriptFieldValueType::Vector3 => "Vector3",
                },
                doc: it.description.unwrap_or(""),
                value: match self.values.get(i).unwrap() {
                    ScriptFieldValue::Number(it) => it,
                    ScriptFieldValue::String(it) => it,
                    ScriptFieldValue::Bool(it) => it,
                    ScriptFieldValue::Node(it) => it,
                    ScriptFieldValue::Vector3(it) => it,
                    ScriptFieldValue::Quaternion(it) => it,
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
            .map(|it| {
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
            .map(|it| {
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
