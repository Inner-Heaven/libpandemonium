extern crate libjail_sys as raw;

use std::ffi::CStr;
use libc::{c_char};

fn from_char_to_string(from: *const c_char) -> Option<String>{
    if from.is_null() {
        return None;
    }
    let buf_name = unsafe { CStr::from_ptr(from).to_bytes() };
    if let Ok(result) = String::from_utf8(buf_name.to_vec()) {
        return Some(result);
    } else {
        return None;
    }
}

pub fn get_name(jid: i32) -> Option<String> {
    let c_name = unsafe { raw::jail_getname(jid) };
    return from_char_to_string(c_name);
}
