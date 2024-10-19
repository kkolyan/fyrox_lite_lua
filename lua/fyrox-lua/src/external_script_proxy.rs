use super::script_object_residence::ScriptResidence;
use crate::fyrox_lua_plugin::PluginsRefMut_Ext;
use crate::lua_lifecycle::invoke_callback;
use crate::script_metadata::ScriptKind;
use crate::script_object_residence::ensure_unpacked;
use crate::user_data_plus::Traitor;
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use fyrox_lite::lite_event::to_lite;
use fyrox_lite::reflect_base;
use mlua::Value;
use send_wrapper::SendWrapper;
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, ComponentProvider)]
pub struct ExternalScriptProxy {
    pub name: String,
    pub data: ScriptResidence,
}

impl ScriptTrait for ExternalScriptProxy {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
        invoke_callback(
            &mut self.data,
            ctx,
            "on_init",
            || Ok(()),
        );
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
        invoke_callback(
            &mut self.data,
            ctx,
            "on_start",
            || Ok(()),
        );
    }

    fn on_deinit(&mut self, ctx: &mut fyrox::script::ScriptDeinitContext) {
        ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
        invoke_callback(
            &mut self.data,
            ctx,
            "on_deinit",
            || Ok(()),
        );
    }

    fn on_os_event(&mut self, event: &fyrox::event::Event<()>, ctx: &mut ScriptContext) {
        if let Some(event) = to_lite(event.clone()) {
            ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
            invoke_callback(
                &mut self.data,
                ctx,
                "on_os_event",
                || Ok(Traitor::new(event.clone())),
            );
        }
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
        let dt = ctx.dt;
        invoke_callback(
            &mut self.data,
            ctx,
            "on_update",
            || Ok(dt),
        );
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(lua_message) = message.downcast_ref::<Traitor<SendWrapper<Value>>>() {
            ensure_unpacked(&mut self.data, ctx.plugins.lua_mut());
            invoke_callback(
                &mut self.data,
                ctx,
                "on_message",
                || Ok(Value::clone(lua_message)),
            );
        } else {
            panic!("non-lua messages not supported by lua scripts")
        }
    }
}


impl BaseScript for ExternalScriptProxy {
    fn clone_box(&self) -> Box<dyn ScriptTrait> {
        Box::new(self.clone())
    }
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn id(&self) -> Uuid {
        self.data.id()
    }
}

impl Visit for ExternalScriptProxy {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.data.visit(name, visitor)
    }
}

impl Reflect for ExternalScriptProxy {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        self.data.with_script_object(|it| it.fields_info(func))
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        self.data.with_script_object(|it| it.fields(func))
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        self.data.with_script_object_mut(|it| it.fields_mut(func))
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        self.data.with_script_object(|it| it.field(name, func))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        self.data
            .with_script_object_mut(|it| it.field_mut(name, func))
    }
}
