use std::ops::Deref;

use fyrox_lite_model::DataType;

use crate::context::GenerationContext;


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
