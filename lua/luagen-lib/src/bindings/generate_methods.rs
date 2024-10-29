use std::borrow::Cow;

use lite_model::{DataType, EngineClass, Method, Param};
use to_vec::ToVec;

use gen_common::{
    context::GenerationContext,
    templating::render,
};
use super::expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua};

pub fn generate_methods(
    s: &mut String,
    class: &EngineClass,
    ctx: &GenerationContext,
    instance: bool,
) {
    for method in class
        .methods
        .iter()
        .filter(|it| is_regular(it))
        .filter(|it| it.instance == instance)
    {
        let params = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
            .to_vec();

        let generics = match method.is_generic() {
            true => "::<TypedUserData<Traitor<ScriptObject>>>",
            false => "",
        };

        let input_names = params.iter()
            .filter(|it| !matches!(it.ty, DataType::Buffer(_)))
            .map(|it| it.name.as_str())
            .to_vec()
            .join(", ");

        let output_names = method
            .signature
            .params
            .iter()
            .map(|it| it.name.as_str())
            .to_vec()
            .join(", ");

        let param_types = params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::Buffer(_)))
            .map(|it| type_to_mlua(&it.ty, ctx))
            .to_vec()
            .join(", ");

        render(
            s,
            r#"
                methods.add_method_mut(
                    "${method_name}",
                    |lua, this, (${input_names}): (${param_types})| {
            "#,
            [
                ("method_name", &method.method_name),
                ("input_names", &input_names),
                ("param_types", &param_types),
            ],
        );

        for param in method.signature.params.iter() {
            if let DataType::Buffer(ty) = &param.ty {
                render(
                    s,
                    r#"
                        let ${param_name} = Vec::new();
                "#,
                    [("param_name", &param.name), ],
                );
            } else {
                let expression = mlua_to_rust_expr(&param.name, &param.ty, ctx);
                render(
                    s,
                    r#"
                        let ${param_name} = ${expression};
                "#,
                    [("param_name", &param.name), ("expression", &expression)],
                );
            }
        }

        let target = match instance {
            true => Cow::Borrowed("this."),
            false => Cow::Owned(format!("{}::", class.rust_struct_path.0)),
        };

        let ret_expression = rust_expr_to_mlua(
            ctx,
            "ret",
            method
                .signature
                .return_ty
                .as_ref()
                .unwrap_or(&DataType::Unit),
        );

        render(
            s,
            r#"
                        let ret = ${target}${method_name}${generics}(${output_names});
                        let ret = ${ret_expression};
                        Ok(ret)
                    },
                );
            "#,
            [
                ("target", &target),
                ("method_name", &method.method_name),
                ("output_names", &output_names),
                ("generics", &generics),
                ("ret_expression", &ret_expression),
            ],
        );
    }
}

pub(crate) fn is_setter(method: &Method) -> bool {
    method.method_name.starts_with("set_") && method.signature.params.len() == 1
}

pub(crate) fn is_getter(method: &Method) -> bool {
    method.method_name.starts_with("get_") && method.signature.params.is_empty() && method.signature.return_ty.is_some()
}

pub(crate) fn is_regular(method: &Method) -> bool {
    !is_setter(method) && !is_getter(method)
}
