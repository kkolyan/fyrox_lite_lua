use std::ops::Deref;

use fyrox_lite_model::{DataType, NamedValue};

use crate::context::GenerationContext;

pub fn mlua_to_rust_expr(param: &NamedValue, ctx: &GenerationContext) -> String {
    match &param.ty {
        DataType::UnresolvedClass(_) => panic!(),
        DataType::Unit => param.name.to_string(),
        DataType::Bool => param.name.to_string(),
        DataType::Byte => param.name.to_string(),
        DataType::I32 => param.name.to_string(),
        DataType::I64 => param.name.to_string(),
        DataType::F32 => param.name.to_string(),
        DataType::F64 => param.name.to_string(),
        DataType::String => format!("{}.to_str()?.to_string()", param.name),
        DataType::Vec(_it) => todo!(),
        DataType::UserScript => param.name.to_string(),
        DataType::UserScriptMessage => {
            // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
            format!("Traitor::new(send_wrapper::SendWrapper::new(unsafe {{ std::mem::transmute::<_, mlua::Value<'static>>({}) }} ))", param.name)
        }
        DataType::UserScriptGenericStub => "Default::default()".to_string(),
        DataType::Object(_it) => format!("{}.borrow()?.inner().clone().into()", param.name),
        DataType::Option(_it) => format!(
            "if let Some({}) = {} {{ Some({}) }} else {{ None }}",
            param.name,
            param.name,
            mlua_to_rust_expr(param, ctx)
        ),
        DataType::Result { ok: _ } => todo!(),
    }
}

pub fn rust_expr_to_mlua(ctx: &GenerationContext, param: &str, ty: &DataType) -> String {
    match &ty {
        DataType::UnresolvedClass(_) => todo!(),
        DataType::Unit => param.to_string(),
        DataType::Bool => param.to_string(),
        DataType::Byte => param.to_string(),
        DataType::I32 => param.to_string(),
        DataType::I64 => param.to_string(),
        DataType::F32 => param.to_string(),
        DataType::F64 => param.to_string(),
        DataType::String => param.to_string(),
        DataType::Vec(it) => format!(
            "lua.create_table_from({}.into_iter().map(|it| {}))",
            param,
            rust_expr_to_mlua(ctx, "it", it.deref())
        ),
        DataType::UserScript => param.to_string(),
        DataType::UserScriptMessage => todo!(),
        DataType::UserScriptGenericStub => todo!(),
        DataType::Object(class_name) => {
            let mut rust_name = ctx
                .domain
                .classes
                .iter()
                .find(|it| it.class_name() == class_name)
                .unwrap()
                .rust_name();
            if let Some(rn) = ctx.internal_to_external.get(rust_name) {
                rust_name = rn;
            }
            format!("Traitor::new({}::from({}))", rust_name.0, param)
        }
        DataType::Option(it) => format!(
            "if let Some({}) = {} {{ Some({}) }} else {{ None }}",
            param,
            param,
            rust_expr_to_mlua(ctx, param, it.deref(),)
        ),
        DataType::Result { ok } => format!("{}?", rust_expr_to_mlua(ctx, param, ok.deref())),
    }
}
