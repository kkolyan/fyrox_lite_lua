use convert_case::Case;
use convert_case::Casing;
use fyrox::core::log::Log;
use fyrox::core::visitor::Visit;
use fyrox::core::visitor::VisitResult;
use fyrox::core::visitor::Visitor;
use fyrox::core::Uuid;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::DerefMut;

use crate::script_metadata::ScriptKind;
use crate::script_object::Lang;
use crate::script_object::ScriptFieldValue;
use crate::script_object::ScriptObject;

/// Initially, when script is loaded from file (scene or save game), it's in "packed" mode.
/// First time this script receives `on_update` callback, it's converted to "unpacked", by
/// transfering state into UserData managed by Lua VM. Thoughm serialization should work fine,
/// because Visit is implemented in both modes.
pub enum ScriptResidence<T: Lang> {
    Packed(ScriptObject<T>),
    Unpacked(T::UnpackedScriptObject),
}

impl<T: Lang> ScriptResidence<T> {
    pub fn is_packed(&self) -> bool {
        match self {
            ScriptResidence::Packed(_) => true,
            ScriptResidence::Unpacked(_) => false,
        }
    }

    pub fn with_script_object<R>(&self, f: impl FnOnce(&ScriptObject<T>) -> R) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&it.borrow().unwrap()),
        }
    }

    pub fn with_script_object_mut<R>(&mut self, f: impl FnOnce(&mut ScriptObject<T>) -> R) -> R {
        match self {
            ScriptResidence::Packed(it) => f(it),
            ScriptResidence::Unpacked(it) => todo!(),
            // ScriptResidence::Unpacked(it) => f(&mut it.borrow_mut().unwrap()),
        }
    }
    
    pub fn id(&self) -> Uuid {
        match self {
            ScriptResidence::Packed(it) => uuid_of_script(it),
            ScriptResidence::Unpacked(it) => T::id_of(it),
        }
    }
}

pub fn uuid_of_script<T: Lang>(script: &ScriptObject<T>) -> Uuid {
    match script.def.metadata.kind {
        ScriptKind::Script(uuid) => uuid,
        ScriptKind::Plugin => panic!("not expected to be called for Plugin scripts"),
    }
}

impl<T: Lang> Debug for ScriptResidence<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.with_script_object(|it| it.fmt(f))
    }
}

impl<T: Lang> Clone for ScriptResidence<T> {
    fn clone(&self) -> Self {
        match self {
            ScriptResidence::Packed(it) => ScriptResidence::Packed(it.clone()),

            // will implement when know when cloning is really needed during game cycle
            ScriptResidence::Unpacked(_) => panic!("cloning for Lua-backed ScriptData is not supported"),
        }
    }
}

impl<T: Lang> Drop for ScriptResidence<T> {
    fn drop(&mut self) {
        match self {
            ScriptResidence::Packed(_it) => {
                // ScriptObject is dropped automatically without delay
            },
            ScriptResidence::Unpacked(it) => {
                T::drop_script_object_to_prevent_delayed_destructor(it);
            },
        }
    }
}

impl<T: Lang> Visit for ScriptResidence<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {

        match self {
            ScriptResidence::Packed(it) => it.visit(name, visitor),
            ScriptResidence::Unpacked(it) => it.visit(name, visitor),
        }
    }
}


impl<T: Lang> Visit for ScriptObject<T> {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut guard = visitor.enter_region(name)?;

        let it = self;
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
    }
}