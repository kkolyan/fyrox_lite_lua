use std::collections::{HashMap, HashSet};
use convert_case::{Case, Casing};
use to_vec::ToVec;
use gen_common::code_model::{ModContent, Module};
use gen_common::context::GenerationContext;
use gen_common::templating::{render, render_string};
use lite_model::{ConstantValue, DataType, EngineClass, Literal, Method};
use crate::lite_csgen::{api_types, wrappers};
use crate::lite_csgen::api_types::CsType;
use crate::lite_csgen::gen_rs::RustEmitter;

pub(crate) fn generate_bindings(class: &EngineClass, ctx: &GenerationContext, rust: &mut RustEmitter) -> Module {
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
                    ("type", &api_types::type_cs(&constant.ty).to_facade()),
                    ("name", &constant.const_name),
                    ("value", &expr)
                ]);
                continue;
            }
        };

        render(&mut s, r#"
                public const ${type} ${name} = ${value};
                "#, [
            ("type", &api_types::type_cs(&constant.ty).to_facade()),
            ("name", &constant.const_name),
            ("value", &value)
        ]);
    }

    let mut props: HashMap<String, (Option<Getter>, Option<Setter>)> = HashMap::new();

    for method in class.methods.iter() {
        if let Some(getter) = as_getter(method) {
            let key = getter.prop_name.to_string();
            props.entry(key)
                .or_default()
                .0 = Some(getter);
        }
        if let Some(setter) = as_setter(method) {
            let key = setter.prop_name.to_string();
            props.entry(key)
                .or_default()
                .1 = Some(setter);
        }
    }

    for method in class.methods.iter() {
        if let Some(getter) = as_getter(method) {
            let Some((getter, setter)) = props.remove(&getter.prop_name) else { continue };
            generate_property(&mut s, class, getter, setter, ctx);
            continue;
        }
        if let Some(setter) = as_setter(method) {
            let Some((getter, setter)) = props.remove(&setter.prop_name) else { continue };
            generate_property(&mut s, class, getter, setter, ctx);
            continue;
        }
        generate_method(&mut s, class, method, ctx);
    }
    for method in class.methods.iter() {
        let native_input_params = method.signature.params.iter()
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| format!(
                "{}{} {}",
                api_types::type_cs(&param.ty).to_blittable(),
                if ctx.is_struct(&param.ty) { "*" } else { "" },
                param.name
            ))
            .to_vec();

        if let Some(DataType::Buffer(buffer_ty)) = &method.signature.return_ty {
            render(&mut s, r#"

                [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                private static unsafe partial int ${rust_path_escaped}_${name}(${this}${native_input_params});
            "#, [
                ("name", &method.method_name),
                ("native_input_params", &native_input_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("this", &if method.instance { format!("{} {}", class.class_name, if native_input_params.is_empty() { "self" } else { "self, " }) } else { "".to_string() }),
            ]);
        } else {
            render(&mut s, r#"

                [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                private static unsafe partial ${return_ty} ${rust_path_escaped}_${name}(${this}${native_input_params});
            "#, [
                ("return_ty", &method.signature.return_ty.as_ref().map(|it| api_types::type_cs(it).to_blittable()).unwrap_or("void".to_string())),
                ("name", &method.method_name),
                ("native_input_params", &native_input_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("this", &if method.instance { format!("{} {}", class.class_name, if native_input_params.is_empty() { "self" } else { "self, " }) } else { "".to_string() }),
            ]);
        }
    }

    render(&mut s, r#"
            }
    "#, []);

    wrappers::generate_optional(&mut s, rust, &DataType::Object(class.class_name.clone()), ctx);

    Module::code(&class.class_name, s)
}

struct Getter {
    instance: bool,
    prop_name: String,
    prop_type: DataType,
}
struct Setter {
    instance: bool,
    prop_name: String,
    prop_type: DataType,
}

