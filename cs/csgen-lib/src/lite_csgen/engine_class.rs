use convert_case::{Case, Casing};
use to_vec::ToVec;
use gen_common::code_model::{ModContent, Module};
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::{ConstantValue, DataType, EngineClass, Literal};
use crate::lite_csgen::{api_types, wrappers};
use crate::lite_csgen::api_types::TypeMarshalling;

pub(crate) fn generate_bindings(class: &EngineClass, ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    render(&mut s, r#"
            // ${rust_path}
            [StructLayout(LayoutKind.Sequential)]
            public readonly partial struct ${class}
            {
    "#, [("class", &class.class_name), ("rust_path", &class.rust_struct_path)]);

    for constant in class.constants.iter() {
        let value = match &constant.value {
            ConstantValue::Literal(it) => match it {
                Literal::Bool(it) => format!("{}", it),
                Literal::Byte(it) => format!("{}", it),
                Literal::Number(it) => format!("{}", it),
                Literal::String(it) => format!("{:?}", it),
            },
            ConstantValue::ComplexExpression(expr) => {
                render(&mut s, r#"
                //public const ${type} ${name} = ${value};
                "#, [
                    ("type", &api_types::type_rs2cs(&constant.ty).to_facade()),
                    ("name", &constant.const_name),
                    ("value", &expr)
                ]);
                continue;
            }
        };

        render(&mut s, r#"
                public const ${type} ${name} = ${value};
                "#, [
            ("type", &api_types::type_rs2cs(&constant.ty).to_facade()),
            ("name", &constant.const_name),
            ("value", &value)
        ]);
    }

    for method in class.methods.iter() {
        let has_class_name_arg = method.signature.params.iter().any(|it| matches!(&it.ty, DataType::ClassName));
        let input_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::ClassName))
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| format!("{} {}", api_types::type_rs2cs(&param.ty).to_facade(), param.name))
            .to_vec();
        let output_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| match &param.ty {
                DataType::ClassName => format!("typeof(T).Name"),
                it => format!("_{}", param.name),
            })
            .to_vec();

        let mut converted_params = String::new();
        for param in method.signature.params.iter() {
            if matches!(&param.ty, DataType::ClassName | DataType::UserScriptGenericStub | DataType::Buffer(_)) {
                continue;
            }
            if let TypeMarshalling::Mapped { blittable, .. } = api_types::type_rs2cs(&param.ty) {
                render(&mut converted_params, r#"
                        var _${name} = ${blittable}.FromFacade(${name});
                        "#, [("name", &param.name), ("blittable", &blittable)]);
            } else {
                render(&mut converted_params, r#"
                        var _${name} = ${name};
                        "#, [("name", &param.name)]);
            }
        }

        if let Some(return_ty) = &method.signature.return_ty {
            let return_ty = api_types::type_rs2cs(return_ty);
            render(&mut s, r#"

                public ${static}${return_ty} ${name}${generics}(${input_params})${generic_where}
                {
                    unsafe {
                        ${converted_params}
                        var __ret = ${rust_path_escaped}_${name}(${this}${output_params});
                        return ${ret_expr}${generic_cast};
                    }
                }
        "#, [
                ("static", &if !method.instance { "static " } else { "" }),
                ("return_ty", &if has_class_name_arg { return_ty.to_facade_generic() } else { return_ty.to_facade() }),
                ("ret_expr", &if return_ty.is_mapped() { format!("{}.ToFacade(__ret)", return_ty.to_blittable()) } else { "__ret".to_string() }),
                ("generic_cast", &if has_class_name_arg {" as T"} else {""}),
                ("generic_where", &if has_class_name_arg {" where T : class"} else {""}),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                ("this", &if method.instance { if output_params.is_empty() { "this" } else { "this, " } } else { "" }),
                ("converted_params", &converted_params.trim_start()),
            ]);
        } else {
            render(&mut s, r#"

                public ${static}void ${name}${generics}(${input_params})
                {
                    unsafe {
                        ${converted_params}
                        ${rust_path_escaped}_${name}(${this}${output_params});
                    }
                }
        "#, [
                ("static", &if !method.instance { "static " } else { "" }),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                ("this", &if method.instance { if output_params.is_empty() { "this" } else { "this, " } } else { "" }),
                ("converted_params", &converted_params.trim_start()),
            ]);
        }
    }
    for method in class.methods.iter() {
        let native_input_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| format!("{} {}", api_types::type_rs2cs(&param.ty).to_blittable(), param.name))
            .to_vec();
        render(&mut s, r#"
                
                [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                private static unsafe partial ${return_ty} ${rust_path_escaped}_${name}(${this}${native_input_params});
        "#, [
            ("return_ty", &method.signature.return_ty.as_ref().map(|it| api_types::type_rs2cs(it).to_blittable()).unwrap_or("void".to_string())),
            ("name", &method.method_name.to_case(Case::Pascal)),
            ("native_input_params", &native_input_params.join(", ")),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("this", &if method.instance { format!("{} {}", class.class_name, if native_input_params.is_empty() { "self" } else { "self, " }) } else { "".to_string() }),
        ]);
    }

    render(&mut s, r#"
            }
    "#, []);

    wrappers::generate_optional(&mut s, &DataType::Object(class.class_name.clone()));

    Module::code(&class.class_name, s)
}