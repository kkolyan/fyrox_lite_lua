//! Game project.
use fyrox::{core::{
    reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*, TypeUuidProvider,
}, script::ScriptTrait};
use mlua::{UserData, UserDataRef};
use send_wrapper::SendWrapper;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ScriptObject {}

impl UserData for ScriptObject {}

#[derive(Visit, Reflect, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "c5671d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
pub struct LuaScript {
    pub script_name: String,

    #[visit(skip)]
    #[reflect(hidden)]
    pub script_data: SendWrapper<&'static UserDataRef<'static, ScriptObject>>,
}

impl Debug for LuaScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LuaScript")
            .field("script_name", &self.script_name)
            .finish()
    }
}

impl Clone for LuaScript {
    fn clone(&self) -> Self {
        panic!()
    }
}

impl ScriptTrait for LuaScript {}
