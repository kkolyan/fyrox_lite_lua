use std::{any::TypeId, fmt::Debug, mem, ops::Deref, sync::Arc};

use fyrox::{
    asset::Resource,
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
        reflect::{FieldInfo, Reflect},
    },
    gui::UiNode,
    resource::model::Model,
    scene::node::Node,
};
use fyrox_lite::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_node::LiteNode,
    lite_prefab::LitePrefab,
    lite_ui::LiteUiNode,
    script_context::with_script_context,
};
use mlua::{MetaMethod, String, UserData, UserDataRef, Value};
use send_wrapper::SendWrapper;

use crate::{
    debug::VerboseLuaValue,
    fyrox_lite_class::{FyroxUserData, Traitor},
    lua_error,
    lua_utils::{OptionX, ValueX},
    reflect_base,
    script_class::ScriptClass,
};

use super::{
    script::ScriptFieldValue,
    script_def::{ScriptDefinition, ScriptFieldValueType},
};

/// For MLua API integration
#[derive(Clone)]
pub struct ScriptObject {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue>,
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
                    ScriptFieldValueType::RawLuaValue => ScriptFieldValue::RawLuaValue(None),
                })
                .collect(),
        }
    }
}

impl UserData for ScriptObject {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("node", |_lua, _this| {
            with_script_context(|ctx| Ok(ctx.handle.map(|it| Traitor::new(LiteNode::new(it)))))
        });
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString.name(), |_lua, this, _args: ()| {
            Ok(format!("{:?}", this))
        });
        methods.add_meta_function(
            MetaMethod::Index.name(),
            |lua, (this, key): (Value, String)| {
                // working with class
                if let Value::Table(this) = this {
                    return this.raw_get(key);
                }
                if let Value::UserData(this) = this {
                    if let Ok(this) = this.borrow::<ScriptObject>() {
                        let field_name = key.to_string_lossy();
                        let field_index = this
                            .def
                            .metadata
                            .field_name_to_index
                            .get(field_name.as_ref());
                        if let Some(field_index) = field_index {
                            let value = &this.values[*field_index];
                            let result = match value {
                                ScriptFieldValue::Number(it) => Value::Number(*it),
                                ScriptFieldValue::String(it) => {
                                    Value::String(lua.create_string(it)?)
                                }
                                ScriptFieldValue::Bool(it) => Value::Boolean(*it),
                                ScriptFieldValue::Node(it) => Value::UserData(
                                    lua.create_userdata(Traitor::new(LiteNode::new(*it)))?,
                                ),
                                ScriptFieldValue::UiNode(it) => Value::UserData(
                                    lua.create_userdata(Traitor::new(LiteUiNode::new(*it)))?,
                                ),
                                ScriptFieldValue::Prefab(it) => {
                                    Value::UserData(lua.create_userdata(Traitor::new(
                                        LitePrefab::new(it.clone().unwrap()),
                                    ))?)
                                }
                                ScriptFieldValue::Vector3(it) => Value::UserData(
                                    lua.create_userdata(Traitor::new(LiteVector3::from(*it)))?,
                                ),
                                ScriptFieldValue::Quaternion(it) => Value::UserData(
                                    lua.create_userdata(Traitor::new(LiteQuaternion::from(*it)))?,
                                ),
                                ScriptFieldValue::RawLuaValue(it) => it
                                    .as_ref()
                                    .map(|it| Value::clone(it.deref()))
                                    .unwrap_or(Value::Nil),
                            };
                            return Ok(result);
                        }

                        let class = lua
                            .globals()
                            .get::<_, Option<UserDataRef<ScriptClass>>>(this.def.metadata.class)?;
                        if let Some(class) = class {
                            let value = class.table.get(field_name.as_ref());
                            if let Some(value) = value {
                                if !value.is_nil() {
                                    return Ok(value.clone());
                                }
                            }
                        }
                        return Err(lua_error!(
                            "cannot index {} with \"{}\": no such method or field",
                            this.def.metadata.class,
                            key.to_string_lossy()
                        ));
                    }
                }
                Err(lua_error!("unexpected type"))
            },
        );
        methods.add_meta_function_mut(
            MetaMethod::NewIndex.name(),
            |_lua, (this, key, value): (Value, String, Value)| {
                // working with class
                if let Value::Table(this) = this {
                    return this.raw_set(key, value);
                }
                if let Value::UserData(this) = this {
                    // working with script instances
                    if let Ok(mut this) = this.borrow_mut::<ScriptObject>() {
                        let field_name = key.to_string_lossy();
                        let class = this.def.metadata.class;
                        let field_index = *this
                            .def
                            .metadata
                            .field_name_to_index
                            .get(field_name.as_ref())
                            .lua_ok_or_else(|| {
                                lua_error!("unknown field {}.`{}`", class, field_name)
                            })?;
                        let value_storage = &mut this.values[field_index];
                        match value_storage {
                            ScriptFieldValue::Number(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })?;
                            }
                            ScriptFieldValue::String(it) => {
                                *it = value
                                    .as_string_lossy()
                                    .map(|it| it.to_string())
                                    .ok_or_else(|| {
                                        lua_error!(
                                            "cannot assign {}.{} with {:?}",
                                            class,
                                            field_name,
                                            VerboseLuaValue::new(value)
                                        )
                                    })?;
                            }
                            ScriptFieldValue::Bool(it) => {
                                *it = value.as_boolean().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })?
                            }
                            ScriptFieldValue::Node(it) => {
                                *it = match value {
                                    Value::Nil => Default::default(),
                                    _ => extract_userdata_value::<LiteNode>(
                                        value,
                                        class,
                                        &field_name,
                                    )?
                                    .inner(),
                                }
                            }
                            ScriptFieldValue::UiNode(it) => {
                                *it = match value {
                                    Value::Nil => Default::default(),
                                    _ => extract_userdata_value::<LiteUiNode>(
                                        value,
                                        class,
                                        &field_name,
                                    )?
                                    .inner(),
                                }
                            }
                            ScriptFieldValue::Prefab(it) => {
                                *it = match value {
                                    Value::Nil => Default::default(),
                                    _ => Some(
                                        extract_userdata_value::<LitePrefab>(
                                            value,
                                            class,
                                            &field_name,
                                        )?
                                        .inner(),
                                    ),
                                }
                            }
                            ScriptFieldValue::Vector3(it) => {
                                *it = extract_userdata_value::<LiteVector3>(
                                    value,
                                    class,
                                    &field_name,
                                )?
                                .into()
                            }
                            ScriptFieldValue::Quaternion(it) => {
                                *it = extract_userdata_value::<LiteQuaternion>(
                                    value,
                                    class,
                                    &field_name,
                                )?
                                .into()
                            }
                            ScriptFieldValue::RawLuaValue(it) => {
                                // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
                                let value: Value<'static> = unsafe { mem::transmute(value) };
                                *it = Some(SendWrapper::new(value))
                            }
                        };
                        return Ok(());
                    }
                }
                Err(lua_error!("unexpected type"))
            },
        );
    }
}

