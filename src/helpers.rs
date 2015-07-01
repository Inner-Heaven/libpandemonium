use std::ffi::CStr;
use libc::c_char;

pub fn from_char_to_string(from: *const c_char) -> Option<String>{
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
