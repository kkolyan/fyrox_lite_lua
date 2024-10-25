use std::collections::{HashMap, HashSet};

use gen_common::templating::render;
use lite_model::{Class, ClassName, DataType, Domain, EngineClass, EnumClass, EnumValue, EnumVariant, Field};

use super::types::generate_ffi_type;

pub(crate) fn generate_enum(
    s: &mut String,
    class: &EnumClass,
    client_replicated_types: &HashSet<ClassName>,
) {

    let has_fields = class.variants.iter().any(|it| !matches!(it.value, EnumValue::Unit));

    if has_fields {
        render(
            s,
            r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Native${class} {
                    pub tag: u8,
                pub value: Native${class}VariantContainer,
            }
        "#,
            [("class", &class.class_name)],
        );
    } else {

        render(
            s,
            r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Native${class} {
                pub tag: u8,
            }
        "#,
            [("class", &class.class_name)],
        );
    }

    render(s, r#"
            native_utils!(Native${class}, Native${class}_array, Native${class}_option, Native${class}_result);
    "#, [("class", &class.class_name)]);

    generate_tag_constants_impl(s, class);

    if has_fields {
        render(
            s,
            r#"
    
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub union Native${class}VariantContainer {
        "#,
            [("class", &class.class_name)],
        );
    
        generate_variant_container_fields(s, class);
    
        render(
            s,
            r#"
            }
        "#,
            [],
        );
    }

    generate_variants(s, class, client_replicated_types);
}

fn generate_tag_constants_impl(s: &mut String, class: &EnumClass) {
    render(
        s,
        r#"

            impl Native${class} {
    "#,
        [("class", &class.class_name)],
    );

    generate_tag_constants(s, class);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_tag_constants(s: &mut String, class: &EnumClass) {
    for (i, variant) in class.variants.iter().enumerate() {
        render(
            s,
            r#"
                pub const ${name}: u8 = ${i};
        "#,
            [("name", &variant.tag), ("i", &i)],
        );
    }
}

fn generate_variants(
    s: &mut String,
    class: &EnumClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    for variant in class.variants.iter() {
        if matches!(variant.value, EnumValue::Unit) {
            continue;
        }
        render(
            s,
            r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Native${class}_${name} {
        "#,
            [("name", &variant.tag), ("class", &class.class_name)],
        );
        match &variant.value {
            EnumValue::Unit => {}
            EnumValue::Tuple { fields } => {
                generate_tuple_variant_fields(s, fields, client_replicated_types);
            }
            EnumValue::Struct { fields } => {
                generate_struct_variant_fields(s, fields, client_replicated_types);
            }
        }
        render(
            s,
            r#"
            }
        "#,
            [],
        );
    }
}

fn generate_tuple_variant_fields(
    s: &mut String,
    fields: &[DataType],
    client_replicated_types: &HashSet<ClassName>,
) {
    for (i, field) in fields.iter().enumerate() {
        render(
            s,
            r#"
                pub _${i}: ${type},
        "#,
            [
                ("i", &i),
                ("type", &generate_ffi_type(field, client_replicated_types)),
            ],
        );
    }
}
fn generate_struct_variant_fields(
    s: &mut String,
    fields: &[Field],
    client_replicated_types: &HashSet<ClassName>,
) {
    for (i, field) in fields.iter().enumerate() {
        render(
            s,
            r#"
                pub ${name}: ${type},
        "#,
            [
                ("name", &field.name),
                (
                    "type",
                    &generate_ffi_type(&field.ty, client_replicated_types),
                ),
            ],
        );
    }
}

fn generate_variant_container_fields(s: &mut String, class: &EnumClass) {
    for variant in class.variants.iter() {
        if matches!(variant.value, EnumValue::Unit) {
            continue;
        }
        render(
            s,
            r#"
                pub ${name}: Native${class}_${name},
        "#,
            [("name", &variant.tag), ("class", &class.class_name)],
        );
    }
}
