use crate::lua_error;
use crate::lua_utils::OptionX;
use fyrox::core::algebra::Vector3;
use fyrox::core::reflect::Reflect;
use fyrox::core::ImmutableString;
use fyrox::script::ScriptContext;
use mlua::FromLua;
use mlua::Lua;
use mlua::MetaMethod;
use mlua::Result as LuaResult;
use mlua::UserData;
use mlua::UserDataMethods;
use mlua::Value;
use std::path::PathBuf;


macro_rules! fyrox_and_lua_numbers {
    ($($type:ty),*) => {
        $(
            impl FyroxAndLua for $type {
                fn fyrox_to_lua<'a>(p: &mut dyn Reflect, _lua: &'a Lua) -> Option<mlua::Result<mlua::Value<'a>>> {
                    let mut result: Option<mlua::Result<mlua::Value<'a>>> = None;
                    p.downcast_ref::<Self>(&mut |p| {
                        result = p.map(|it| Ok(mlua::Value::Number(*it as mlua::Number)))
                    });
                    result
                }

                fn lua_to_fyrox(p: &mut dyn Reflect, value: &mlua::Value) -> Option<mlua::Result<()>> {
                    let mut result: Option<mlua::Result<()>> = None;
                    p.downcast_mut::<Self>(&mut |p| {
                        result = p.map(|it| {
                            let s = crate::lua_utils::ValueX::as_f64_tolerant(value).lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?;
                            *it = s as Self;
                            Ok(())
                        })
                    });
                    result
                }
            }
        )*
    };
}
fyrox_and_lua_numbers!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

fn get_terminal_node_of_path_mut(
    source: &mut dyn Reflect,
    path: &[LuaTableKey],
    result: &mut dyn FnMut(Result<&mut dyn Reflect, &str>),
) {
    if path.is_empty() {
        result(Ok(source));
    } else if path.len() > 1 {
        get_value_by_lua_key_mut(source, path.first().unwrap(), &mut |value| match value {
            Ok(value) => get_terminal_node_of_path_mut(value, &path[1..], result),
            Err(error) => result(Err(error)),
        });
    } else {
        get_value_by_lua_key_mut(source, path.first().unwrap(), result);
    }
}

fn get_value_by_lua_key_mut(
    source: &mut dyn Reflect,
    key: &LuaTableKey,
    result: &mut dyn FnMut(Result<&mut dyn Reflect, &str>),
) {
    match key {
        LuaTableKey::Number(i) => {
            source.as_array_mut(&mut |array| match array {
                None => {
                    result(Err("index used to access non-indexable object"));
                }
                Some(array) => {
                    if let Some(it) = array.reflect_index_mut(*i) {
                        result(Ok(it));
                    }
                }
            });
        }
        LuaTableKey::String(key) => {
            source.field_mut(&key, &mut |value| {
                if let Some(it) = value {
                    result(Ok(it));
                };
            });
        }
    }
}

trait FyroxAndLua {
    /// None if type not supported
    /// Some(Err) if type is supported, but something went wrong
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>>;

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>>;
}

impl FyroxAndLua for bool {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, _lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<bool>(&mut |p| result = p.map(|it| Ok(Value::Boolean(*it))));
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                *it = value
                    .as_boolean()
                    .lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?;
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for String {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| lua.create_string(it).map(Value::String))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                *it = value
                    .as_str()
                    .lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?
                    .to_string();
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for PathBuf {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| {
                it.to_str()
                    .map(|s| lua.create_string(s).map(Value::String))
                    .unwrap_or_else(|| Err(lua_error!("")))
            })
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                let s = value
                    .as_str()
                    .lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?
                    .to_string();
                *it = Self::from(s);
                Ok(())
            })
        });
        result
    }
}

impl FyroxAndLua for ImmutableString {
    fn fyrox_to_lua<'a>(p: &mut dyn Reflect, lua: &'a Lua) -> Option<LuaResult<Value<'a>>> {
        let mut result: Option<LuaResult<Value<'a>>> = None;
        p.downcast_ref::<Self>(&mut |p| {
            result = p.map(|it| lua.create_string(it.as_str()).map(Value::String))
        });
        result
    }

    fn lua_to_fyrox<'a>(p: &mut dyn Reflect, value: &Value<'a>) -> Option<LuaResult<()>> {
        let mut result: Option<LuaResult<()>> = None;
        p.downcast_mut::<Self>(&mut |p| {
            result = p.map(|it| {
                let s = value
                    .as_str()
                    .lua_ok_or_else(|| lua_error!("cannot assign String field with {:?}", value))?
                    .to_string();
                *it = Self::from(s);
                Ok(())
            })
        });
        result
    }
}

#[derive(Clone)]
pub enum LuaTableKey {
    Number(usize),
    // TODO isn't it too heavy?
    String(String),
}

fn format_path(path: &[LuaTableKey]) -> String {
    let mut s = String::new();
    for it in path {
        s.push_str(
            match it {
                LuaTableKey::Number(i) => format!("[{}]", i),
                LuaTableKey::String(k) => format!(".{}", k),
            }
            .as_str(),
        );
    }
    s
}

impl FromLua<'_> for LuaTableKey {
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::Integer(i) => Ok(LuaTableKey::Number(i as usize)),
            Value::Number(i) => Ok(LuaTableKey::Number(i as usize)),
            Value::String(key) => Ok(LuaTableKey::String(key.to_str()?.to_string())),
            invalid => Err(lua_error!("Invalid LuaTableKey: {:?}", invalid)),
        }
    }
}
