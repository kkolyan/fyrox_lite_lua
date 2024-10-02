use std::{any::type_name, marker::PhantomData};

use mlua::{AnyUserData, FromLua, IntoLua, UserDataRef};

use crate::lua_error;

#[derive(Clone)]
pub struct TypedUserData<'lua, T> {
    pd: PhantomData<T>,
    ud: AnyUserData<'lua>,
}

impl<'lua, T: 'static> TypedUserData<'lua, T> {
    pub fn new(ud: AnyUserData<'lua>) -> Self {
        Self {
            pd: Default::default(),
            ud,
        }
    }

    pub fn borrow(&self) -> Result<std::cell::Ref<'_, T>, mlua::Error> {
        self.ud.borrow()
    }

    pub fn borrow_mut(&self) -> Result<std::cell::RefMut<'_, T>, mlua::Error> {
        self.ud.borrow_mut()
    }
}

impl<'lua, T> IntoLua<'lua> for TypedUserData<'lua, T> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        self.ud.into_lua(lua)
    }
}

impl <'lua, T : 'static> FromLua<'lua> for TypedUserData<'lua, T> {
    fn from_lua(value: mlua::Value<'lua>, _lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(TypedUserData::new(ud)),
            value => Err(lua_error!("cannot cast {:?} to {}", value, type_name::<T>())),
        }
    }
}