fn as_setter(method: &Method) -> Option<Setter> {
    if method.signature.return_ty.is_none() && method.signature.params.iter().filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub)).count() == 1 && method.method_name.starts_with("set_") {
        let prop_name = method.method_name.strip_prefix("set_").unwrap().to_string();
        let prop_type = method.signature.params.first().as_ref().unwrap().ty.clone();
        return Some(Setter {
            instance: method.instance,
            prop_name,
            prop_type,
        });
    }
    None
}
fn as_getter(method: &Method) -> Option<Getter> {
    if method.signature.return_ty.is_some() && !method.signature.params.iter().any(|it| !matches!(&it.ty, DataType::UserScriptGenericStub)) && method.method_name.starts_with("get_") {
        let prop_name = method.method_name.strip_prefix("get_").unwrap().to_string();
        let prop_type = method.signature.return_ty.as_ref().unwrap().clone();
        return Some(Getter {
            instance: method.instance,
            prop_name,
            prop_type,
        });
    }
    None
}

fn generate_property(s: &mut String, class: &EngineClass, getter: Option<Getter>, setter: Option<Setter>, ctx: &GenerationContext) {
    let instance: HashSet<_> = [getter.as_ref().map(|it| it.instance), setter.as_ref().map(|it| it.instance)].into_iter().flatten().collect();
    assert_eq!(instance.len(), 1);
    let instance = instance.into_iter().next().unwrap();

    let prop_type: HashSet<_> = [getter.as_ref().map(|it| &it.prop_type), setter.as_ref().map(|it| &it.prop_type)].into_iter().flatten().collect();
    assert_eq!(prop_type.len(), 1);
    let prop_type = prop_type.into_iter().next().unwrap();

    let prop_name: HashSet<_> = [getter.as_ref().map(|it| &it.prop_name), setter.as_ref().map(|it| &it.prop_name)].into_iter().flatten().collect();
    assert_eq!(prop_name.len(), 1);
    let prop_name = prop_name.into_iter().next().unwrap();

    let ty_marshalling = api_types::type_cs(prop_type);

    render(s, r#"
                public ${static}${property_type} ${property_name}
                {
    "#, [
        ("static", &if !instance { "static " } else { "" }),
        ("property_name", &prop_name.to_case(Case::Pascal)),
        ("property_type", &ty_marshalling.to_facade()),
    ]);


    if let Some(Getter { .. }) = getter.as_ref() {
        render(s, r#"
                    get
                    {
                        unsafe {
                            var __ret = ${rust_path_escaped}_get_${name}(${this});
                            return ${ret_expr};
                        }
                    }
                "#, [
            ("ret_expr", &if ty_marshalling.is_mapped() { format!("{}.ToFacade(__ret)", ty_marshalling.to_blittable()) } else { "__ret".to_string() }),
            ("name", &prop_name),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("this", &if instance { "this" } else { "" }),
        ]);
    }
    if let Some(Setter { .. }) = setter.as_ref() {
        let conversion = if let CsType::Mapped { blittable, .. } = api_types::type_cs(prop_type) {
            render_string(r#"var _value = ${blittable}.FromFacade(value)"#, [("blittable", &blittable)])
        } else {
            render_string(r#"var _value = value"#, [])
        };
        render(s, r#"
                    set
                    {
                        unsafe {
                            ${conversion};
                            ${rust_path_escaped}_set_${name}(${this}${deref}_value);
                        }
                    }
        "#, [
            ("name", &prop_name),
            ("deref", &if ctx.is_struct(&prop_type) { "&" } else { "" }),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("this", &if instance { "this, " } else { "" }),
            ("conversion", &conversion),
        ]);
    }

    render(s, r#"
                }
    "#, []);
}

