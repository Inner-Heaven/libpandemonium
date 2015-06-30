extern crate libc;
use libc::{c_char, c_void, size_t, c_int, c_uint};
use std::clone::Clone;
use std::default::Default;
#[repr(C)]
#[derive(Copy)]
pub struct Jailparam {
    pub jp_name: *mut c_char,
    pub jp_value: *mut c_void,
    pub jp_valuelen: size_t,
    pub jp_elemlen: size_t,
    pub jp_ctltype: c_int,
    pub jp_structtype: c_int,
    pub jp_flags: c_uint,
}
impl Clone for Jailparam {
    fn clone(&self) -> Self { *self }
}
impl Default for Jailparam {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub static mut jail_errmsg: *mut c_char;

    pub fn jail_getid(name: *const c_char) -> c_int;
    pub fn jail_getname(jid: c_int) -> *mut c_char;
    pub fn jail_setv(flags: c_int, ...) -> c_int;
    pub fn jail_getv(flags: c_int, ...) -> c_int;
    pub fn jailparam_all(jpp: *mut Jailparam) -> c_int;
    pub fn jailparam_init(jp: *mut Jailparam,
                          name: *const c_char) -> c_int;
    pub fn jailparam_import(jp: *mut Jailparam,
                            value: *const c_char) -> c_int;
    pub fn jailparam_import_raw(jp: *mut Jailparam,
                                value: *mut c_void, valuelen: size_t)
     -> c_int;
    pub fn jailparam_set(jp: *mut Jailparam, njp: c_uint,
                         flags: c_int) -> c_int;
    pub fn jailparam_get(jp: *mut Jailparam, njp: c_uint,
                         flags: c_int) -> c_int;
    pub fn jailparam_export(jp: *mut Jailparam) -> *mut c_char;
    pub fn jailparam_free(jp: *mut Jailparam, njp: c_uint) -> ();
}
