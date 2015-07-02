extern crate libjail_sys as raw;

use std::ffi::{CString};
use std::ops::Drop;
use std::ptr;
use libc::{c_uint, c_void, c_int};
use errors::Error;

/// Safe interface for operation with jails
pub struct JailOperation {
    /// Pointers to jailparams. never try to own them.
    jps:        Vec<*mut c_void>,
    /// After execution of operation it can't be reused.
    executed:   bool
}

impl JailOperation {
    /// Proper way of setting up Operation
    pub fn new(name: String) -> Result<JailOperation, Error> {
        let mut operation = JailOperation { jps: Vec::new(), executed: false };
        match operation.add_jailparam_with_value(&ParamKey::Name, &name) {
            Ok(_)    => return Ok(operation),
            Err(err) => return Err(err)
        }
    }
    /// executes read operation on jail and returns Vector of values for requested keys.
    /// Values returned in the same order they were requested
    pub fn get(&mut self) -> Result<Vec<Option<String>>, Error> {
        match self.jp_get() {
            Ok(_)   => {
                let mut export = Vec::with_capacity(self.jps.len());
                for jp in self.jps.iter() {
                    let ptr = jp.clone();
                    export.push(JailOperation::jp_export(ptr));
                }
                return Ok(export);
            },
            Err(err) => { return Err(err) }
        }
    }

    pub fn add_jailparams(&mut self, params: Vec<ParamKey>) -> Result<(), Error> {
        //let err: Option<Error> = None;
        for param in params.iter() {
            if let Err(err) = self.add_jailparam(param) {
                return Err(err);
            }
        }
        return Ok(());
    }
    pub fn add_jaiparams_with_values(&mut self, params: Vec<(ParamKey, String)>) -> Result<(), Error> {
        for pair in params.iter() {
            if let Err(err) = self.add_jailparam_with_value(&pair.0, &pair.1) {
                return Err(err);
            }
        }
        return Ok(());
    }

    /// Use this if you want _set_ value to jail
    pub fn add_jailparam_with_value(&mut self, key: &ParamKey, value: &String) -> Result<(), Error> {
        match JailOperation::jp_for_value(&key, &value) {
            Ok(jp) => {
                self.jps.push(jp);
                return Ok(());
            },
            Err(err) => { return Err(err); }
        }
    }
    /// Use this if you want _get_ value from jail
    pub fn add_jailparam(&mut self, key: &ParamKey) -> Result<(), Error> {
        match JailOperation::jp_for_key(key) {
            Ok(jp) => {
                self.jps.push(jp);
                return Ok(());
            },
            Err(err) => { return Err(err); }
        }
    }
    fn jp_get(&mut self) -> Result<(), Error> {
        if self.jps.len() < 2 {
            let err = Error {klass: 12, message: Some("Empty operation list".to_string()) };
            return Err(err);
        }
        let result = unsafe { raw::jailparam_set(self.jps.as_mut_ptr() as *mut c_void,
                                         self.jps.len() as c_uint, 0 as c_int) };
        if result == 0 {
            return Ok(());
        } else {
            return Err(Error::last_error());
        }
    }
    /// Shortcut to initializ jailparam and get pointer.
    fn jp_for_key(key: &ParamKey) -> Result<*mut raw::JailParam, Error> {
        let key_c = CString::new(key.to_string()).unwrap().as_ptr();
        let jp: *mut raw::JailParam = ptr::null_mut();
        let result = unsafe {  raw::jailparam_init(jp, key_c) };
        if result == 0 {
            return Ok(jp);
        } else {
            return Err(Error::last_error());
        }
    }
    /// The same as jp_for_key, but also imports value into jailparam.
    fn jp_for_value(key: &ParamKey, value: &String) -> Result<*mut raw::JailParam, Error> {
        match  JailOperation::jp_for_key(key) {
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

    /// Safe wrapper for jailparam_export.
    fn jp_export(jp: *mut c_void) -> Option<String> {
        let result = unsafe { raw::jailparam_export(jp) };
        return ::helpers::from_char_to_string(result);
    }
}
/// Since we have to call jailparam_free() everytime we done using it.
impl Drop for JailOperation {
    fn drop(&mut self) {
        unsafe {
            raw::jailparam_free(self.jps.as_mut_ptr() as *mut c_void, self.jps.len() as c_uint);
        }
    }
}

pub enum ParamKey {
    Name,
    Hostname
}
#[allow(unused_variables)]
impl ParamKey {
    fn to_string(&self) -> String {
        match *self {
            ParamKey::Name      => return "name".to_string(),
            ParamKey::Hostname  => return "hostname".to_string(),
        }
    }
}
