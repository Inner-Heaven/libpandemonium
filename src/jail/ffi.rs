extern crate libjail_sys as raw;
use std::iter::Iterator;

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

pub fn name_for_jid(jid: i32) -> Option<String> {
    let c_name = unsafe { raw::jail_getname(jid) };
    return from_char_to_string(c_name);
}

pub fn jid_iter() -> NameIter {
    return NameIter::new();
}
pub struct NameIter {
    last_jid: i32
}
impl NameIter {
    pub fn new() -> NameIter {
        NameIter { last_jid: 0 }
    }
}

impl Iterator for NameIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.last_jid = self.last_jid + 1;
        return name_for_jid(self.last_jid);
    }
}
