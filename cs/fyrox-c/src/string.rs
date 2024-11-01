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

pub extern "C" fn fyrox_lite_upload_data(data: u8_slice) -> u8_slice {
    let len = data.len as i32;
    let data = Vec::from(data);
    let ptr = Arena::allocate_vec(data);
    u8_slice { begin: ptr, len }
}