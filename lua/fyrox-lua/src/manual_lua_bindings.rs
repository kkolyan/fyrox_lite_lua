use fyrox::core::{log::Log, Uuid};
use fyrox_lite::{lite_node::LiteNode, lite_prefab::LitePrefab, lite_ui::LiteUiNode, script_context::with_script_context};
use fyrox_lite_math::lite_math::{LiteQuaternion, LiteVector3};
use mlua::{MetaMethod, String as LuaString, Table, UserData, UserDataRef, Value};

use crate::{debug::VerboseLuaValue, lua_error, lua_utils::{OptionX, ValueX}, script_class::ScriptClass, script_object::{ScriptFieldValue, ScriptObject}, user_data_plus::{FyroxUserData, Traitor}};



impl UserData for ScriptClass {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(
            MetaMethod::NewIndex.name(),
            |_lua, this, (k, v): (mlua::String, mlua::Value)| {
                let static_v: mlua::Value<'static> = unsafe {
                    // we use Lua the whole life of the program, so that seems sound
                    std::mem::transmute(v)
                };
                Log::info(format!("register user-defined method: {}.{}", this.name, k.to_string_lossy()));
                this.table.insert(k.to_string_lossy().to_string(), static_v);
                Ok(())
            },
        );
        methods.add_meta_method(
            MetaMethod::Index.name(),
            |_lua, this, (k, _v): (mlua::String, mlua::Value)| {
				let value = this.table.get(&k.to_string_lossy().to_string()).unwrap_or_else(|| &mlua::Value::Nil);
				Ok(value.clone())
            },
        );
    }
}


impl UserData for Traitor<ScriptObject> {
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
            |lua, (this, key): (Value, LuaString)| {
                // working with class
                if let Value::Table(this) = &this {
                    return this.raw_get(key);
                }
                if let Value::UserData(this) = &this {
                    if let Ok(this) = this.borrow::<Traitor<ScriptObject>>() {
                        let field_name = key.to_string_lossy();
                        let field_index = this
                            .def
                            .metadata
                            .field_name_to_index
                            .get(field_name.as_ref());
                        if let Some(field_index) = field_index {
                            let value = &this.values[*field_index];
                            let result = match value {
                                ScriptFieldValue::f32(it) => Value::Number(*it as f64),
                                ScriptFieldValue::f64(it) => Value::Number(*it),
                                ScriptFieldValue::i16(it) => Value::Number(*it as f64),
                                ScriptFieldValue::i32(it) => Value::Number(*it as f64),
                                ScriptFieldValue::i64(it) => Value::Number(*it as f64),
                                ScriptFieldValue::String(it) => {
                                    Value::String(lua.create_string(it)?)
                                }
                                ScriptFieldValue::bool(it) => Value::Boolean(*it),
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
                                ScriptFieldValue::RuntimePin(it) => match it {
                                    Some(it) => lua
                                        .globals()
                                        .get::<_, Table>("PINS")
                                        .unwrap()
                                        .get::<_, mlua::Value>(it.as_str())?,
                                    None => Value::Nil,
                                },
                            };
                            return Ok(result);
                        }

                        let class = lua
                            .globals()
                            .get::<_, Option<UserDataRef<ScriptClass>>>(this.def.metadata.class.as_str())?;
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
            |lua, (this, key, value): (Value, LuaString, Value)| {
                // working with class
                if let Value::Table(this) = &this {
                    return this.raw_set(key, value);
                }
                if let Value::UserData(this) = &this {
                    // working with script instances
                    if let Ok(mut this) = this.borrow_mut::<Traitor<ScriptObject>>() {
                        let field_name = key.to_string_lossy();
                        let class = this.def.metadata.class.clone();
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
                            ScriptFieldValue::f32(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })? as f32;
                            }
                            ScriptFieldValue::f64(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })?;
                            }
                            ScriptFieldValue::i16(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })? as i16;
                            }
                            ScriptFieldValue::i32(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })? as i32;
                            }
                            ScriptFieldValue::i64(it) => {
                                *it = value.as_f64_tolerant().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        class,
                                        field_name,
                                        VerboseLuaValue::new(value)
                                    )
                                })? as i64;
                            }
                            ScriptFieldValue::String(it) => {
                                *it = value
                                    .as_string_lossy()
                                    .map(|it| it.to_string())
                                    .ok_or_else(|| {
                                        lua_error!(
                                            "cannot assign {}.{} with {:?}",
                                            &class,
                                            field_name,
                                            VerboseLuaValue::new(value)
                                        )
                                    })?;
                            }
                            ScriptFieldValue::bool(it) => {
                                *it = value.as_boolean().ok_or_else(|| {
                                    lua_error!(
                                        "cannot assign {}.{} with {:?}",
                                        &class,
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
                                        &class,
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
                                        &class,
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
                                            &class,
                                            &field_name,
                                        )?
                                        .inner(),
                                    ),
                                }
                            }
                            ScriptFieldValue::Vector3(it) => {
                                *it = extract_userdata_value::<LiteVector3>(
                                    value,
                                    &class,
                                    &field_name,
                                )?
                                .into()
                            }
                            ScriptFieldValue::Quaternion(it) => {
                                *it = extract_userdata_value::<LiteQuaternion>(
                                    value,
                                    &class,
                                    &field_name,
                                )?
                                .into()
                            }
                            ScriptFieldValue::RuntimePin(it) => {
                                let key = Uuid::new_v4().to_string();
                                lua.globals()
                                    .get::<_, Table>("PINS")
                                    .unwrap()
                                    .set(key.as_str(), value)?;
                                let prev = std::mem::replace(it, Some(key));
                                if let Some(prev) = prev {
                                    lua.globals()
                                        .get::<_, Table>("PINS")
                                        .unwrap()
                                        .set(prev.as_str(), Value::Nil)?;
                                }
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
