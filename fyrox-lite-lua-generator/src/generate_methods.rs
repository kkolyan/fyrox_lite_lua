use std::borrow::Cow;

use fyrox_lite_model::{DataType, EngineClass, NamedValue};
use to_vec::ToVec;

use crate::{
    context::GenerationContext,
    mlua_to_rust_expr::{mlua_to_rust_expr, rust_expr_to_mlua},
    templating::render,
    type_to_mlua::type_to_mlua,
};

pub fn generate_methods(
    s: &mut String,
    class: &EngineClass,
    ctx: &GenerationContext,
    instance: bool,
) {
    for method in class.methods.iter().filter(|it| it.instance == instance) {
        let params = method
            .signature
            .params
            .iter()
            .filter(|it| it.ty != DataType::UserScriptGenericStub)
            .to_vec();

        let generics = method
            .signature
            .params
            .iter()
            .find(|it| {
                matches!(
                    it.ty,
                    DataType::UserScript
                        | DataType::UserScriptMessage
                        | DataType::UserScriptGenericStub
                )
            })
            .map(|_| "::<TypedUserData<ScriptObject>>")
            .unwrap_or("");

        let input_names = params.iter().map(|it| it.name.as_str()).to_vec().join(", ");

        let output_names = method
            .signature
            .params
            .iter()
            .map(|it| it.name.as_str())
            .to_vec()
            .join(", ");

        let param_types = params
            .iter()
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
            let expression = mlua_to_rust_expr(param, ctx);
            render(
                s,
                r#"
						let ${param_name} = ${expression};
				"#,
                [("param_name", &param.name), ("expression", &expression)],
            );
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
