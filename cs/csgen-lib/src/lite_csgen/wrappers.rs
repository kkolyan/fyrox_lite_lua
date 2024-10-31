use std::fmt::Display;
use gen_common::templating::render;
use lite_model::{DataType, StructClass};
use crate::lite_csgen::api_types;

pub(crate) fn generate_result(s: &mut String, wrapped_type: &DataType) {
    let marshalling = api_types::type_rs2cs(wrapped_type);

    render(s, r#"

            [StructLayout(LayoutKind.Explicit)]
            internal struct ${blittable}_result
            {
                [FieldOffset(0)]
                internal ${bool_type} Ok;

                [FieldOffset(sizeof(${bool_type}))]
                internal ${blittable} Value;

                [FieldOffset(sizeof(${bool_type}))]
                internal string Err;

                internal static unsafe ${facade} ToFacade(in ${blittable}_result self)
                {
                    if (self.Ok != 0)
                    {
                        var __item = self.Value;
                        var __item_to_facade = ${item_to_facade};
                        return __item_to_facade;
                    }
                    throw new Exception(self.Err);
                }

                internal static ${blittable}_result FromFacade(in ${facade} self)
                {
                    var __item = self;
                    var __item_from_facade = ${item_from_facade};
                    return new ${blittable}_result {Ok = 1, Value = __item_from_facade};
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

pub(crate) fn generate_optional(s: &mut String, wrapped_type: &DataType) {
    let marshalling = api_types::type_rs2cs(wrapped_type);
    render(s, r#"

            [StructLayout(LayoutKind.Sequential)]
            internal struct ${blittable}_optional
            {
                internal ${blittable} Value;
                internal bool HasValue;

                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                public static ${facade}? ToFacade(in ${blittable}_optional value)
                {
                    if (value.HasValue)
                    {
                        var __item = value.Value;
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
                        return new ${blittable}_optional { Value = default, HasValue = false };
                    }
                    var __item = value;
                    var __item_from_facade = ${item_from_facade};
                    return new ${blittable}_optional { Value = __item_from_facade${unwrap}, HasValue = true };
                }
            }
    "#, [
        ("blittable", &marshalling.to_blittable()),
        ("facade", &marshalling.to_facade()),
        ("item_to_facade", &if marshalling.is_mapped() {format!("{}.ToFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("item_from_facade", &if marshalling.is_mapped() {format!("{}.FromFacade(__item)", marshalling.to_blittable())} else {"__item".to_string()}),
        ("unwrap", &if marshalling.to_facade() == "object" {""} else {".Value"}),
    ]);
}

pub fn generate_slice(mut s: &mut String, wrapped_type: &DataType) {
    let marshalling = api_types::type_rs2cs(wrapped_type);
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
