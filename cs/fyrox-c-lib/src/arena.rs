use std::{
    cell::RefCell,
    ffi::{c_char, CStr, CString},
    os::raw::c_void,
};

// TODO replace with SendWrapper
thread_local! {
    static ARENA: RefCell<Arena> = Default::default();
}

#[derive(Debug, Default)]
pub(crate) struct Arena {
    ptrs: Vec<Ptr>,
}

#[derive(Debug)]
enum Ptr {
    Vec {
        ptr: *mut c_void,
        len: usize,
        cap: usize,
    },
    Str(*mut c_char),
}

impl Arena {
    pub(crate) fn allocate_vec<T>(mut v: Vec<T>) -> *mut T {
        ARENA.with_borrow_mut(|arena| {
            let ptr = v.as_mut_ptr();
            arena.ptrs.push(Ptr::Vec {
                ptr: ptr as *mut c_void,
                len: v.len(),
                cap: v.capacity(),
            });
            std::mem::forget(v);
            ptr
        })
    }
    pub(crate) fn allocate_c_str<T>(mut v: String) -> *mut c_char {
        ARENA.with_borrow_mut(|arena| {
            let v = CString::new(v).unwrap();
            let ptr = v.into_raw();
            arena.ptrs.push(Ptr::Str(ptr));
            ptr
        })
    }

    pub(crate) fn free() {
        ARENA.with_borrow_mut(|arena| {
            while let Some(ptr) = arena.ptrs.pop() {
                match ptr {
                    Ptr::Vec { ptr, len, cap } => {
                        let _ = unsafe { Vec::from_raw_parts(ptr, len, cap) };
                    }
                    Ptr::Str(it) => {
                        let _ = unsafe { CString::from_raw(it) };
                    }
                }
            }
        })
    }
}
