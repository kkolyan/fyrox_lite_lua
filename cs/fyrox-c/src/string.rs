use crate::bindings_manual::{u8_array, NativeString, NativeType};

impl From<u8_array> for String {
    fn from(value: NativeString) -> Self {
        let vec = u8::from_native_array(value);
        String::from_utf8(vec).unwrap()
    }
}

impl From<String> for u8_array {
    fn from(value: String) -> Self {
        let vec = value.into_bytes();
        u8::to_native_array(vec)
    }
}