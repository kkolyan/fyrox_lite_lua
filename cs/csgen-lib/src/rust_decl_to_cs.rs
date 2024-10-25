use std::collections::{HashMap, HashSet};

use gen_common::templating::render;
use quote::ToTokens;
use syn::{Attribute, Expr, File, FnArg, Item, ReturnType, Type};
use to_vec::ToVec;

struct CustomTypeProps {
    is_delegate: bool
}

pub fn rust_decl_to_c(file: &File) -> String {
    let mut custom_type_names: HashMap<String, CustomTypeProps> = Default::default();
    collect_custom_type_names(file, &mut custom_type_names);
    convert_file(file, &custom_type_names)
}

fn collect_custom_type_names(file: &File, custom_type_names: &mut HashMap<String, CustomTypeProps> ) {
    for item in file.items.iter() {
        if let Item::Struct(item) = item {
            custom_type_names.insert(item.ident.to_string(), CustomTypeProps { is_delegate: false });
        }
        if let Item::Union(item) = item {
            custom_type_names.insert(item.ident.to_string(), CustomTypeProps { is_delegate: false });
        }
        if let Item::Enum(item) = item {
            custom_type_names.insert(item.ident.to_string(), CustomTypeProps { is_delegate: false });
        }
        if let Item::Type(item) = item {
            custom_type_names.insert(item.ident.to_string(), CustomTypeProps { is_delegate: true });
        }
    }
}

fn convert_file(file: &File, custom_type_names: &HashMap<String, CustomTypeProps>) -> String {
    let mut s = String::new();
    for item in file.items.iter() {
        if let Item::Struct(item) = item {
            convert_struct(&mut s, item, custom_type_names);
        }
        if let Item::Union(item) = item {
            convert_union(&mut s, item, custom_type_names);
        }
        if let Item::Enum(item) = item {
            convert_enum(&mut s, item, custom_type_names);
        }
        if let Item::Type(item) = item {
            convert_functor_def(&mut s, item, custom_type_names);
        }
        if let Item::Fn(item) = item {
            convert_function(&mut s, item, custom_type_names);
        }
    }
    s
}

pub struct OopDecl {
    pub owner_name: String,
    pub code: String,
}

fn extract_owner_class(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .map(|it| match &it.meta {
            syn::Meta::Path(_) => None,
            syn::Meta::List(list) => {
                if list.path.get_ident().unwrap() == "doc" {
                    Some(list.tokens.to_string())
                } else {
                    None
                }
            }
            syn::Meta::NameValue(_) => None,
        })
        .flatten()
        .map(|it| it.strip_prefix("@owner_class ").map(|it| it.to_string()))
        .flatten()
        .next()
        // .expect("owner class anotation required by the function: `///@owner_class MyClass`")
        .unwrap_or_default()
}

