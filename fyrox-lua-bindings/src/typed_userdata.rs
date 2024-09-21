use std::marker::PhantomData;

use mlua::{AnyUserData, IntoLua};

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
