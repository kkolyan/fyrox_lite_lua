
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::unused_unit)]
#![allow(clippy::let_unit_value)]
#![allow(unused)]
use crate::*;
use fyrox_lite::externalizable::Externalizable;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_result {
    pub ok: i32,
    pub value: NativeBool_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeBool_result_value {
    ok: NativeBool,
    err: NativeString,
}

impl NativeBool_result {
    pub fn into_result_shallow(self) -> Result<NativeBool, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<bool, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<bool, crate::LangSpecificError>> for NativeBool_result {
    fn from(value: Result<bool, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeBool_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeBool_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeBool_result> for Result<bool, crate::LangSpecificError> {
    fn from(value: NativeBool_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_optional {
    pub value: NativeBool,
    pub has_value: i32,
}

impl From<Option<bool>> for NativeBool_optional {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeBool_optional> for Option<bool> {
    fn from(value: NativeBool_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeBool_slice {
    pub begin: *mut NativeBool,
    pub len: i32,
}

impl Default for NativeBool_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<bool>> for NativeBool_slice {
    fn from(value: Vec<bool>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeBool> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeBool_slice> for Vec<bool> {
    fn from(value: NativeBool_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_bool_slice(data: NativeBool_slice) -> NativeBool_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeBool_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_result {
    pub ok: i32,
    pub value: u8_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union u8_result_value {
    ok: u8,
    err: NativeString,
}

impl u8_result {
    pub fn into_result_shallow(self) -> Result<u8, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<u8, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<u8, crate::LangSpecificError>> for u8_result {
    fn from(value: Result<u8, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: u8_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: u8_result_value { err: err.into() },
            },
        }
    }
}

impl From<u8_result> for Result<u8, crate::LangSpecificError> {
    fn from(value: u8_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_optional {
    pub value: u8,
    pub has_value: i32,
}

impl From<Option<u8>> for u8_optional {
    fn from(value: Option<u8>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<u8_optional> for Option<u8> {
    fn from(value: u8_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct u8_slice {
    pub begin: *mut u8,
    pub len: i32,
}

impl Default for u8_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<u8>> for u8_slice {
    fn from(value: Vec<u8>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<u8> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<u8_slice> for Vec<u8> {
    fn from(value: u8_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_u8_slice(data: u8_slice) -> u8_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    u8_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_result {
    pub ok: i32,
    pub value: i32_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union i32_result_value {
    ok: i32,
    err: NativeString,
}

impl i32_result {
    pub fn into_result_shallow(self) -> Result<i32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<i32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<i32, crate::LangSpecificError>> for i32_result {
    fn from(value: Result<i32, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: i32_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: i32_result_value { err: err.into() },
            },
        }
    }
}

impl From<i32_result> for Result<i32, crate::LangSpecificError> {
    fn from(value: i32_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_optional {
    pub value: i32,
    pub has_value: i32,
}

impl From<Option<i32>> for i32_optional {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<i32_optional> for Option<i32> {
    fn from(value: i32_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i32_slice {
    pub begin: *mut i32,
    pub len: i32,
}

impl Default for i32_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<i32>> for i32_slice {
    fn from(value: Vec<i32>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<i32> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<i32_slice> for Vec<i32> {
    fn from(value: i32_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_i32_slice(data: i32_slice) -> i32_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    i32_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_result {
    pub ok: i32,
    pub value: i64_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union i64_result_value {
    ok: i64,
    err: NativeString,
}

impl i64_result {
    pub fn into_result_shallow(self) -> Result<i64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<i64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<i64, crate::LangSpecificError>> for i64_result {
    fn from(value: Result<i64, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: i64_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: i64_result_value { err: err.into() },
            },
        }
    }
}

impl From<i64_result> for Result<i64, crate::LangSpecificError> {
    fn from(value: i64_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_optional {
    pub value: i64,
    pub has_value: i32,
}

impl From<Option<i64>> for i64_optional {
    fn from(value: Option<i64>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<i64_optional> for Option<i64> {
    fn from(value: i64_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct i64_slice {
    pub begin: *mut i64,
    pub len: i32,
}

impl Default for i64_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<i64>> for i64_slice {
    fn from(value: Vec<i64>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<i64> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<i64_slice> for Vec<i64> {
    fn from(value: i64_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_i64_slice(data: i64_slice) -> i64_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    i64_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_result {
    pub ok: i32,
    pub value: f32_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union f32_result_value {
    ok: f32,
    err: NativeString,
}

impl f32_result {
    pub fn into_result_shallow(self) -> Result<f32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<f32, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<f32, crate::LangSpecificError>> for f32_result {
    fn from(value: Result<f32, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: f32_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: f32_result_value { err: err.into() },
            },
        }
    }
}

impl From<f32_result> for Result<f32, crate::LangSpecificError> {
    fn from(value: f32_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_optional {
    pub value: f32,
    pub has_value: i32,
}

impl From<Option<f32>> for f32_optional {
    fn from(value: Option<f32>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<f32_optional> for Option<f32> {
    fn from(value: f32_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f32_slice {
    pub begin: *mut f32,
    pub len: i32,
}

impl Default for f32_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<f32>> for f32_slice {
    fn from(value: Vec<f32>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<f32> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<f32_slice> for Vec<f32> {
    fn from(value: f32_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_f32_slice(data: f32_slice) -> f32_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    f32_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_result {
    pub ok: i32,
    pub value: f64_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union f64_result_value {
    ok: f64,
    err: NativeString,
}

impl f64_result {
    pub fn into_result_shallow(self) -> Result<f64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<f64, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<f64, crate::LangSpecificError>> for f64_result {
    fn from(value: Result<f64, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: f64_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: f64_result_value { err: err.into() },
            },
        }
    }
}

impl From<f64_result> for Result<f64, crate::LangSpecificError> {
    fn from(value: f64_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_optional {
    pub value: f64,
    pub has_value: i32,
}

impl From<Option<f64>> for f64_optional {
    fn from(value: Option<f64>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<f64_optional> for Option<f64> {
    fn from(value: f64_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct f64_slice {
    pub begin: *mut f64,
    pub len: i32,
}

impl Default for f64_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<f64>> for f64_slice {
    fn from(value: Vec<f64>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<f64> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<f64_slice> for Vec<f64> {
    fn from(value: f64_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_f64_slice(data: f64_slice) -> f64_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    f64_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_result {
    pub ok: i32,
    pub value: NativeString_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeString_result_value {
    ok: NativeString,
    err: NativeString,
}

impl NativeString_result {
    pub fn into_result_shallow(self) -> Result<NativeString, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<String, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<String, crate::LangSpecificError>> for NativeString_result {
    fn from(value: Result<String, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeString_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeString_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeString_result> for Result<String, crate::LangSpecificError> {
    fn from(value: NativeString_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_optional {
    pub value: NativeString,
    pub has_value: i32,
}

impl From<Option<String>> for NativeString_optional {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeString_optional> for Option<String> {
    fn from(value: NativeString_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString_slice {
    pub begin: *mut NativeString,
    pub len: i32,
}

impl Default for NativeString_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<String>> for NativeString_slice {
    fn from(value: Vec<String>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeString> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeString_slice> for Vec<String> {
    fn from(value: NativeString_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_String_slice(data: NativeString_slice) -> NativeString_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeString_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_result {
    pub ok: i32,
    pub value: NativeInstanceId_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeInstanceId_result_value {
    ok: NativeInstanceId,
    err: NativeString,
}

impl NativeInstanceId_result {
    pub fn into_result_shallow(self) -> Result<NativeInstanceId, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<crate::UserScriptImpl, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<crate::UserScriptImpl, crate::LangSpecificError>> for NativeInstanceId_result {
    fn from(value: Result<crate::UserScriptImpl, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeInstanceId_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeInstanceId_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeInstanceId_result> for Result<crate::UserScriptImpl, crate::LangSpecificError> {
    fn from(value: NativeInstanceId_result) -> Self {
        value.into_result()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_optional {
    pub value: NativeInstanceId,
    pub has_value: i32,
}

impl From<Option<crate::UserScriptImpl>> for NativeInstanceId_optional {
    fn from(value: Option<crate::UserScriptImpl>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeInstanceId_optional> for Option<crate::UserScriptImpl> {
    fn from(value: NativeInstanceId_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_slice {
    pub begin: *mut NativeInstanceId,
    pub len: i32,
}

impl Default for NativeInstanceId_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<crate::UserScriptImpl>> for NativeInstanceId_slice {
    fn from(value: Vec<crate::UserScriptImpl>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeInstanceId> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeInstanceId_slice> for Vec<crate::UserScriptImpl> {
    fn from(value: NativeInstanceId_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_crate_UserScriptImpl_slice(
    data: NativeInstanceId_slice,
) -> NativeInstanceId_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeInstanceId_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptMetadata_slice {
    pub begin: *mut NativeScriptMetadata,
    pub len: i32,
}

impl Default for NativeScriptMetadata_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeScriptMetadata>> for NativeScriptMetadata_slice {
    fn from(value: Vec<NativeScriptMetadata>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeScriptMetadata> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeScriptMetadata_slice> for Vec<NativeScriptMetadata> {
    fn from(value: NativeScriptMetadata_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeScriptMetadata_slice(
    data: NativeScriptMetadata_slice,
) -> NativeScriptMetadata_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeScriptMetadata_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptProperty_slice {
    pub begin: *mut NativeScriptProperty,
    pub len: i32,
}

impl Default for NativeScriptProperty_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeScriptProperty>> for NativeScriptProperty_slice {
    fn from(value: Vec<NativeScriptProperty>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeScriptProperty> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeScriptProperty_slice> for Vec<NativeScriptProperty> {
    fn from(value: NativeScriptProperty_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeScriptProperty_slice(
    data: NativeScriptProperty_slice,
) -> NativeScriptProperty_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeScriptProperty_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeValue_slice {
    pub begin: *mut NativeValue,
    pub len: i32,
}

impl Default for NativeValue_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativeValue>> for NativeValue_slice {
    fn from(value: Vec<NativeValue>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativeValue> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativeValue_slice> for Vec<NativeValue> {
    fn from(value: NativeValue_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativeValue_slice(
    data: NativeValue_slice,
) -> NativeValue_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativeValue_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeHandle_optional {
    pub value: NativeHandle,
    pub has_value: i32,
}

impl From<Option<NativeHandle>> for NativeHandle_optional {
    fn from(value: Option<NativeHandle>) -> Self {
        match value {
            Some(it) => Self {
                value: it.into(),
                has_value: 1,
            },
            None => Self {
                value: unsafe { std::mem::zeroed() },
                has_value: 0,
            },
        }
    }
}

impl From<NativeHandle_optional> for Option<NativeHandle> {
    fn from(value: NativeHandle_optional) -> Self {
        if value.has_value != 0 {
            Some(value.value.into())
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePropertyValue_slice {
    pub begin: *mut NativePropertyValue,
    pub len: i32,
}

impl Default for NativePropertyValue_slice {
    fn default() -> Self {
        Self {
            begin: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl From<Vec<NativePropertyValue>> for NativePropertyValue_slice {
    fn from(value: Vec<NativePropertyValue>) -> Self {
        let len = value.len() as i32;
        if len == 0 {
            return Self::default();
        }
        let native_vec: Vec<NativePropertyValue> = value.into_iter().map(|it| it.into()).collect();
        let begin = crate::Arena::allocate_vec(native_vec);
        Self { begin, len }
    }
}

impl From<NativePropertyValue_slice> for Vec<NativePropertyValue> {
    fn from(value: NativePropertyValue_slice) -> Self {
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

#[no_mangle]
pub extern "C" fn fyrox_lite_upload_NativePropertyValue_slice(
    data: NativePropertyValue_slice,
) -> NativePropertyValue_slice {
    let mut vec = Vec::new();
    unsafe {
        for i in 0..data.len {
            let v = *data.begin.add(i as usize);
            vec.push(v);
        }
    }
    let ptr = crate::Arena::allocate_vec(vec);
    NativePropertyValue_slice {
        begin: ptr,
        len: data.len,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeInstanceId_optional_result {
    pub ok: i32,
    pub value: NativeInstanceId_optional_result_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeInstanceId_optional_result_value {
    ok: NativeInstanceId_optional,
    err: NativeString,
}

impl NativeInstanceId_optional_result {
    pub fn into_result_shallow(
        self,
    ) -> Result<NativeInstanceId_optional, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok)
            } else {
                Err(self.value.err.into())
            }
        }
    }
    pub fn into_result(self) -> Result<Option<crate::UserScriptImpl>, crate::LangSpecificError> {
        unsafe {
            if self.ok != 0 {
                Ok(self.value.ok.into())
            } else {
                Err(self.value.err.into())
            }
        }
    }
}

impl From<Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>>
    for NativeInstanceId_optional_result
{
    fn from(value: Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>) -> Self {
        match value {
            Ok(it) => Self {
                ok: 1,
                value: NativeInstanceId_optional_result_value { ok: it.into() },
            },
            Err(err) => Self {
                ok: 0,
                value: NativeInstanceId_optional_result_value { err: err.into() },
            },
        }
    }
}

impl From<NativeInstanceId_optional_result>
    for Result<Option<crate::UserScriptImpl>, crate::LangSpecificError>
{
    fn from(value: NativeInstanceId_optional_result) -> Self {
        value.into_result()
    }
}
