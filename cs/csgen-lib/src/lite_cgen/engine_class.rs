use std::collections::HashSet;

use gen_common::templating::render;
use lite_model::{Class, ClassName, DataType, Domain, EngineClass};
use to_vec::ToVec;

use super::types;

pub(crate) fn generate_engine_class(s: &mut String, class: &EngineClass, client_replicated_types: &HashSet<ClassName>) {
    for method in class.methods.iter() {
        let mut input_args = vec![];
        let mut output_args = vec![];

        let params_final = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
            .to_vec();

        if method.instance {
            input_args.push("__this: NativeInstanceId".to_string());
        }

        for param in params_final.iter() {
            input_args.push(format!(
                "{}: {}",
                param.name,
                types::generate_ffi_type(&param.ty, client_replicated_types)
            ));
        }

        render(
            s,
            r#"
            pub extern "C" fn fyrox_lite_${class}_${method}(${input_args}) -> ${return} {
        "#,
            [
                ("class", &class.class_name),
                ("method", &method.method_name),
                ("input_args", &input_args.join(", ")),
                (
                    "return",
                    &match &method.signature.return_ty {
                        Some(it) => types::generate_ffi_type(it, client_replicated_types),
                        None => "()".to_string(),
                    },
                ),
            ],
        );

        for param in params_final.iter() {
            output_args.push(format!("{}", param.name));

            render(
                s,
                "
                let ${var} = ${expr};
            ",
                [
                    ("expr", &types::generate_from_native(&param.ty, &param.name)),
                    ("var", &param.name),
                ],
            );
        }

        render(
            s,
            r#"
                let __result = ${receiver}${method}(${output_args});
        "#,
            [
                ("method", &method.method_name),
                ("output_args", &output_args.join(", ")),
                (
                    "receiver",
                    &match method.instance {
                        true => "__this.".to_string(),
                        false => format!("{}::", class.rust_struct_path),
                    },
                ),
            ],
        );

        if let Some(ty) = &method.signature.return_ty {
            render(
                s,
                "
                let __result = ${expr};
            ",
                [("expr", &types::generate_to_native(ty, "__result", client_replicated_types))],
            );
        }

        render(
            s,
            r#"
                __result
            }
        "#,
            [],
        );
    }
}
