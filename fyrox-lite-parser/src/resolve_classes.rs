use std::{collections::HashMap, ops::Deref};

use fyrox_lite_model::{DataType, Domain};

use crate::RustSymbol;

pub fn resolve_classes(domain: &mut Domain, aliases: &mut HashMap<String, RustSymbol>) {
    let mut classes: HashMap<RustSymbol, DataType> = Default::default();

    for item in domain.items.iter() {
        match item {
            fyrox_lite_model::DomainItem::EngineClass(class) => {
                classes.insert(
                    aliases[&class.class_name.0].clone(),
                    DataType::EngineObject(class.class_name.clone()),
                );
            }
            fyrox_lite_model::DomainItem::StructClass(class) => {
                classes.insert(
                    aliases[&class.class_name.0].clone(),
                    DataType::Struct(class.class_name.clone()),
                );
            }
            fyrox_lite_model::DomainItem::EnumClass(class) => {
                classes.insert(
                    aliases[&class.class_name.0].clone(),
                    DataType::Enum(class.class_name.clone()),
                );
            }
        }
    }

    for item in domain.items.iter_mut() {
        match item {
            fyrox_lite_model::DomainItem::EngineClass(class) => {
                resolve_classes_engine_class(class, &classes)
            }
            fyrox_lite_model::DomainItem::StructClass(class) => {
                resolve_classes_struct_class(class, &classes)
            }
            fyrox_lite_model::DomainItem::EnumClass(class) => {
                resolve_classes_enum_class(class, &classes)
            }
        }
    }
}

fn resolve_type(ty: &DataType, classes: &HashMap<RustSymbol, DataType>) -> DataType {
    match ty {
        DataType::UnresolvedClass(ref it) => match classes.get(&RustSymbol(it.clone())) {
            Some(it) => it.clone(),
            None => {
                println!("ERROR: failed to resolve {}", it);
                ty.clone()
            }
        },
        DataType::Vec(it) => DataType::Vec(Box::new(resolve_type(it.deref(), classes))),
        DataType::Option(it) => DataType::Option(Box::new(resolve_type(it.deref(), classes))),
        DataType::Result { ok: it } => DataType::Result {
            ok: Box::new(resolve_type(it.deref(), classes)),
        },
        it => it.clone(),
    }
}

fn resolve_classes_enum_class(
    class: &mut fyrox_lite_model::EnumClass,
    classes: &HashMap<RustSymbol, DataType>,
) {
    for variant in class.variants.iter_mut() {
        match &mut variant.value {
            fyrox_lite_model::EnumValue::Unit => {}
            fyrox_lite_model::EnumValue::Tuple { fields } => {
                for field in fields.iter_mut() {
                    *field = resolve_type(field, classes);
                }
            }
            fyrox_lite_model::EnumValue::Struct { fields } => {
                for field in fields.iter_mut() {
                    field.ty = resolve_type(&field.ty, classes);
                }
            }
        }
    }
}

fn resolve_classes_struct_class(
    class: &mut fyrox_lite_model::StructClass,
    classes: &HashMap<RustSymbol, DataType>,
) {
    for field in class.fields.iter_mut() {
        field.ty = resolve_type(&field.ty, classes);
    }
}

fn resolve_classes_engine_class(
    class: &mut fyrox_lite_model::EngineClass,
    classes: &HashMap<RustSymbol, DataType>,
) {
    for method in class.methods.iter_mut() {
        for param in method.signature.params.iter_mut() {
            *param = resolve_type(param, classes);
        }
        if let Some(it) = method.signature.return_ty.as_mut() {
            *it = resolve_type(it, classes);
        }
    }
}