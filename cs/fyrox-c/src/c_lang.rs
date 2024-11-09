use std::{cell::RefCell, collections::HashMap, ffi::CString};
use std::sync::Arc;
use fyrox::{
    core::{
        pool::{Handle, Pool},
        visitor::Visit,
        Uuid,
    },
};
use fyrox_lite::{
    externalizable::Externalizable, lite_prefab::LitePrefab, script_metadata::ScriptFieldValueType, script_object::{Lang, ScriptFieldValue, ScriptObject}, script_object_residence::uuid_of_script
};

use crate::{bindings_manual::{
    NativeHandle, NativeValue,
}, scripted_app::APP, UserScriptImpl};
use crate::bindings_lite_2::{NativePrefab, NativeQuaternion, NativeVector2, NativeVector2I, NativeVector3};
use crate::bindings_manual::{NativeClassId, NativeInstanceId, NativePropertyValue, NativeString, NativeValueType};
use crate::errors::print_backtrace_and_exit;
use crate::tracked::{AutoDisposableScriptInstance};

#[derive(Debug, Clone)]
pub struct CCompatibleLang;

impl Lang for CCompatibleLang {
    type String<'a> = *mut u8;

    type RuntimePin = NativeRuntimePin;

    type UnpackedScriptObject = UserScriptImpl;

    fn drop_runtime_pin(_runtime_pin: &mut Self::RuntimePin) {
        todo!("is not needed without Hot reload")
    }

    fn clone_runtime_pin(_runtime_pin: &Self::RuntimePin) -> Self::RuntimePin {
        todo!("is not needed without Hot reload")
    }

    fn id_of(script: &Self::UnpackedScriptObject) -> fyrox::core::Uuid {
        script.uuid
    }

    fn unpack_script(script: &ScriptObject<Self>) -> Result<Self::UnpackedScriptObject, String> {
        let uuid = uuid_of_script(script);
        APP.with_borrow(|app| {
            let app = app.as_ref().unwrap();
            let metadata = app.scripts.get(&uuid).unwrap();
            let mut state = Vec::new();

            for (i, prop) in metadata.md.fields.iter().enumerate() {
                let value = &script.values[i];
                let name = prop.name.clone().into();

                let value = convert_value(name, value, prop.ty);
                state.push(value);
            }
            let instance = (app.functions.create_script_instance)(metadata.id, state.into(), Some(script.node.into()).into()).into_result_shallow()?;
            Ok(UnpackedObject {
                uuid,
                class: metadata.id,
                instance: AutoDisposableScriptInstance::new(instance),
            })
        })
    }
}

