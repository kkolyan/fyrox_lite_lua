use std::{collections::HashMap, fs, io::BufRead, path::Path};

use convert_case::Casing;
use fyrox::core::Uuid;
use fyrox_lite::script_metadata::{ScriptField, ScriptFieldValueType, ScriptKind, ScriptMetadata};

pub fn parse_file(path: impl AsRef<Path>) -> Result<ScriptMetadata, Vec<String>> {
    let script_source = fs::read(&path).unwrap();
    let path = path.as_ref();
    let file_name = match path.file_name() {
        None => {
            return Err(vec![format!(
                "failed to get name of file {}",
                &path.to_string_lossy()
            )])
        }
        Some(it) => it.to_string_lossy(),
    };
    parse_source(script_source).and_then(|it| {
        if !file_name.starts_with(format!("{}.", it.class).as_str()) {
            Err(vec![format!(
                "file name should match class name. class: {}",
                it.class
            )])
        } else {
            Ok(it)
        }
    })
}

/*
Example:
---@uuid 9596c7ea-8751-423d-839a-5f6c8364223c
---@class Bullet: Script
---@field velocity Vector3
---@field damage number
---@field ttl_sec number
---@field owner Node
     */
pub(crate) fn parse_source(script_source: Vec<u8>) -> Result<ScriptMetadata, Vec<String>> {
    let mut uuid = None;
    let mut class = None;
    let mut parent_class = None;
    let mut fields = Vec::new();
    let mut errors = Vec::new();
    for line in script_source.lines() {
        match line {
            Ok(line) => {
                let annotation_prefix = "---@";
                if !line.starts_with(annotation_prefix) {
                    break;
                }
                let annotation = &line[annotation_prefix.len()..];
                if let Some((tag, value)) = annotation.split_once(" ") {
                    match tag {
                        "uuid" => {
                            if uuid.is_some() {
                                errors.push("duplicated uuid tag".to_string());
                            }
                            match Uuid::parse_str(value) {
                                Ok(value) => {
                                    uuid = Some(value);
                                }
                                Err(err) => {
                                    errors.push(format!(
                                        "failed to parse UUID from text {:?} due to {}",
                                        value, err
                                    ));
                                }
                            }
                        }
                        "class" => {
                            if class.is_some() {
                                errors.push("duplicated class tag".to_string());
                            }
                            let mut parts = value.splitn(2, ":");
                            class = Some(parts.next().unwrap().trim().to_string().leak());
                            parent_class = parts.next().map(|it| it.trim().to_string());
                        }
                        "field" => {
                            let mut parts = value.splitn(3, " ");
                            let name = parts.next().unwrap().to_string();
                            let private = name == "private";
                            let name = if private {
                                parts.next().unwrap().to_string()
                            } else {
                                name
                            };
                            match parts.next() {
                                None => errors.push(format!(
                                    "failed to parse field type: {}",
                                    &annotation
                                )),
                                Some(it) => {
                                    let ty = match it {
                                        "number" => Some(ScriptFieldValueType::f64),
                                        "string" => Some(ScriptFieldValueType::String),
                                        "boolean" => Some(ScriptFieldValueType::bool),
                                        "Node" => Some(ScriptFieldValueType::Node),
                                        "UiNode" => Some(ScriptFieldValueType::UiNode),
                                        "Prefab" => Some(ScriptFieldValueType::Prefab),
                                        "Vector3" => Some(ScriptFieldValueType::Vector3),
                                        "Quaternion" => Some(ScriptFieldValueType::Quaternion),
                                        _ => Some(ScriptFieldValueType::RuntimePin),
                                    };
                                    if let Some(ty) = ty {
                                        let description = parts.next().map(|it| {
                                            let s: &'static str = it.to_string().leak();
                                            s
                                        });
                                        let title = name.to_case(convert_case::Case::Title);
                                        fields.push(ScriptField {
                                            name,
                                            ty,
                                            description,
                                            private,
                                            title,
                                        });
                                    }
                                }
                            };
                        }
                        unknown_tag => {
                            errors.push(format!("unknown tag: {}", unknown_tag));
                        }
                    }
                    continue;
                }
                errors.push(format!("invalid tag: {}", annotation));
            }
            Err(err) => {
                errors.push(format!("failed to extract lines of text: {}", err));
            }
        }
    }
    if class.is_none() {
        errors.push("class tag is missing".to_string());
    }

    let parent_class = parent_class.as_deref();

    if parent_class.is_none() || !(parent_class.unwrap() == "Script" || parent_class.unwrap() == "Plugin") {
        errors.push("parent class is required to be either Script or Plugin".to_string());
    };
    if parent_class.is_some() && parent_class.unwrap() == "Script" && uuid.is_none() {
        errors.push("uuid tag is required for class extending Script".to_string());
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let field_name_to_index = fields
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();
    let kind =  match parent_class.unwrap() {
        "Script" => ScriptKind::Script(uuid.unwrap()),
        "Plugin" => ScriptKind::Plugin,
        unknown => panic!("unknown ScriptKind constant: {:?}", unknown),
    };
    Ok(ScriptMetadata {
        class: class.unwrap().to_string(),
        kind,
        fields,
        field_name_to_index,
    })
}