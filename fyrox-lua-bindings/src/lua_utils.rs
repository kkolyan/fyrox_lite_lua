use fyrox::core::log::Log;
use fyrox::core::num_traits::ToPrimitive;
use mlua::prelude::LuaError;
use mlua::FromLuaMulti;
use mlua::Function;
use mlua::IntoLua;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::MultiValue;
use mlua::Result as LuaResult;
use mlua::Table;
use mlua::UserData;
use mlua::Value;
use std::cell::Ref;
use std::fmt::Display;
use std::process::exit;

#[macro_export]
macro_rules! lua_error {
    ($($arg:tt)*) => {{
        mlua::Error::runtime(format!($($arg)*))
    }}
}

pub fn exit_on_error<T>(result: LuaResult<T>) -> T {
    match result {
        Ok(t) => t,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}

pub fn log_error<T>(action_description: impl Display, result: LuaResult<T>) {
    match result {
        Ok(_) => {}
        Err(err) => {
            Log::err(format!(
                "Lua action failed: {}: {}",
                action_description, err
            ));
            println!("{}", err);
        }
    }
}

pub fn eval_func<'lua, A, R>(lua: &'lua Lua, code: &str, args: A) -> R
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    let boot = lua.load(code).eval::<Function>().unwrap();
    exit_on_error(boot.call::<A, R>(args))
}

pub fn eval_script(lua: &Lua, code: &str) {
    exit_on_error(lua.load(code).eval::<()>())
}

pub fn set_package_loaded<'lua, V: IntoLuaMulti<'lua>>(lua: &'lua Lua, key: &str, value: V) {
    eval_func::<_, ()>(
        &lua,
        "return function(key, value)
                package.loaded[key] = value
              end",
        (key, value),
    );
}

pub trait TableX<'lua> {
    fn get_table<K: IntoLua<'lua>>(&self, key: K) -> LuaResult<Table>;
}

impl<'lua> TableX<'lua> for Table<'lua> {
    fn get_table<K: IntoLua<'lua>>(&self, key: K) -> LuaResult<Table> {
        self.get(key)
    }
}

pub trait OptionX<T> {
    fn lua_ok_or<S: Display>(self, s: S) -> LuaResult<T>;
    fn lua_ok_or_else<S: Display>(self, f: impl FnOnce() -> S) -> LuaResult<T>;
}

impl<T> OptionX<T> for Option<T> {
    fn lua_ok_or<S: Display>(self, s: S) -> LuaResult<T> {
        self.ok_or(LuaError::runtime(s))
    }
    fn lua_ok_or_else<S: Display>(self, f: impl FnOnce() -> S) -> LuaResult<T> {
        self.ok_or_else(|| LuaError::runtime(f()))
    }
}

pub trait ValueX {
    fn as_f32_tolerant(&self) -> Option<f32> {
        if let Some(value) = self.as_value().as_number() {
            return value.to_f32();
        }
        if let Some(value) = self.as_value().as_integer() {
            return value.to_f32();
        }
        None
    }
    fn as_f64_tolerant(&self) -> Option<f64> {
        if let Some(value) = self.as_value().as_number() {
            return value.to_f64();
        }
        if let Some(value) = self.as_value().as_integer() {
            return value.to_f64();
        }
        None
    }

    fn as_value(&self) -> &Value;
}

impl ValueX for Value<'_> {
    fn as_value(&self) -> &Value {
        return self;
    }
}

pub enum ArgDefault<T> {
    IsRequired,
    HasDefault(T),
}

pub trait MultiValueX {
    fn extract_arg<'a, T: 'a>(
        &'a self,
        index: usize,
        name: impl Display,
        extractor: impl FnOnce(Option<&'a Value>) -> LuaResult<T>,
    ) -> LuaResult<T>;

    fn not_supported(&self, index: usize, name: impl Display) -> LuaResult<()> {
        self.extract_arg(index, name, |v| match v {
            None => Ok(()),
            Some(_) => Err(lua_error!("not supported")),
        })
    }

    fn extract_f32(
        &self,
        index: usize,
        name: impl Display,
        default: ArgDefault<f32>,
    ) -> LuaResult<f32> {
        self.extract_arg(index, name, |v| match v {
            None => match default {
                ArgDefault::IsRequired => Err(lua_error!("is missing")),
                ArgDefault::HasDefault(default) => Ok(default),
            },
            Some(v) => v.as_f32_tolerant().lua_ok_or("is not a number"),
        })
    }

    fn extract_text(
        &self,
        index: usize,
        name: impl Display,
        default: ArgDefault<&str>,
    ) -> LuaResult<String> {
        self.extract_arg(index, name, |v| match v {
            None => match default {
                ArgDefault::IsRequired => Err(lua_error!("is missing")),
                ArgDefault::HasDefault(default) => Ok(default.to_string()),
            },
            Some(v) => Ok(v.as_str().lua_ok_or("is not a string")?.to_string()),
        })
    }

    fn extract_userdata<T: UserData + 'static>(
        &self,
        index: usize,
        name: impl Display,
    ) -> LuaResult<Ref<T>> {
        self.extract_arg(index, name, |v| {
            v.lua_ok_or("is missing")?
                .as_userdata()
                .lua_ok_or("is not userdata")?
                .borrow::<T>()
        })
    }
}

impl MultiValueX for MultiValue<'_> {
    fn extract_arg<'a, T: 'a>(
        &'a self,
        index: usize,
        name: impl Display,
        extractor: impl FnOnce(Option<&'a Value>) -> LuaResult<T>,
    ) -> LuaResult<T> {
        let value = self.get(index);

        extractor(value).map_err(|err| {
            lua_error!(
                "invalid argument '{}' (#{}): {}: {:?}",
                name,
                index,
                err.to_string(),
                value
            )
        })
    }
}
