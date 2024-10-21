use std::{cell::RefCell, collections::HashMap, ffi::CString};

use fyrox::{
    asset::untyped::{ResourceHeader, UntypedResource},
    core::{
        pool::{Handle, Pool},
        visitor::Visit,
        Uuid,
    },
};
use fyrox_lite::{
    script_metadata::ScriptFieldValueType,
    script_object::{Lang, ScriptFieldValue, ScriptObject},
    script_object_residence::uuid_of_script,
};

use crate::{
    manual_bindings::{
        NativeHandle, NativeInstanceId, NativeQuaternion, NativeValue, NativeVector3,
    },
    scripted_app::APP,
};

#[derive(Debug, Clone)]
pub struct CCompatibleLang;

impl Lang for CCompatibleLang {
    type String<'a> = *mut u8;

    type RuntimePin = NativeRuntimePin;

    type UnpackedScriptObject = UnpackedObject;

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
        let handle = APP.with_borrow(|app| {
            let app = app.as_ref().unwrap();
            let metadata = app.scripts.get(&uuid).unwrap();
            let instance = (app.functions.create_script_instance)(metadata.id);

            for (i, prop) in metadata.md.fields.iter().enumerate() {
                let value = &script.values[i];

                match value {
                    ScriptFieldValue::String(it) => {
                        let s = CString::new(it.as_str()).unwrap();
                        assert_eq!(prop.ty, ScriptFieldValueType::String);
                        (app.functions.set_property)(instance, i as u16, NativeValue { String: s.as_ptr() });
                    }
                    ScriptFieldValue::Prefab(resource) => {
                        assert_eq!(prop.ty, ScriptFieldValueType::Prefab);
                        RESOURCES.with_borrow_mut(|registry| {
                            if registry.is_none() {
                                *registry = Some(Default::default());
                            }
                            let registry = registry.as_mut().unwrap();
                            if let Some(resource) = resource {
                                let resource = resource.clone().into_untyped();
                                let existing_handle =
                                    registry.reverse_resources.get(&resource.0.data_ptr());
                                let handle = existing_handle
                                    .copied()
                                    .unwrap_or_else(|| registry.resources.spawn(resource));
                                (app.functions.set_property)(
                                    instance,
                                    i as u16,
                                    NativeValue {
                                        Handle: handle.into(),
                                    },
                                );
                            } else {
                                (app.functions.set_property)(
                                    instance,
                                    i as u16,
                                    NativeValue {
                                        Handle: Handle::<()>::NONE.into(),
                                    },
                                );
                            }
                        });
                    }
                    ScriptFieldValue::Vector3(it) => {
                        assert_eq!(prop.ty, ScriptFieldValueType::Vector3);
                        (app.functions.set_property)(
                            instance,
                            i as u16,
                            NativeValue {
                                Vector3: NativeVector3 {
                                    x: it.x, y: it.y, z: it.z
                                },
                            },
                        );
                    }
                    ScriptFieldValue::Quaternion(it) => {
                        assert_eq!(prop.ty, ScriptFieldValueType::Quaternion);
                        (app.functions.set_property)(
                            instance,
                            i as u16,
                            NativeValue {
                                Quaternion: NativeQuaternion {
                                    i: it.i,j: it.j,k: it.k, w:it.w
                                },
                            },
                        );
                    }
                    value => {
                        #[rustfmt::skip]
                        let native_value = match value {
                            ScriptFieldValue::bool(it) => { assert_eq!(prop.ty, ScriptFieldValueType::bool); NativeValue { bool: *it} },
                            ScriptFieldValue::f32(it) => { assert_eq!(prop.ty, ScriptFieldValueType::f32); NativeValue { f32: *it} },
                            ScriptFieldValue::f64(it) => { assert_eq!(prop.ty, ScriptFieldValueType::f64); NativeValue { f64: *it} },
                            ScriptFieldValue::i16(it) => { assert_eq!(prop.ty, ScriptFieldValueType::i16); NativeValue { i16: *it} },
                            ScriptFieldValue::i32(it) => { assert_eq!(prop.ty, ScriptFieldValueType::i32); NativeValue { i32: *it} },
                            ScriptFieldValue::i64(it) => { assert_eq!(prop.ty, ScriptFieldValueType::i64); NativeValue { i64: *it} },
                            ScriptFieldValue::Node(it) => { assert_eq!(prop.ty, ScriptFieldValueType::Node); NativeValue { Handle: (*it).into()} },
                            ScriptFieldValue::UiNode(it) => { assert_eq!(prop.ty, ScriptFieldValueType::UiNode); NativeValue { Handle: (*it).into()} },
                            ScriptFieldValue::String(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Prefab(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Vector3(_) => { todo!("handled in another block") },
                            ScriptFieldValue::Quaternion(_) => { todo!("handled in another block") },
                            ScriptFieldValue::RuntimePin(_) => { todo!("not supported for C") },
                        };
                        (app.functions.set_property)(instance, i as u16, native_value);
                    }
                }
            }
            instance
        });
        Ok(UnpackedObject { uuid, handle })
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

thread_local! {
    static RESOURCES: RefCell<Option<ResourceRegistry>> = Default::default();
}

#[derive(Debug, Default)]
pub struct ResourceRegistry {
    pub resources: Pool<UntypedResource>,
    pub reverse_resources: HashMap<*mut ResourceHeader, Handle<UntypedResource>>,
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

#[derive(Clone, Copy, Debug)]
pub struct UnpackedObject {
    pub uuid: Uuid,
    pub handle: NativeInstanceId,
}

impl Visit for UnpackedObject {
    fn visit(
        &mut self,
        _name: &str,
        _visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        todo!("is not needed without Hot reload")
    }
}
