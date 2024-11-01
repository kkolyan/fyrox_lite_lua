use std::fmt::Display;
use gen_common::context::GenerationContext;
use gen_common::templating::{render, render_string};
use lite_model::{DataType, StructClass};
use crate::lite_csgen::api_types;
use crate::lite_csgen::gen_rs::RustEmitter;

pub(crate) fn generate_result(s: &mut String, rust: &RustEmitter, wrapped_type: &DataType) {
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
                    var __item = value;
                    var __item_from_facade = ${item_from_facade};
                    return new ${blittable}_optional { value = __item_from_facade${unwrap}, has_value = 1 };
                }
            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("facade", &marshalling.to_facade()),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("unwrap", &if marshalling.to_facade() == "object" {""} else {".Value"}),
    ]);
    rust.emit_statement(render_string(r#"
            #[repr(C)]
            pub struct ${class_native}_optional {
                pub value: ${class_lite},
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
    ]))
}

pub fn generate_slice(mut s: &mut String, rust: &RustEmitter, wrapped_type: &DataType) {
    let marshalling = api_types::type_cs(wrapped_type);
    render(&mut s, r#"

            [StructLayout(LayoutKind.Sequential)]
            internal struct ${blittable}_slice
            {
                private unsafe ${blittable}* begin;
                private int length;
                internal List<${facade}>? Fetched;

                internal unsafe ${blittable}_slice(${blittable}* begin, int length)
                {
                    this.begin = begin;
                    this.length = length;
                }

                internal static unsafe void Fetch(ref ${blittable}_slice self)
                {
                    var fetched = new List<${facade}>();
                    for (int i = 0; i < self.length; i++)
                    {
                        var __item = *(self.begin + i);
                        var __item_to_facade = ${item_to_facade};
                        fetched.Add(__item_to_facade);
                    }
                    self.Fetched = fetched;
                }

                internal static unsafe List<${facade}> ToFacade(in ${blittable}_slice self)
                {
                    var fetched = new List<${facade}>();
                    for (int i = 0; i < self.length; i++)
                    {
                        var __item = *(self.begin + i);
                        var __item_to_facade = ${item_to_facade};
                        fetched.Add(__item_to_facade);
                    }
                    return fetched;
                }

                internal static ${blittable}_slice FromFacade(in List<${facade}> self)
                {
                    // ${item_from_facade}
                    throw new Exception("slice serialization not implemented yet");
                }

            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("facade", &marshalling.to_facade()),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
    ]);
}
