use std::{borrow::Borrow, marker::PhantomData};

use mlua::{AnyUserData, IntoLuaMulti, UserDataRef};

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

impl<'lua, T> IntoLuaMulti<'lua> for TypedUserData<'lua, T> {
    fn into_lua_multi(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::MultiValue<'lua>> {
        self.ud.into_lua_multi(lua)
    }
}
