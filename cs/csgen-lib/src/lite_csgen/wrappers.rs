use std::fmt::Display;
use gen_common::context::GenerationContext;
use gen_common::templating::{render, render_string};
use lite_model::{DataType};
use crate::lite_csgen::api_types;
use crate::lite_csgen::gen_rs::RustEmitter;

pub(crate) fn generate_result(s: &mut String, rust: &mut RustEmitter, wrapped_type: &DataType, ctx: &GenerationContext) {
    let marshalling = api_types::type_cs(wrapped_type);

    render(s, r#"

            [StructLayout(LayoutKind.Explicit)]
            internal struct ${blittable}_result
            {
                [FieldOffset(0)]
                private ${bool_type} ok;

                [FieldOffset(sizeof(${bool_type}))]
                private ${blittable} value;

                [FieldOffset(sizeof(${bool_type}))]
                private string err;

                internal static unsafe ${facade} ToFacade(in ${blittable}_result self)
                {
                    if (self.ok != 0)
                    {
                        var __item = self.value;
                        var __item_to_facade = ${item_to_facade};
                        return __item_to_facade;
                    }
                    throw new Exception(self.err);
                }

                internal static ${blittable}_result FromFacade(in ${facade} self)
                {
                    var __item = self;
                    var __item_from_facade = ${item_from_facade};
                    return new ${blittable}_result {ok = 1, value = __item_from_facade};
                }
            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("bool_type", &"int"),
        ("facade", &marshalling.to_facade()),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
    ]);
    rust.emit_statement(render_string(r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ${class_native}_result {
                pub ok: i32,
                pub value: ${class_native}_result_value,
            }

            #[repr(C)]
            #[derive(Clone, Copy)]
            pub union ${class_native}_result_value {
                ok: ${class_native},
                err: NativeString,
            }

            impl From<Result<${class_lite}, crate::LangSpecificError>> for ${class_native}_result {
                fn from(value: Result<${class_lite}, crate::LangSpecificError>) -> Self {
                    match value {
                        Ok(it) => Self { ok: 1, value: ${class_native}_result_value { ok: it.into() } },
                        Err(err) => Self { ok: 0, value: ${class_native}_result_value { err: err.into() } },
                    }
                }
            }

            impl From<${class_native}_result> for Result<${class_lite}, crate::LangSpecificError> {
                fn from(value: ${class_native}_result) -> Self {
                    unsafe {
                        if value.ok != 0 {
                            Ok(value.value.ok.into())
                        } else {
                            Err(value.value.err.into())
                        }
                    }
                }
            }
    "#, [
        ("class_native", &api_types::type_rs(wrapped_type, ctx).to_native()),
        ("class_lite", &api_types::type_rs(wrapped_type, ctx).to_lite()),
    ]));
}

pub(crate) fn generate_optional(s: &mut String, rust: &mut RustEmitter, wrapped_type: &DataType, ctx: &GenerationContext) {
    let marshalling = api_types::type_cs(wrapped_type);
    render(s, r#"

            [StructLayout(LayoutKind.Sequential)]
            internal struct ${blittable}_optional
            {
                private ${blittable} value;
                private int has_value;

                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                public static ${facade}? ToFacade(in ${blittable}_optional value)
                {
                    if (value.has_value != 0)
                    {
                        var __item = value.value;
                        var __item_to_facade = ${item_to_facade};
                        return __item_to_facade;
                    }
                    return null;
                }

                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                public static ${blittable}_optional FromFacade(in ${facade}? value)
                {
                    if (value == null)
                    {
                        return new ${blittable}_optional { value = default, has_value = 0 };
                    }
                    var __item = value${unwrap};
                    var __item_from_facade = ${item_from_facade};
                    return new ${blittable}_optional { value = __item_from_facade, has_value = 1 };
                }
            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("facade", &marshalling.to_facade()),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("unwrap", &if marshalling.to_facade() == "object" || marshalling.to_facade() == "string" {""} else {".Value"}),
    ]);
    rust.emit_statement(render_string(r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ${class_native}_optional {
                pub value: ${class_native},
                pub has_value: i32,
            }

            impl From<Option<${class_lite}>> for ${class_native}_optional {
                fn from(value: Option<${class_lite}>) -> Self {
                    match value {
                        Some(it) => Self { value: it.into(), has_value: 1 },
                        None => Self { value: unsafe { std::mem::zeroed() }, has_value: 0 },
                    }
                }
            }

            impl From<${class_native}_optional> for Option<${class_lite}> {
                fn from(value: ${class_native}_optional) -> Self {
                    if value.has_value != 0 {
                        Some(value.value.into())
                    } else {
                        None
                    }
                }
            }
    "#, [
        ("class_native", &api_types::type_rs(wrapped_type, ctx).to_native()),
        ("class_lite", &api_types::type_rs(wrapped_type, ctx).to_lite()),
    ]));
}

