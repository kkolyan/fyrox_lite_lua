use std::{collections::HashMap, mem};

use fyrox::core::log::Log;
use mlua::{MetaMethod, UserData};

#[derive(Debug)]
pub struct ScriptClass {
	pub name: &'static str,
    pub table: HashMap<String, mlua::Value<'static>>,
}


impl UserData for ScriptClass {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(
            MetaMethod::NewIndex.name(),
            |_lua, this, (k, v): (mlua::String, mlua::Value)| {
                let static_v: mlua::Value<'static> = unsafe {
                    // we use Lua the whole life of the program, so that seems sound
                    mem::transmute(v)
                };
                Log::info(format!("register user-defined method: {}#{}", this.name, k.to_string_lossy()));
                this.table.insert(k.to_string_lossy().to_string(), static_v);
                Ok(())
            },
        );
        methods.add_meta_method(
            MetaMethod::Index.name(),
            |_lua, this, (k, v): (mlua::String, mlua::Value)| {
				let value = this.table.get(&k.to_string_lossy().to_string()).unwrap_or_else(|| &mlua::Value::Nil);
				Ok(value.clone())
            },
        );
    }
}
