use std::{cell::RefCell, collections::HashMap, ffi::CStr};

use convert_case::Casing;
use fyrox::core::Uuid;
use fyrox_lite::script_metadata::{
    ScriptField, ScriptFieldValueType, ScriptKind, ScriptMetadata,
};
use to_vec::ToVec;

use crate::bindings_manual::{
    NativeClassId, NativeScriptAppFunctions, NativeScriptKind, NativeScriptMetadata,
    NativeScriptedApp, NativeValueType,
};

// TODO replace with SendWrapper
thread_local! {
    pub static APP: RefCell<Option<ScriptedApp>> = Default::default();
}

pub struct CScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_on_init: bool,
    pub has_on_start: bool,
    pub has_on_deinit: bool,
    pub has_on_update: bool,
    pub has_on_message: bool,
}

pub struct ScriptedApp {
    pub scripts: HashMap<Uuid, CScriptMetadata>,
    pub uuid_by_class: HashMap<NativeClassId, Uuid>,
    pub functions: NativeScriptAppFunctions,
}

impl ScriptedApp {
    pub fn from_native(app: NativeScriptedApp) -> Self {
        let scripts: Vec<_> = app.scripts.into();
        let scripts: HashMap<Uuid, CScriptMetadata> = scripts.into_iter()
            .map(|native_class| {
                let uuid = Uuid::parse_str(String::from(native_class.uuid).as_str()).unwrap();
                let script = extract_for_def(&native_class);
                (
                    uuid,
                    CScriptMetadata {
                        id: native_class.id,
                        md: script,
                        has_on_init: true,
                        has_on_start: true,
                        has_on_deinit: true,
                        has_on_update: true,
                        has_on_message: true,
                    },
                )
            })
            .collect();
        let uuid_by_class = scripts.iter()
            .map(|(uuid, md)| (md.id, *uuid))
            .collect();
        ScriptedApp {
            uuid_by_class,
            scripts,
            functions: app.functions,
        }
    }
}

pub fn extract_for_def(md: &NativeScriptMetadata) -> ScriptMetadata {
    let properties: Vec<_> = md.properties.into();
    let class: String = md.name.into();
    let uuid: String = md.uuid.into();
    let fields = properties
        .into_iter()
        .map(|property| {
            let name: String = property.name.into();
            let title = name.to_case(convert_case::Case::Title);
            ScriptField {
                name,
                title,
                ty: match property.ty {
                    NativeValueType::bool => ScriptFieldValueType::bool,
                    NativeValueType::f32 => ScriptFieldValueType::f32,
                    NativeValueType::f64 => ScriptFieldValueType::f64,
                    NativeValueType::i16 => ScriptFieldValueType::i16,
                    NativeValueType::i32 => ScriptFieldValueType::i32,
                    NativeValueType::i64 => ScriptFieldValueType::i64,
                    NativeValueType::String => ScriptFieldValueType::String,
                    NativeValueType::Handle => ScriptFieldValueType::Node,
                    NativeValueType::Prefab => ScriptFieldValueType::Prefab,
                    NativeValueType::Vector3 => ScriptFieldValueType::Vector3,
                    NativeValueType::Vector2 => ScriptFieldValueType::Vector2,
                    NativeValueType::Vector2I => ScriptFieldValueType::Vector2I,
                    NativeValueType::Quaternion => ScriptFieldValueType::Quaternion,
                },
                description: None,
                private: false,
            }
        })
        .to_vec();
    let field_name_to_index = fields
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();
    ScriptMetadata {
        class,
        kind: match md.kind {
            NativeScriptKind::Node => ScriptKind::Node(
                Uuid::parse_str(&uuid).unwrap(),
            ),
            NativeScriptKind::Global => ScriptKind::Global,
        },
        fields,
        field_name_to_index,
    }
}
