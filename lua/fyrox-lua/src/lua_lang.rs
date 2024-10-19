use std::ops::DerefMut;

use fyrox::core::{visitor::Visit, Uuid};
use fyrox_lite::{script_object::Lang, script_object_residence::uuid_of_script};
use mlua::{Table, Value};
use send_wrapper::SendWrapper;

use crate::{lua_lifecycle::lua_vm, script_object::ScriptObject, typed_userdata::TypedUserData, user_data_plus::Traitor};

#[derive(Debug, Clone)]
pub struct LuaLang;

impl Lang for LuaLang {
    type String<'a> = mlua::String<'a>;
    type RuntimePin = String;
    type UnpackedScriptObject = UnpackedScriptObjectVisit;

    fn drop_runtime_pin(runtime_pin: &mut Self::RuntimePin) {
        lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .set(runtime_pin.as_str(), Value::Nil)
            .unwrap();
    }

    fn clone_runtime_pin(runtime_pin: &Self::RuntimePin) -> Self::RuntimePin {
        let new = Uuid::new_v4().to_string();
        let ex_value = lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .get::<_, mlua::Value>(runtime_pin.as_str())
            .unwrap();
        lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .set(new.as_str(), ex_value)
            .unwrap();
        new
    }

    fn drop_script_object_to_prevent_delayed_destructor(script: &mut Self::UnpackedScriptObject) {
        // take ScriptObject out of Lua VM and destroy it right now to prevent nested destructors
        // to be invoked at random moment in future by Lua GC, anth thus ruin Hit Reload
        if let Ok(it) = TypedUserData::take(script.0.deref_mut()) {
            let _s: ScriptObject = it.into_inner();
        }
    }
    
    fn id_of(script: &Self::UnpackedScriptObject) -> Uuid {
        uuid_of_script(&script.0.borrow().unwrap())
    }
}

pub struct UnpackedScriptObjectVisit(pub SendWrapper<TypedUserData<'static, Traitor<ScriptObject>>>);

impl Visit for UnpackedScriptObjectVisit {
    fn visit(&mut self, name: &str, visitor: &mut fyrox::core::visitor::Visitor) -> fyrox::core::visitor::VisitResult {
        self.0.borrow_mut().unwrap().visit(name, visitor)
    }
}