pub fn generate_slice(mut s: &mut String, rust: &mut RustEmitter, wrapped_type: &DataType, ctx: &GenerationContext) {
    let marshalling = api_types::type_cs(wrapped_type);
    let class_lite_escaped = api_types::type_rs(wrapped_type, ctx).to_lite().replace("::", "_");
    render(&mut s, r#"

            [StructLayout(LayoutKind.Sequential)]
            internal partial struct ${blittable}_slice
            {
                internal unsafe ${blittable}* begin;
                internal int length;

                internal unsafe ${blittable}_slice(${blittable}* begin, int length)
                {
                    this.begin = begin;
                    this.length = length;
                }

                internal static unsafe List<${facade}> ToFacade(in ${blittable}_slice self)
                {
                    var fetched = new List<${facade}>();
                    
                    for (var i = 0; i < self.length; i++)
                    {
                        var __item = *(self.begin + i);
                        var __item_to_facade = ${item_to_facade};
                        fetched.Add(__item_to_facade);
                    }
                    return fetched;
                }

                [ThreadStatic]
                private static ${blittable}[]? _uploadBuffer;

                internal static ${blittable}_slice FromFacade(in List<${facade}> self)
                {
                    _uploadBuffer ??= new ${blittable}[1024];
                    while (_uploadBuffer.Length < self.Count)
                    {
                        _uploadBuffer = new ${blittable}[_uploadBuffer.Length * 2];
                    }

                    for (var i = 0; i < self.Count; i++)
                    {
                        var __item = self[i];
                        var __item_from_facade = ${item_from_facade};
                        _uploadBuffer[i] = __item_from_facade;
                    }

                    unsafe
                    {
                        fixed (${blittable}* buffer_ptr = _uploadBuffer)
                        {
                            var native_slice = fyrox_lite_upload_${class_lite_escaped}_slice(new ${blittable}_slice(buffer_ptr, self.Count));
                            return native_slice;
                        }
                    }
                }

                [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                private static unsafe partial ${blittable}_slice fyrox_lite_upload_${class_lite_escaped}_slice(${blittable}_slice managed);
            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("facade", &marshalling.to_facade()),
        ("class_lite_escaped", &class_lite_escaped),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
    ]);
    rust.emit_statement(render_string(r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ${class_native}_slice {
                pub begin: *mut ${class_native},
                pub len: i32,
            }

            impl From<Vec<${class_lite}>> for ${class_native}_slice {
                fn from(value: Vec<${class_lite}>) -> Self {
                    let len = value.len() as i32;
                    let native_vec: Vec<${class_native}> = value.into_iter().map(|it| it.into()).collect();
                    let begin = crate::Arena::allocate_vec(native_vec);
                    Self { begin, len }
                }
            }

            impl From<${class_native}_slice> for Vec<${class_lite}> {
                fn from(value: ${class_native}_slice) -> Self {
                    let mut vec = Vec::new();
                    unsafe {
                        for i in 0..value.len {
                            let v = *value.begin.add(i as usize);
                            vec.push(v.into());
                        }
                    }
                    vec
                }
            }

            pub extern "C" fn fyrox_lite_upload_${class_lite_escaped}_slice(data: ${class_native}_slice) -> ${class_native}_slice {
                let len = data.len;
                let data = Vec::from(data);
                let ptr = Arena::allocate_vec(data);
                ${class_native}_slice { begin: ptr, len }
            }
    "#, [
        ("class_native", &api_types::type_rs(wrapped_type, ctx).to_native()),
        ("class_lite", &api_types::type_rs(wrapped_type, ctx).to_lite()),
        ("class_lite_escaped", &class_lite_escaped),
    ]));
}
