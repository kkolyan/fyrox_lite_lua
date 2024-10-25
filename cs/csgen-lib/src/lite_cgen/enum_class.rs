use std::collections::{HashMap, HashSet};

use gen_common::templating::render;
use lite_model::{
    Class, ClassName, DataType, Domain, EngineClass, EnumClass, EnumValue, EnumVariant, Field,
};

use super::{
    simple_from,
    types::{self, generate_ffi_type},
};

pub(crate) fn generate_enum(
    s: &mut String,
    class: &EnumClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    let has_fields = class
        .variants
        .iter()
        .any(|it| !matches!(it.value, EnumValue::Unit));

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

    render(
        s,
        r#"
            native_utils!(Native${class}, Native${class}_array, Native${class}_option, Native${class}_result);
    "#,
        [("class", &class.class_name)],
    );

    generate_from_native(s, class, client_replicated_types);

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

fn generate_from_native(
    s: &mut String,
    class: &EnumClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    render(
        s,
        r#"
            impl From<Native${class}> for ${rust_class} {
                fn from(__value: Native${class}) -> Self {
    "#,
        [
            ("rust_class", &class.rust_struct_path),
            ("class", &class.class_name),
        ],
    );

    for variant in class.variants.iter() {
        render(
            s,
            r#"
                    if __value.tag == Native${class}::${variant} {
        "#,
            [("class", &class.class_name), ("variant", &variant.tag)],
        );

        match &variant.value {
            EnumValue::Unit => {
                render(
                    s,
                    r#"
                        return Self::${variant};
                    }
                "#,
                    [("variant", &variant.tag)],
                );
            }
            EnumValue::Tuple { fields } => {
                let mut output_fields = vec![];
                for (i, ty) in fields.iter().enumerate() {
                    let var = format!("_{}", i);
                    output_fields.push(var.clone());

                    render(
                        s,
                        r#"
                        let ${var} = unsafe { __value.value.${variant}.${var} };
                        let ${var} = ${expr};
                    "#,
                        [
                            ("var", &var),
                            ("variant", &variant.tag),
                            (
                                "expr",
                                &types::generate_from_native(ty, &var, client_replicated_types),
                            ),
                        ],
                    );
                }
                render(
                    s,
                    r#"
                        return Self::${variant}( ${output_fields} );
                    }
                "#,
                    [
                        ("variant", &variant.tag),
                        ("output_fields", &output_fields.join(", ")),
                    ],
                );
            }
            EnumValue::Struct { fields } => {
                let mut output_fields = vec![];
                for field in fields.iter() {
                    let var = &field.name;
                    let ty = &field.ty;
                    output_fields.push(var.clone());

                    render(
                        s,
                        r#"
                        let ${var} = unsafe { __value.value.${variant}.${var} };
                        let ${var} = ${expr};
                    "#,
                        [
                            ("var", &var),
                            ("variant", &variant.tag),
                            (
                                "expr",
                                &types::generate_from_native(ty, &var, client_replicated_types),
                            ),
                        ],
                    );
                }
                render(
                    s,
                    r#"
                        return Self::${variant} { ${output_fields} };
                    }
                "#,
                    [
                        ("variant", &variant.tag),
                        ("output_fields", &output_fields.join(", ")),
                    ],
                );
            }
        }
    }

    render(
        s,
        r#"
                    panic!("unsupported enum tag: NativeBrush::{}", __value.tag)
                }
            }
    "#,
        [],
    );
}
