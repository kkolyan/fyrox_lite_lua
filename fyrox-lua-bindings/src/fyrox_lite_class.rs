use std::marker::PhantomData;

use fyrox::core::log::Log;
use mlua::{Lua, MetaMethod, UserData};

use crate::fyrox_lite::Traitor;



pub struct UserDataClass<T> {
    pd: PhantomData<T>,
}

#[extend::ext]
pub impl<T> T
where
    UserDataClass<T>: UserData,
    T: 'static,
    Traitor<T>: UserData,
{
    fn register_class(lua: &Lua) {
        let value = UserDataClass::<T> {
            pd: Default::default(),
        };
        let name = lua
            .create_proxy::<Traitor<T>>()
            .unwrap()
            .get_metatable()
            .unwrap()
            .get::<Option<mlua::String>>(MetaMethod::Type.name())
            .unwrap()
            .unwrap();
        let name = name.to_str().unwrap();
        Log::info(format!("register Lua class {}", name));
        lua.globals().set(name, value).unwrap();
    }
}