fn generate_method(
    s: &mut String,
    class: &EngineClass,
    method: &Method,
    ctx: &GenerationContext,
) {
    let has_class_name_arg = method.signature.params.iter().any(|it| matches!(&it.ty, DataType::ClassName));
    let input_params = method.signature.params.iter()
        .filter(|it| !matches!(&it.ty, DataType::ClassName))
        .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
        .map(|param| format!("{} {}", api_types::type_cs(&param.ty).to_facade(), param.name))
        .to_vec();
    let output_params = method.signature.params.iter()
        .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
        .map(|param| match &param.ty {
            DataType::ClassName => format!("typeof(T).Name"),
            ty => if ctx.is_struct(ty) { format!("&_{}", param.name) } else { format!("_{}", param.name) },
        })
        .to_vec();

    let mut converted_params = String::new();
    for param in method.signature.params.iter() {
        if matches!(&param.ty, DataType::ClassName | DataType::UserScriptGenericStub | DataType::Buffer(_)) {
            continue;
        }
        if let CsType::Mapped { blittable, .. } = api_types::type_cs(&param.ty) {
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
        if let DataType::Buffer(buffer_item_ty) = return_ty {
            let buffer_param = method.signature.params.iter()
                .find(|it| matches!(&it.ty, DataType::Buffer(_)))
                .unwrap();
            let buffer_ty_marshalling = api_types::type_cs(buffer_item_ty);
            if buffer_ty_marshalling.is_mapped() {
                panic!("buffer type should be blittable");
            }

            let input_params = method.signature.params.iter()
                .filter(|it| !matches!(&it.ty, DataType::ClassName))
                .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
                .map(|param| match &param.ty {
                    DataType::Buffer(ty) => format!("{}[] {}", api_types::type_cs(ty).to_blittable(), param.name),
                    ty => format!("{} {}", api_types::type_cs(ty).to_facade(), param.name),
                })
                .to_vec();

            let output_params = method.signature.params.iter()
                .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
                .map(|param| match &param.ty {
                    DataType::ClassName => format!("typeof(T).Name"),
                    DataType::Buffer(_) => format!("{}_slice", param.name),
                    ty => if ctx.is_struct(ty) { format!("&_{}", param.name) } else { format!("_{}", param.name) },
                })
                .to_vec();

            render(s, r#"

                public ${static}int ${name}${generics}(${input_params})${generic_where}
                {
                    unsafe
                    {
                    fixed (${buffer_type}* ${buffer_param}_ptr = ${buffer_param})
                    {
                        var ${buffer_param}_slice = new ${buffer_type}_slice(${buffer_param}_ptr, ${buffer_param}.Length);
                        ${converted_params}
                        return ${rust_path_escaped}_${rust_name}(${this}${output_params});
                    }
                    }
                }
                "#, [
                ("buffer_type", &buffer_ty_marshalling.to_facade()),
                ("buffer_param", &buffer_param.name),
                ("static", &if !method.instance { "static " } else { "" }),
                ("generic_where", &if has_class_name_arg { " where T : class" } else { "" }),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("rust_name", &method.method_name),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                ("this", &if method.instance { if output_params.is_empty() { "this" } else { "this, " } } else { "" }),
                ("converted_params", &converted_params.trim_start()),
            ]);
        } else {
            let return_ty = api_types::type_cs(return_ty);
            render(s, r#"

                public ${static}${return_ty} ${name}${generics}(${input_params})${generic_where}
                {
                    unsafe {
                        ${converted_params}
                        var __ret = ${rust_path_escaped}_${rust_name}(${this}${output_params});
                        return ${ret_expr}${generic_cast};
                    }
                }
                "#, [
                ("static", &if !method.instance { "static " } else { "" }),
                ("return_ty", &if has_class_name_arg { return_ty.to_facade_generic() } else { return_ty.to_facade() }),
                ("ret_expr", &if return_ty.is_mapped() { format!("{}.ToFacade(__ret)", return_ty.to_blittable()) } else { "__ret".to_string() }),
                ("generic_cast", &if has_class_name_arg { " as T" } else { "" }),
                ("generic_where", &if has_class_name_arg { " where T : class" } else { "" }),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("rust_name", &method.method_name),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                ("this", &if method.instance { if output_params.is_empty() { "this" } else { "this, " } } else { "" }),
                ("converted_params", &converted_params.trim_start()),
            ]);
        }
    } else {
        render(s, r#"

                public ${static}void ${name}${generics}(${input_params})
                {
                    unsafe {
                        ${converted_params}
                        ${rust_path_escaped}_${rust_name}(${this}${output_params});
                    }
                }
        "#, [
            ("static", &if !method.instance { "static " } else { "" }),
            ("name", &method.method_name.to_case(Case::Pascal)),
            ("rust_name", &method.method_name),
            ("input_params", &input_params.join(", ")),
            ("output_params", &output_params.join(", ")),
            ("rust_path_escaped", &class.rust_struct_path.to_string().replace("::", "_")),
            ("generics", &if has_class_name_arg { "<T>" } else { "" }),
            ("this", &if method.instance { if output_params.is_empty() { "this" } else { "this, " } } else { "" }),
            ("converted_params", &converted_params.trim_start()),
        ]);
    }
}