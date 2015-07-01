extern crate libjail_sys as raw;
use std::iter::Iterator;
use std::ops::Drop;
use std::ptr;
use std::ffi::{CString};
use libc::{c_uint, c_void};

use errors::Error;

pub enum ParamKey {
    Name
}
#[allow(unused_variables)]
impl ParamKey {
    fn to_string(&self) -> String {
        match *self {
            ParamKey::Name    => return "name".to_string(),
        }
    }
}

/// Used to as "safe" wat to create jailparam for the to most of libjail calls.
struct  JailParams(Vec<*mut raw::JailParam>);
impl    JailParams {
    fn add_value(&mut self, key: ParamKey, value: String) -> Result<(), Error> {
        match JailParams::jp_for_value(key, value) {
            Ok(jp) => {
                self.0.push(jp);
                return Ok(());
            },
            Err(err) => { return Err(err); }
        }
    }
    fn ask_for_value(&mut self, key: ParamKey) -> Result<(), Error> {
        match JailParams::jp_for_key(key) {
            Ok(jp) => {
                self.0.push(jp);
                return Ok(());
            },
            Err(err) => { return Err(err); }
        }
    }
    /// Get Jailparam vector for jail with name
    fn for_name(name: String) -> Result<JailParams, Error> {
        let mut jps = JailParams(Vec::new());
        match  jps.add_value(ParamKey::Name, name) {
            Ok(_)       =>  return Ok(jps),
            Err(err)    =>  return Err(err)
        }
    }

    fn jp_for_key(key: ParamKey) -> Result<*mut raw::JailParam, Error> {
        let key_c = CString::new(key.to_string()).unwrap().as_ptr();
        let jp: *mut raw::JailParam = ptr::null_mut();
        let result = unsafe {  raw::jailparam_init(jp, key_c) };
        if result == 0 {
            return Ok(jp);
        } else {
            return Err(Error::last_error());
        }
    }

    fn jp_for_value(key: ParamKey, value: String) -> Result<*mut raw::JailParam, Error> {
        match  JailParams::jp_for_key(key) {
            Ok(jp) => {
                let value_c = CString::new(value.to_string()).unwrap().as_ptr();
                let result = unsafe { raw::jailparam_import(jp, value_c) };
                if result == 0 {
                    return Ok(jp);
                } else {
                    return Err(Error::last_error());
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}
/// Since we have to call jailparam_free() everytime we done using it.
impl Drop for JailParams {
    fn drop(&mut self) {
        unsafe {
            raw::jailparam_free(self.0.as_mut_ptr() as *mut c_void, self.0.len() as c_uint);
        }
    }

}
/// Safe wrapper for jailparam_export.
fn value_from_jp(jp: *mut raw::JailParam) -> Option<String> {
    let result = unsafe { raw::jailparam_export(jp) };
    return ::helpers::from_char_to_string(result);
}

pub fn name_for_jid(jid: i32) -> Option<String> {
    let c_name = unsafe { raw::jail_getname(jid) };
    return ::helpers::from_char_to_string(c_name);
}
/// Nothing more than a shortcut to get iterator for jails.
pub fn jid_iter() -> NameIter {
    return NameIter::new();
}

/// Low leven iterator for jails. The idea is to use this iterator for every other iterator since
/// the only thing it returns is jail name.
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
