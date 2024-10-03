use std::ops::Deref;

use fyrox_lite_model::{ DataType};

use crate::context::GenerationContext;

pub fn mlua_to_rust_expr(param: &str, ty: &DataType, ctx: &GenerationContext) -> String {
    match ty {
        DataType::UnresolvedClass(_) => panic!(),
        DataType::Unit => param.to_string(),
        DataType::Bool => param.to_string(),
        DataType::Byte => param.to_string(),
        DataType::I32 => param.to_string(),
        DataType::I64 => param.to_string(),
        DataType::F32 => param.to_string(),
        DataType::F64 => param.to_string(),
        DataType::String => format!("{}.to_str()?.to_string()", param),
        DataType::Vec(_it) => todo!(),
        DataType::UserScript => param.to_string(),
        DataType::UserScriptMessage => {
            // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
            format!("Traitor::new(send_wrapper::SendWrapper::new(unsafe {{ std::mem::transmute::<_, mlua::Value<'static>>({}) }} ))", param)
        }
        DataType::UserScriptGenericStub => "Default::default()".to_string(),
        DataType::Object(_it) => format!("{}.borrow()?.inner().clone().into()", param),
        DataType::Option(_it) => format!(
            "if let Some({}) = {} {{ Some({}) }} else {{ None }}",
            param,
            param,
            mlua_to_rust_expr(param, ty, ctx)
        ),
        DataType::Result { ok: _ } => todo!(),
    }
}

pub fn rust_expr_to_mlua(ctx: &GenerationContext, param: &str, ty: &DataType) -> String {
    match &ty {
        DataType::UnresolvedClass(it) => todo!("unresolved class: {}", it),
        DataType::Unit => param.to_string(),
        DataType::Bool => param.to_string(),
        DataType::Byte => param.to_string(),
        DataType::I32 => param.to_string(),
        DataType::I64 => param.to_string(),
        DataType::F32 => param.to_string(),
        DataType::F64 => param.to_string(),
        DataType::String => param.to_string(),
        DataType::Vec(it) => format!(
            "lua.create_table_from({}.into_iter().map(|it| {}).enumerate())",
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

pub fn type_to_mlua(ty: &DataType, ctx: &GenerationContext) -> String {
    match ty {
        DataType::UnresolvedClass(_it) => todo!(),
        DataType::Unit => panic!("Unit type is not epxected to be rendered implicitly"),
        DataType::Bool => "bool".to_owned(),
        DataType::Byte => "u8".to_owned(),
        DataType::I32 => "i32".to_owned(),
        DataType::I64 => "i64".to_owned(),
        DataType::F32 => "f32".to_owned(),
        DataType::F64 => "f64".to_owned(),
        DataType::String => "mlua::String".to_owned(),
        DataType::Vec(it) => format!("Vec<{}>", type_to_mlua(it.deref(), ctx)),
        DataType::UserScript => "TypedUserData<ScriptObject>".to_string(),
        DataType::UserScriptMessage => "mlua::Value".to_string(),
        DataType::UserScriptGenericStub => panic!("WTF, it should be filtered out before!"),
        DataType::Object(it) => {
			let class = ctx.domain.classes.iter().find(|item| item.class_name() == it).unwrap();
			let mut rust_name = class.rust_name();
			if let Some(ext) = ctx.internal_to_external.get(rust_name) {
				rust_name = ext;
			}
			format!("TypedUserData<Traitor<{}>>", rust_name.0)
		},
        DataType::Option(it) => format!("Option<{}>", type_to_mlua(it.deref(), ctx)),
        DataType::Result { ok: it } => format!("mlua::Result<{}>", type_to_mlua(it, ctx)),
    }
}
