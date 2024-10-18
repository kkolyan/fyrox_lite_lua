use fyrox::core::log::Log;
use fyrox::core::num_traits::ToPrimitive;
use mlua::prelude::LuaError;
use mlua::FromLuaMulti;
use mlua::Function;
use mlua::IntoLuaMulti;
use mlua::Lua;
use mlua::Result as LuaResult;
use mlua::Value;
use std::fmt::Display;
use std::process::exit;

#[macro_export]
macro_rules! lua_error {
    ($($arg:tt)*) => {{
        // panic!($($arg)*);
        mlua::Error::runtime(format!($($arg)*))
    }}
}

#[allow(unused)]
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

#[allow(unused)]
pub fn eval_func<'lua, A, R>(lua: &'lua Lua, code: &str, args: A) -> R
where
    A: IntoLuaMulti<'lua>,
    R: FromLuaMulti<'lua>,
{
    let boot = lua.load(code).eval::<Function>().unwrap();
    exit_on_error(boot.call::<A, R>(args))
}

#[allow(unused)]
pub fn eval_script(lua: &Lua, code: &str) {
    exit_on_error(lua.load(code).eval::<()>())
}

pub trait OptionX<T> {
    #[allow(unused)]
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
    #[allow(unused)]
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
        self
    }
}
