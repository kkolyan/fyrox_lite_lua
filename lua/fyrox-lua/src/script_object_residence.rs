use convert_case::Case;
use convert_case::Casing;
use fyrox::core::log::Log;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::VisitResult;
use fyrox::core::visitor::Visitor;
use send_wrapper::SendWrapper;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::DerefMut;

use crate::fyrox_lua_plugin::LuaPlugin;
use crate::lua_lifecycle::lua_vm;
use crate::script_object::ScriptFieldValue;
use crate::script_object::ScriptObject;
use crate::typed_userdata::TypedUserData;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum ScriptResidence {
    Packed(ScriptObject),
    Unpacked(SendWrapper<TypedUserData<'static, ScriptObject>>),
}

impl ScriptResidence {
    pub fn is_packed(&self) -> bool {
        match self {
            ScriptResidence::Packed(_) => true,
            ScriptResidence::Unpacked(_) => false,
        }
    }

    pub fn with_script_object<R>(&self, f: impl FnOnce(&ScriptObject) -> R) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(it) => f(&it.borrow().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(&mut self, f: impl FnOnce(&mut ScriptObject) -> R) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(it) => f(&mut it.borrow_mut().unwrap()),
        }
    }

    pub fn inner_unpacked(&self) -> Option<TypedUserData<'static, ScriptObject>> {
        match self {
            ScriptResidence::Packed(_it) => None,
            ScriptResidence::Unpacked(it) => Some(TypedUserData::clone(it)),
        }
    }

    pub fn ensure_unpacked(&mut self, plugin: &mut LuaPlugin) {
        if plugin.failed {
            // don't spam logs, though, plugin is completely broken at this point
            return;
        }
        if self.is_packed() {
            // script was just loaded from the scene file or safe game. unpack it!
            let data = match self {
                ScriptResidence::Packed(it) => {
                    let so = lua_vm()
                        .create_userdata(it.clone())
                        .map(TypedUserData::<ScriptObject>::new)
                        .map(SendWrapper::new)
                        .map(ScriptResidence::Unpacked);
                    match so {
                        Ok(it) => it,
                        Err(err) => {
                            Log::err(format!("failed to unpack LuaScript: {:?}", err));
                            plugin.failed = true;
                            return;
                        }
                    }
                }
                ScriptResidence::Unpacked(_) => panic!("WTF?"),
            };
            *self = data;
        }
    }
}

impl Debug for ScriptResidence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.with_script_object(|it| it.fmt(f))
    }
}

impl Clone for ScriptResidence {
    fn clone(&self) -> Self {
        match self {
            ScriptResidence::Packed(it) => ScriptResidence::Packed(it.clone()),

            // will implement when know when cloning is really needed during game cycle
            ScriptResidence::Unpacked(_) => panic!("cloning for Lua-backed ScriptData is not supported"),
        }
    }
}

impl Drop for ScriptResidence {
    fn drop(&mut self) {
        match self {
            ScriptResidence::Packed(_it) => {
                // ScriptObject is dropped automatically without delay
            },
            ScriptResidence::Unpacked(it) => {
                // take ScriptObject out of Lua VM and destroy it right now to prevent nested destructors 
                // to be invoked at random moment in future by Lua GC, anth thus ruin Hit Reload
                if let Ok(it) = TypedUserData::take(it.deref_mut()) {
                    let _s: ScriptObject = it;
                }
            },
        }
    }
}

impl Visit for ScriptResidence {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        self.with_script_object_mut(|it| {
            let def = it.def.clone();
            for (i, field) in def.metadata.fields.iter().enumerate() {
                let field_name = &field.name.to_case(Case::UpperCamel);
                let result = match &mut it.values[i] {
                    ScriptFieldValue::Number(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::String(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::Bool(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::Node(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::UiNode(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::Prefab(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::Vector3(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::Quaternion(it) => it.visit(field_name, &mut guard),
                    ScriptFieldValue::RuntimePin(it) => it.visit(&field_name, &mut guard),
                };
                if let Err(err) = &result {
                    Log::warn(format!("skipping deserialization of field `{}::{}` due to error: {}", it.def.metadata.class, field_name, err).as_str());
                }
            }
            Ok(())
        })
    }
}