fn extract_userdata_value<T: FyroxUserData + 'static + Clone>(
    value: Value,
    class: &str,
    field: &str,
) -> mlua::Result<T> {
    if let Value::UserData(value) = value {
        match value.borrow::<Traitor<T>>() {
            Ok(it) => return Ok(it.inner().clone()),
            Err(err) => match err {
                mlua::Error::UserDataBorrowError => panic!(
                    "immutable borrow failed while getting {}.`{}`",
                    class, field
                ),
                err => return Err(err),
            },
        }
    }
    Err(lua_error!(
        "cannot assign {}.`{}` with {:?}",
        class,
        field,
        value
    ))
}

impl Reflect for ScriptObject {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        let def = self.def.clone();
        let fields = def
            .metadata
            .fields
            .iter()
            .filter(|it| it.ty != ScriptFieldValueType::RawLuaValue)
            .enumerate()
            .filter(|(_i, it)| !it.private)
            .map(|(i, it)| FieldInfo {
                owner_type_id: TypeId::of::<ScriptObject>(),
                name: it.name.as_str(),
                display_name: it.title.as_str(),
                description: it.name.as_str(),
                type_name: match it.ty {
                    ScriptFieldValueType::Number => std::any::type_name::<f32>(),
                    ScriptFieldValueType::String => std::any::type_name::<String>(),
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
                    ScriptFieldValueType::RawLuaValue => panic!("WTF, it's excluded above"),
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
                    ScriptFieldValue::RawLuaValue(_it) => panic!("WTF, it's excluded above"),
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