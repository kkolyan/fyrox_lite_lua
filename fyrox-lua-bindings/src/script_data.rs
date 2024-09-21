use mlua::AnyUserData;
use mlua::UserDataRefMut;
use send_wrapper::SendWrapper;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::script_object::ScriptObject;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum ScriptData {
    Packed(ScriptObject),
    Unpacked(SendWrapper<AnyUserData<'static>>),
}

impl ScriptData {
    pub fn is_packed(&self) -> bool {
        match self {
            ScriptData::Packed(_) => true,
            ScriptData::Unpacked(_) => false,
        }
    }

    pub fn with_script_object<R>(&self, f: impl FnOnce(&ScriptObject) -> R) -> R {
        match self {
            ScriptData::Packed(it) => f(it),
            ScriptData::Unpacked(it) => f(&it.borrow::<ScriptObject>().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(&mut self, f: impl FnOnce(&mut ScriptObject) -> R) -> R {
        match self {
            ScriptData::Packed(it) => f(it),
            ScriptData::Unpacked(it) => f(&mut it.borrow_mut::<ScriptObject>().unwrap()),
        }
    }
    pub fn as_script_object(&self) -> &ScriptObject {
        match self {
            ScriptData::Packed(it) => it,
            ScriptData::Unpacked(it) => it,
        }
    }

    pub fn as_script_object_mut(&mut self) -> &mut ScriptObject {
        match self {
            ScriptData::Packed(it) => it,
            ScriptData::Unpacked(it) => it.borrow_mut::<ScriptObject>().unwrap(),
        }
    }

    pub fn inner_unpacked(&mut self) -> UserDataRefMut<'static, ScriptObject> {
        match self {
            ScriptData::Packed(it) => panic!(),
            ScriptData::Unpacked(it) => it.clone(),
        }
    }
}

impl Debug for ScriptData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_script_object().fmt(f)
    }
}

impl Clone for ScriptData {
    fn clone(&self) -> Self {
        match self {
            ScriptData::Packed(it) => ScriptData::Packed(it.clone()),

            // will implement when know when cloning is really needed during game cycle
            ScriptData::Unpacked(_) => panic!("cloning for Lua-backed ScriptData is not supported"),
        }
    }
}
