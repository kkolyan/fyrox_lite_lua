#[macro_export]
macro_rules! native_utils {
    ($ty:ty, $array:ident, $option:ident, $result:ident) => {
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct $array {
            pub len: u32,
            pub capacity: u32,
            pub items: *mut $ty,
        }
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct $option {
            pub present: bool,
            pub value: $ty,
        }
        #[allow(non_camel_case_types)]
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct $result {
            pub ok: bool,
            pub err: NativeString,
            pub value: $ty,
        }
        
        #[allow(non_camel_case_types)]
        impl NativeType for $ty {
            type Array = $array;
            type Option = $option;
            type Result = $result;
        
            fn to_native_array(v: Vec<$ty>) -> $array {
                let len = v.len() as u32;
                let capacity = v.capacity() as u32;
                let items = crate::arena::Arena::allocate_vec(v);
                $array {
                    len,
                    items,
                    capacity,
                }
            }
        
            fn from_native_array(v: $array) -> Vec<$ty> {
                let $array {
                    len,
                    capacity,
                    items,
                } = v;
                unsafe { Vec::from_raw_parts(items, len as usize, capacity as usize) }
            }
        
            fn to_native_option(v: Option<Self>) -> Self::Option {
                match v {
                    Some(it) => Self::Option {
                        present: true,
                        value: it,
                    },
                    None => Self::Option {
                        present: false,
                        value: unsafe { std::mem::zeroed() },
                    },
                }
            }
        
            fn from_native_option(v: Self::Option) -> Option<Self> {
                if v.present {
                    Some(v.value)
                } else {
                    None
                }
            }
        
            fn to_native_result<E: Display>(v: Result<Self, E>) -> Self::Result {
                match v {
                    Ok(it) => Self::Result {
                        ok: true,
                        err: unsafe { std::mem::zeroed() },
                        value: it,
                    },
                    Err(err) => Self::Result {
                        ok: false,
                        err: u8::to_native_array(err.to_string().into_bytes()),
                        value: unsafe { std::mem::zeroed() },
                    },
                }
            }
        
            fn from_native_result<U: UserScript>(v: Self::Result) -> Result<Self, U::LangSpecificError> {
                if v.ok {
                    Ok(v.value)
                } else {
                    Err(U::create_error(
                        &String::from_utf8(u8::from_native_array(v.err)).unwrap(),
                    ))
                }
            }
        }
    };
}