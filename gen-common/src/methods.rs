use lite_model::{DataType, Method};

pub struct MethodResult {
    pub may_fail: bool,
    pub success_type: DataType,
}

pub fn analyze_method_result(method: &Method) -> MethodResult {

    let (success_type, may_fail) = match &method.signature.return_ty {
        None => (DataType::Unit, false),
        Some(it) => match it {
            DataType::Result { ok } => (DataType::clone(ok), true),
            it => (it.clone(), false),
        }
    };

    MethodResult {
        may_fail,
        success_type,
    }
}