fn convert_value(name: NativeString, value: &ScriptFieldValue<CCompatibleLang>, ty: ScriptFieldValueType) -> NativePropertyValue {
    match value {
        ScriptFieldValue::Prefab(resource) => {
            assert_eq!(ty, ScriptFieldValueType::Prefab);
            match resource {
                Some(resource) => {
                    let prefab = LitePrefab::new(resource.clone());
                    NativePropertyValue {
                        name,
                        ty: NativeValueType::Prefab,
                        value: NativeValue {
                            Handle: NativeHandle::from_u128(prefab.to_external()),
                        },
                    }
                },
                None => {
                    NativePropertyValue {
                        name,
                        ty: NativeValueType::Prefab,
                        value: NativeValue {
                            Handle: Handle::<()>::NONE.into(),
                        },
                    }
                },
            }
        }
        ScriptFieldValue::Vector3(it) => {
            assert_eq!(ty, ScriptFieldValueType::Vector3);
            NativePropertyValue {
                name,
                ty: NativeValueType::Vector3,
                value: NativeValue {
                    Vector3: NativeVector3 {
                        x: it.x, y: it.y, z: it.z
                    },
                },
            }
        }
        ScriptFieldValue::Vector2(it) => {
            assert_eq!(ty, ScriptFieldValueType::Vector2);
            NativePropertyValue {
                name,
                ty: NativeValueType::Vector2,
                value: NativeValue {
                    Vector2: NativeVector2 {
                        x: it.x, y: it.y
                    },
                },
            }
        }
        ScriptFieldValue::Vector2I(it) => {
            assert_eq!(ty, ScriptFieldValueType::Vector2I);
            NativePropertyValue {
                name,
                ty: NativeValueType::Vector2,
                value: NativeValue {
                    Vector2I: NativeVector2I {
                        x: it.x, y: it.y
                    },
                },
            }
        }
        ScriptFieldValue::Quaternion(it) => {
            assert_eq!(ty, ScriptFieldValueType::Quaternion);
            NativePropertyValue {
                name,
                ty: NativeValueType::Quaternion,
                value: NativeValue {
                    Quaternion: NativeQuaternion {
                        i: it.i,j: it.j,k: it.k, w:it.w
                    },
                },
            }
        }
        value => {
            #[rustfmt::skip]
                        let (native_value_type, native_value) = match value {
                            ScriptFieldValue::bool(it) => { assert_eq!(ty, ScriptFieldValueType::bool); (NativeValueType::bool, NativeValue { bool: (*it).into()}) },
                            ScriptFieldValue::String(it) => { assert_eq!(ty, ScriptFieldValueType::String); (NativeValueType::String, NativeValue { String: (it.clone()).into()}) },
                            ScriptFieldValue::f32(it) => { assert_eq!(ty, ScriptFieldValueType::f32); (NativeValueType::f32, NativeValue { f32: *it}) },
                            ScriptFieldValue::f64(it) => { assert_eq!(ty, ScriptFieldValueType::f64); (NativeValueType::f64, NativeValue { f64: *it}) },
                            ScriptFieldValue::i16(it) => { assert_eq!(ty, ScriptFieldValueType::i16); (NativeValueType::i16, NativeValue { i16: *it}) },
                            ScriptFieldValue::i32(it) => { assert_eq!(ty, ScriptFieldValueType::i32); (NativeValueType::i32, NativeValue { i32: *it}) },
                            ScriptFieldValue::i64(it) => { assert_eq!(ty, ScriptFieldValueType::i64); (NativeValueType::i64, NativeValue { i64: *it}) },
                            ScriptFieldValue::Node(it) => { assert_eq!(ty, ScriptFieldValueType::Node); (NativeValueType::Handle,NativeValue { Handle: (*it).into()}) },
                            ScriptFieldValue::UiNode(it) => { assert_eq!(ty, ScriptFieldValueType::UiNode); (NativeValueType::Handle, NativeValue { Handle: (*it).into()}) },
                            ScriptFieldValue::String(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Prefab(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Vector3(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Vector2(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Vector2I(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Quaternion(_) => { todo!("handled in another block") },
                            ScriptFieldValue::RuntimePin(_) => { todo!("not supported for C") },
                        };
            NativePropertyValue {
                name,
                ty: native_value_type,
                value: native_value,
            }
        }
    }
}

impl<T> From<Handle<T>> for NativeHandle {
    fn from(value: Handle<T>) -> Self {
        let value = value.encode_to_u128();
        NativeHandle {
            high: (value >> 64) as u64,
            low: value as u64,
        }
    }
}

impl<T> From<NativeHandle> for Handle<T> {
    fn from(value: NativeHandle) -> Self {
        let value = (value.high as u128) << 64 | (value.low as u128);
        Handle::decode_from_u128(value)
    }
}

#[derive(Debug, Default, Clone)]
pub struct NativeRuntimePin;

impl Visit for NativeRuntimePin {
    fn visit(
        &mut self,
        _name: &str,
        _visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        todo!("is not needed without Hot reload")
    }
}

#[derive(Debug, Clone)]
pub struct UnpackedObject {
    pub uuid: Uuid,
    pub class: NativeClassId,
    pub instance: AutoDisposableScriptInstance,
}


impl From<UnpackedObject> for NativeInstanceId {
    fn from(value: UnpackedObject) -> Self {
        value.instance.inner()
    }
}

impl From<NativeInstanceId> for UnpackedObject {
    fn from(_value: NativeInstanceId) -> Self {
        println!("call of Rust-implemented script methods is not implemented yet");
        print_backtrace_and_exit();
    }
}

impl Visit for UnpackedObject {
    fn visit(
        &mut self,
        _name: &str,
        _visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        println!("is not needed without Hot reload");
        print_backtrace_and_exit();
    }
}