fn convert_function(s: &mut String, item: &syn::ItemFn, custom_type_names: &HashMap<String, CustomTypeProps> ) {
    let ret = match &item.sig.output {
        ReturnType::Default => "void".to_string(),
        ReturnType::Type(_, ty) => type_rs2cs(ty.as_ref(), custom_type_names),
    };
    let name = item.sig.ident.to_string();
    let args = item
        .sig
        .inputs
        .iter()
        .map(|arg| {
            let FnArg::Typed(it) = arg else {
                panic!("WTF? receiver?");
            };
            let ident = match it.pat.as_ref() {
                syn::Pat::Ident(it) => &it.ident,
                it => panic!(
                    "only plain ident arg names are allowed. {:?}",
                    it.to_token_stream()
                ),
            };
            format!("{} {}", type_rs2cs(&it.ty, custom_type_names), &ident)
        })
        .to_vec();
    render(
        s,
        r#"
                [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                public static partial ${fn};
        "#,
        [("fn", &format!("{} {}({})", ret, name, &args.join(", ")))],
    );
}

fn convert_functor_def(s: &mut String, item: &syn::ItemType, custom_type_names: &HashMap<String, CustomTypeProps> ) {
    if let Type::BareFn(f) = item.ty.as_ref() {
        let ret = match &f.output {
            ReturnType::Default => "void".to_string(),
            ReturnType::Type(_, ty) => type_rs2cs(ty.as_ref(), custom_type_names),
        };
        let name = item.ident.to_string();
        let args = f
            .inputs
            .iter()
            .map(|arg| {
                format!(
                    "{} {}",
                    type_rs2cs(&arg.ty, custom_type_names),
                    match arg.name.as_ref() {
                        Some(it) => it.0.to_string(),
                        None => panic!("arg name missed: {}", item.to_token_stream()),
                    }
                )
            })
            .to_vec();

        render(
            s,
            r#"
                public delegate ${fn};
            "#,
            [("fn", &format!("{} {}({})", ret, name, &args.join(", ")))],
        );
    }
}

fn convert_enum(s: &mut String, item: &syn::ItemEnum, _custom_type_names: &HashMap<String, CustomTypeProps> ) {
    if item
        .attrs
        .iter()
        .any(|it| it.to_token_stream().to_string() == "# [repr (C)]")
    {
        let name = item.ident.to_string();

        render(
            s,
            r#"
                public enum ${name} {
            "#,
            [("name", &name)],
        );

        for variant in item.variants.iter() {
            render(
                s,
                "
                    ${name},
                ",
                [("name", &escape_keywords(&variant.ident.to_string()))],
            );
        }

        render(
            s,
            "
                }
            ",
            [],
        );
    }
}

fn escape_keywords(s: &str) -> &str {
    if s == "bool" {
        return "@bool";
    }
    s
}

fn convert_struct(s: &mut String, item: &syn::ItemStruct, custom_type_names: &HashMap<String, CustomTypeProps> ) {
    if item
        .attrs
        .iter()
        .any(|it| it.to_token_stream().to_string() == "# [repr (C)]")
    {
        let name = item.ident.to_string();
        render(
            s,
            r#"
                [StructLayout(LayoutKind.Sequential)]
                public unsafe struct ${name} {
            "#,
            [("name", &name)],
        );

        for field in item.fields.iter() {
            render(
                s,
                "
                    public ${type} ${name};
                ",
                [
                    ("name", field.ident.as_ref().unwrap()),
                    ("type", &type_rs2cs(&field.ty, &custom_type_names)),
                ],
            );
        }

        render(
            s,
            "
                }
            ",
            [],
        );
    }
}

fn convert_union(s: &mut String, item: &syn::ItemUnion, custom_type_names: &HashMap<String, CustomTypeProps> ) {
    if item
        .attrs
        .iter()
        .any(|it| it.to_token_stream().to_string() == "# [repr (C)]")
    {
        let name = item.ident.to_string();
        render(
            s,
            r#"
                [StructLayout(LayoutKind.Explicit)]
                public unsafe struct ${name} {
            "#,
            [("name", &name)],
        );

        for field in item.fields.named.iter() {
            render(
                s,
                "

                    [FieldOffset(0)]
                    public ${type} ${name};
                    ",
                [
                    ("name", &escape_keywords(field.ident.as_ref().unwrap().to_string().as_str())),
                    ("type", &type_rs2cs(&field.ty, &custom_type_names)),
                ],
            );
        }

        render(
            s,
            "
                }
            ",
            [],
        );
    }
}

#[rustfmt::skip]
fn type_rs2cs(ty: &Type, custom_type_names: &HashMap<String, CustomTypeProps> ) -> String {
    let s = ty.to_token_stream().to_string();
    if let Type::Tuple(ty) = ty {
        if ty.elems.is_empty() {
            return "void".to_owned();
        }
    }
    if let Type::Path(ty) = ty {
        if let Some(ident) = ty.path.get_ident() {
            let ident = ident.to_string();
            if ident == "bool" { return "bool".to_owned(); }
            if ident == "u8" { return "ubyte".to_owned(); }
            if ident == "u16" { return "ushort".to_owned(); }
            if ident == "u32" { return "uint".to_owned(); }
            if ident == "u64" { return "ulong".to_owned(); }
            if ident == "f32" { return "float".to_owned(); }
            if ident == "f64" { return "double".to_owned(); }
            if ident == "i16" { return "short".to_owned(); }
            if ident == "i32" { return "int".to_owned(); }
            if ident == "i64" { return "long".to_owned(); }
            if ident == "NativeString" { return "NativeString".to_owned(); }
            if ident == "NativeHandle" { return "NativeHandle".to_owned(); }
            if ident == "NativeVector3" { return "NativeVector3".to_owned(); }
            if ident == "NativeQuaternion" { return "NativeQuaternion".to_owned(); }
            if let Some(it) = custom_type_names.get(&ident) {
                if it.is_delegate {
                    return format!("IntPtr");
                }
                return ident;
            }
        }
    }
    if let Type::Ptr(ty) = ty {
        if let Type::Path(st) = ty.elem.as_ref() {
            if let Some(ident) = st.path.get_ident() {
                let ident = ident.to_string();
                if custom_type_names.get(&ident).is_some() {
                    return format!("{}*", ident);
                }
                if ident == "c_char" {
                    return "byte*".to_string();
                }
            }
        }
    }
    if custom_type_names.contains_key(&s) {
        return s;
    }
    panic!("unsupported type: {} ({:?})", s, ty)
}
