use convert_case::Case;
use convert_case::Casing;
use fyrox::core::log::Log;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::VisitResult;
use fyrox::core::visitor::Visitor;
use send_wrapper::SendWrapper;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::plugin::LuaPlugin;
use crate::script::ScriptFieldValue;
use crate::script_object::ScriptObject;
use crate::typed_userdata::TypedUserData;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum ScriptData {
    Packed(ScriptObject),
    Unpacked(SendWrapper<TypedUserData<'static, ScriptObject>>),
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
            ScriptData::Unpacked(it) => f(&it.borrow().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(&mut self, f: impl FnOnce(&mut ScriptObject) -> R) -> R {
        match self {
            ScriptData::Packed(it) => f(it),
            ScriptData::Unpacked(it) => f(&mut it.borrow_mut().unwrap()),
        }
    }

    pub fn inner_unpacked(&self) -> Option<TypedUserData<'static, ScriptObject>> {
        match self {
            ScriptData::Packed(_it) => None,
            ScriptData::Unpacked(it) => Some(TypedUserData::clone(it)),
        }
    }

    pub fn ensure_unpacked(&mut self, plugin: &mut LuaPlugin) {
        if plugin.failed {
            // don't spam logs, though, plugin is completely broken at this point
            return;
        }
        if self.is_packed() {
            // script was just loaded from the scene file or safe game. unpack it!
            *self = match self {
                ScriptData::Packed(it) => {
                    let so = plugin
                        .vm
                        .create_userdata(it.clone())
                        .map(TypedUserData::<ScriptObject>::new)
                        .map(SendWrapper::new)
                        .map(ScriptData::Unpacked);
                    match so {
                        Ok(it) => it,
                        Err(err) => {
                            Log::err(format!("failed to unpack LuaScript: {:?}", err));
                            plugin.failed = true;
                            return;
                        }
                    }
                }
                ScriptData::Unpacked(_) => panic!("WTF?"),
            }
        }
    }
}

impl Debug for ScriptData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.with_script_object(|it| it.fmt(f))
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

impl Visit for ScriptData {
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
                    ScriptFieldValue::RawLuaValue(_it) => Ok(()),
                };
                if let Err(err) = &result {
                    Log::warn(format!("skipping deserialization of field `{}::{}` due to error: {}", it.def.metadata.class, field_name, err).as_str());
                }
            }
            Ok(())
        })
    }
}
