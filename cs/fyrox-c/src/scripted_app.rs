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

thread_local! {
    pub static APP: RefCell<Option<ScriptedApp>> = Default::default();
}

pub struct CScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_on_init: bool,
    pub has_on_start: bool,
    pub has_on_deinit: bool,
    pub has_on_os_event: bool,
    pub has_on_update: bool,
    pub has_on_message: bool,
}

pub struct ScriptedApp {
    pub scripts: HashMap<Uuid, CScriptMetadata>,
    pub functions: NativeScriptAppFunctions,
}

impl ScriptedApp {
    pub fn from_native(app: NativeScriptedApp) -> Self {
        ScriptedApp {
            scripts: (0..app.scripts_len)
                .map(|i| {
                    let native_class = &unsafe { *app.scripts.add(i as usize) };
                    let script = extract_for_def(native_class);
                    Some((
                        match script.kind {
                            ScriptKind::Script(uuid) => uuid,
                            ScriptKind::Plugin => return None,
                        },
                        CScriptMetadata {
                            id: native_class.id,
                            md: script,
                            has_on_init: true,
                            has_on_start: true,
                            has_on_deinit: true,
                            has_on_os_event: true,
                            has_on_update: true,
                            has_on_message: true,
                        },
                    ))
                })
                .flatten()
                .collect(),
            functions: app.functions,
        }
    }
}

pub fn extract_for_def(md: &NativeScriptMetadata) -> ScriptMetadata {
    let name = unsafe { CStr::from_ptr(md.name) };
    let fields = (0..md.properties_len)
        .map(|it| {
            let property = unsafe { *md.properties.add(it as usize) };
            let name = unsafe { CStr::from_ptr(property.name) }
                .to_str()
                .unwrap()
                .to_string();
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
                    NativeValueType::Node => ScriptFieldValueType::Node,
                    NativeValueType::UiNode => ScriptFieldValueType::UiNode,
                    NativeValueType::Prefab => ScriptFieldValueType::Prefab,
                    NativeValueType::Vector3 => ScriptFieldValueType::Vector3,
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
        class: name.to_str().unwrap().to_string(),
        kind: match md.kind {
            NativeScriptKind::Node => ScriptKind::Script(
                Uuid::parse_str(unsafe { CStr::from_ptr(md.uuid) }.to_str().unwrap()).unwrap(),
            ),
            NativeScriptKind::Global => ScriptKind::Plugin,
        },
        fields,
        field_name_to_index,
    }
}
