use std::{collections::HashMap, fs, io::BufRead, path::Path};

use fyrox::core::Uuid;

#[derive(Debug)]
pub struct ScriptDefinition {
    pub metadata: ScriptMetadata,
    pub assembly_name: &'static str,
}

#[derive(Debug)]
pub struct ScriptMetadata {
    pub uuid: Uuid,
    pub class: &'static str,
    pub parent_class: Option<String>,
    pub fields: Vec<ScriptField>,
    pub field_name_to_index: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct ScriptField {
    pub name: String,
    pub ty: ScriptFieldValueType,
    pub description: Option<&'static str>,
}

#[derive(Copy, Clone, Debug)]
pub enum ScriptFieldValueType {
    Number,
    String,
    Bool,
    Node,
    Vector3,
}

impl ScriptMetadata {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, Vec<String>> {
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
        Self::parse_source(script_source).and_then(|it| {
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
    pub fn parse_source(script_source: Vec<u8>) -> Result<ScriptMetadata, Vec<String>> {
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
                                    errors.push(format!("duplicated uuid tag"));
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
                                    errors.push(format!("duplicated class tag"));
                                }
                                let mut parts = value.splitn(2, ":");
                                class = Some(parts.next().unwrap().trim().to_string().leak());
                                parent_class = parts.next().map(|it| it.trim().to_string());
                            }
                            "field" => {
                                let mut parts = value.splitn(3, " ");
                                let name = parts.next().unwrap().to_string();
                                match parts.next() {
                                    None => errors.push(format!(
                                        "failed to parse field type: {}",
                                        &annotation
                                    )),
                                    Some(it) => {
                                        let ty = match it {
                                            "number" => Some(ScriptFieldValueType::Number),
                                            "string" => Some(ScriptFieldValueType::String),
                                            "bool" => Some(ScriptFieldValueType::Bool),
                                            "Node" => Some(ScriptFieldValueType::Node),
                                            "Vector3" => Some(ScriptFieldValueType::Vector3),
                                            unsupported => {
                                                errors.push(format!(
                                                    "type not supported: {}",
                                                    unsupported
                                                ));
                                                None
                                            }
                                        };
                                        if let Some(ty) = ty {
                                            let description = parts.next().map(|it| {
                                                let s: &'static str = it.to_string().leak();
                                                s
                                            });
                                            fields.push(ScriptField {
                                                name,
                                                ty,
                                                description,
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
        if uuid.is_none() {
            errors.push("uuid tag is missing".to_string());
        }
        if class.is_none() {
            errors.push("class tag is missing".to_string());
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let field_name_to_index = fields
            .iter()
            .enumerate()
            .map(|(i, v)| (v.name.clone(), i))
            .collect();
        Ok(ScriptMetadata {
            uuid: uuid.unwrap(),
            class: class.unwrap(),
            parent_class,
            fields,
            field_name_to_index,
        })
    }
}
