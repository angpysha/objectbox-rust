#![allow(dead_code)]
use crate::c;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null;

// not using bindgen's derived #define OBX_NOT_FOUND 404, because it's a u32
pub const NOT_FOUND_404: i32 = 404;
pub const SUCCESS_0: i32 = 0;
pub const QUERY_NO_OP: i32 = 0;

pub type MutConstVoidPtr = *mut *const c_void;
pub type ConstVoidPtr = *const c_void;
pub type PtrConstChar = *const ::std::os::raw::c_char;

/// Convert a Rust `&str` to a `CString`.
/// The caller must keep the returned `CString` alive for as long as
/// the pointer (obtained via `.as_ptr()`) is needed.
pub(crate) fn to_c_string(s: &str) -> Option<CString> {
    match CString::new(s) {
        Ok(c_str) => Some(c_str),
        Err(err) => {
            eprintln!("CString conversion error: {err}");
            None
        }
    }
}

/// DEPRECATED: This function returns a dangling pointer!
/// The CString is dropped before the pointer can be used.
/// Use `to_c_string()` instead and keep the CString alive.
#[deprecated(note = "Returns dangling pointer. Use to_c_string() instead.")]
pub(crate) fn as_c_char_ptr(s: &str) -> *const c_char {
    let out_path = String::from(s);
    match CString::new(out_path.as_str()) {
        Ok(c_str) => c_str.as_ptr() as *const c_char,
        Err(err) => {
            eprintln!("{err}");
            null()
        }
    }
}

pub fn test_fn_ptr_on_char_ptr(c_ptr: PtrConstChar, fn_ptr: fn(String) -> bool) -> bool {
    // allow panic here, it's for testing anyway
    if c_ptr.is_null() {
        panic!("Encountered a null ptr");
    }

    let mut out_str = String::new();
    unsafe {
        let c_str = CStr::from_ptr(c_ptr); // assuming the ptr ends with '\0'
        match c_str.to_str() {
            // allow panic here, it's for testing anyway
            Err(err) => panic!("{err}"),
            Ok(s) => out_str.push_str(s),
        }
        fn_ptr(out_str)
    }
}

pub trait ToCChar {
    fn as_c_char_ptr(&self) -> *const c_char;
}

impl ToCChar for String {
    fn as_c_char_ptr(&self) -> *const c_char {
        as_c_char_ptr(self)
    }
}

impl ToCChar for Path {
    fn as_c_char_ptr(&self) -> *const c_char {
        as_c_char_ptr(self.to_str().unwrap())
    }
}

pub(crate) trait ToCVoid {
    fn to_const_c_void(&self) -> *const c_void;
}

impl ToCVoid for Vec<u8> {
    fn to_const_c_void(&self) -> *const c_void {
        let sl = self.as_slice();
        sl.as_ptr() as *const c_void
    }
}

pub(crate) trait VecToPtrAndLength {
    fn as_ptr_and_length_tuple<T>(&self) -> (*const T, usize);
}

impl VecToPtrAndLength for Vec<i64> {
    fn as_ptr_and_length_tuple<T>(&self) -> (*const T, usize) {
        (self.as_ptr() as *const T, self.len())
    }
}

impl VecToPtrAndLength for Vec<c::obx_qb_cond> {
    fn as_ptr_and_length_tuple<T>(&self) -> (*const T, usize) {
        (self.as_ptr() as *const T, self.len())
    }
}

impl VecToPtrAndLength for Vec<u8> {
    fn as_ptr_and_length_tuple<T>(&self) -> (*const T, usize) {
        (self.as_ptr() as *const T, self.len())
    }
}

impl VecToPtrAndLength for Vec<&CStr> {
    fn as_ptr_and_length_tuple<T>(&self) -> (*const T, usize) {
        (self.as_ptr() as *const T, self.len())
    }
}
