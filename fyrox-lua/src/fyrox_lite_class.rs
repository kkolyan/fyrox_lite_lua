use std::{fmt::Debug, marker::PhantomData, ops::{Deref, DerefMut}};

use fyrox::core::log::Log;
use mlua::{Lua, MetaMethod, UserData, UserDataFields, UserDataMethods};





#[derive(Clone, Copy, Debug)]
pub struct Traitor<T>(T);

impl<T> Deref for Traitor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Traitor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Traitor<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub trait FyroxUserData: Sized + 'static {
    const CLASS_NAME: &'static str;

    #[allow(unused_variables)]
    fn add_instance_fields<'lua, F: UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {}

    #[allow(unused_variables)]
    fn add_instance_methods<'lua, M: UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {}

    #[allow(unused_variables)]
    fn add_class_fields<'lua, F: UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}

    #[allow(unused_variables)]
    fn add_class_methods<'lua, M: UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {}
    
    fn register_class(lua: &Lua) {
        let value = UserDataClass::<Self> {
            pd: Default::default(),
        };
        let name = Self::CLASS_NAME;
        Log::info(format!("register Lua class {}", name));
        lua.globals().set(name, value).unwrap();
    }
}

impl<T> UserData for Traitor<T>
where
    T: FyroxUserData,
    T: Debug
{

    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), T::CLASS_NAME);
        T::add_instance_fields(fields);
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString.name(), |_lua, this, _args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        T::add_instance_methods(methods);
    }
}

impl<T> UserData for UserDataClass<T>
where
    T: FyroxUserData,
{
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_meta_field(MetaMethod::Type.name(), format!("Class<{}>", T::CLASS_NAME));
        T::add_class_fields(fields);
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        T::add_class_methods(methods);
    }
}

pub struct UserDataClass<T> {
    pd: PhantomData<T>,
}
