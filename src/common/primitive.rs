use core::ffi::c_char;
use std::ffi::{CStr}; 

pub fn c_char_to_string(s: *const c_char) -> String {
    let r = unsafe{ CStr::from_ptr(s).to_str() };
    if let Ok(s) = r {
        s.to_string()
    } else {
        "".to_string()
    }
}

pub fn vec_to_string(v: &[u8]) -> String {
    let r = unsafe{ CStr::from_ptr(v.as_ptr() as *const _) };
    r.to_string_lossy().to_string()
}

pub fn str_to_vec(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}
