
use convert_case::{Case, Casing};
use lite_model::EngineClass;
use std::borrow::Cow;
use to_vec::ToVec;

use crate::{
    code_model::{Mod, ModContent}, context::GenerationContext, eq::generate_eq, expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua}, generate_methods::{generate_methods, is_getter, is_setter}, supress_lint::SUPRESSIONS, templating::render
};

pub fn generate_engine_class_bindings(class: &EngineClass, ctx: &GenerationContext) -> Mod {
    let mut s: String = Default::default();
    s.push_str(SUPRESSIONS);

    render(
        &mut s,
        r#"

        use fyrox_lite::*;
        use fyrox_lite_math::*;
        use mlua;

        use crate::{
            fyrox_lite_class::{FyroxUserData, Traitor, UserDataClass},
            script_object::ScriptObject,
            typed_userdata::TypedUserData,
        };

        impl FyroxUserData for ${rust_struct_path} {
            const CLASS_NAME: &'static str = "${class_name}";
        "#,
        [
            ("class_name", &class.class_name.0),
            ("rust_struct_path", &class.rust_struct_path.0),
        ],
    );

    generate_instance_methods(&mut s, class, ctx);
    generate_class_methods(&mut s, class, ctx);
    generate_instance_fields(&mut s, class, ctx);
    generate_class_fields(&mut s, class, ctx);

    s += "
        }
    ";

    Mod {
        name: class.class_name.0.to_case(Case::Snake),
        content: ModContent::Code(s),
    }
}

fn generate_class_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
    "#,
        [],
    );
    generate_methods(s, class, ctx, false);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_instance_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            
            fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
                methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
                    Ok(format!("{:?}", this.inner()))
                });
        "#,
        [],
    );
    generate_eq(s, &class.features);
    generate_methods(s, class, ctx, true);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_instance_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
    "#,
        [],
    );

    generate_instance_getters(s, class, ctx);

    generate_setters(s, class, ctx, true);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_setters(s: &mut String, class: &EngineClass, ctx: &GenerationContext, instance: bool) {
    let writable_props = class
        .methods
        .iter()
        .filter(|it| it.instance == instance)
        .filter(|it| is_setter(it))
        .map(|method| (method.method_name.strip_prefix("set_").unwrap(), method))
        .to_vec();

    for (prop, setter) in writable_props {
        let target = match instance {
            true => Cow::Borrowed("this."),
            false => Cow::Owned(format!("{}::", class.rust_struct_path.0)),
        };
        render(
            s,
            r#"
                fields.add_field_method_set("${field_name}", |lua, this, value: ${input_type}| {
                    ${target}set_${field_name}(${expression});
                    Ok(())
                });
        "#,
            [
                ("field_name", &prop),
                (
                    "input_type",
                    &type_to_mlua(&setter.signature.params[0].ty, ctx),
                ),
                (
                    "expression",
                    &mlua_to_rust_expr("value", &setter.signature.params[0].ty, ctx),
                ),
                (
                    "target",
                    match instance {
                        true => &"this.",
                        false => &target,
                    },
                ),
            ],
        );
    }
}

fn generate_instance_getters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    let readable_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_props {
        render(
            s,
            r#"
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = this.get_${field_name}();
                    Ok(${expression})
                });
        "#,
            [
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }
}

fn generate_class_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
    "#,
        [],
    );

    generate_class_getters(s, class, ctx);
    generate_setters(s, class, ctx, false);

    *s += "
            }
    ";
}

fn generate_class_getters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    for constant in class.constants.iter() {
        render(
            s,
            r#"                
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = ${rust_struct_path}::${field_name};
                    Ok(${expression})
                });
        "#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &constant.const_name),
                ("expression", &rust_expr_to_mlua(ctx, "value", &constant.ty)),
            ],
        );
    }

    let readable_static_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| !it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_static_props {
        render(
            s,
            r#"                
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = ${rust_struct_path}::get_${field_name}();
                    Ok(${expression})
                });
        "#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }
}
