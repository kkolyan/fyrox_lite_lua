use convert_case::{Case, Casing};
use to_vec::ToVec;
use gen_common::code_model::{ModContent, Module};
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::{ConstantValue, DataType, EngineClass, Literal};
use crate::lite_csgen::api_types;

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
        let generics = method.signature.params.iter().any(|it| matches!(&it.ty, DataType::ClassName));
        let input_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::ClassName))
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| format!("{} {}", api_types::type_rs2cs(&param.ty).to_facade(), param.name))
            .to_vec();
        let output_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| match &param.ty {
                DataType::ClassName => format!("typeof(T).Name"),
                it => param.name.clone(),
            })
            .to_vec();
        render(&mut s, r#"

                public ${static}${return_ty} ${name}${generics}(${input_params})
                {
                    unsafe {
                        ${fix}${return_stmnt}${rust_path_escaped}_${name}(${this}${output_params});
                    }
                }
        "#, [
            ("static", &if !method.instance { "static " } else { "" }),
            ("return_ty", &method.signature.return_ty.as_ref().map(|it| api_types::type_rs2cs(it).to_facade()).unwrap_or("void".to_string())),
            ("name", &method.method_name.to_case(Case::Pascal)),
            ("input_params", &input_params.join(",")),
            ("output_params", &output_params.join(",")),
            ("return_stmnt", &if method.signature.return_ty.is_some() { "return " } else { "" }),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("generics", &if generics { "<T>" } else { "" }),
            ("this", &if method.instance { "self, " } else { "" }),
            ("fix", &if method.instance { format!("fixed ({}* self = &this) ", class.class_name) } else { "".to_owned() }),
        ]);
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
            ("native_input_params", &native_input_params.join(",")),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("this", &if method.instance {format!("{}* self, ", class.class_name)} else {"".to_string()}),
        ]);
        
    }

    render(&mut s, r#"
            }
    "#, []);

    Module::code(&class.class_name, s)
}