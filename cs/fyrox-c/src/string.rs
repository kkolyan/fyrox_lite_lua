use crate::arena::Arena;
use crate::bindings_lite_2::u8_slice;
use crate::bindings_manual::{NativeString};

impl From<u8_slice> for String {
    fn from(value: NativeString) -> Self {
        let vec = Vec::from(value);
        String::from_utf8(vec).unwrap()
    }
}

impl From<String> for u8_slice {
    fn from(value: String) -> Self {
        let vec = value.into_bytes();
        u8_slice::from(vec)
    }
}