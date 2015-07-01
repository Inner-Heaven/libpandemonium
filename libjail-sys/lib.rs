extern crate libc;
use libc::{c_char, c_void, size_t, c_int, c_uint};
/// Never try to get hold of this unless you know what are you doing
pub type JailParam = c_void;
extern "C" {
    pub static mut jail_errmsg: *mut c_char;

    pub fn jail_getid(name: *const c_char) -> c_int;
    pub fn jail_getname(jid: c_int) -> *mut c_char;
    pub fn jail_setv(flags: c_int, ...) -> c_int;
    pub fn jail_getv(flags: c_int, ...) -> c_int;
    pub fn jailparam_all(jpp: *mut c_void) -> c_int;
    pub fn jailparam_init(jp: *mut JailParam,
                          name: *const c_char) -> c_int;
    pub fn jailparam_import(jp: *mut JailParam,
                            value: *const c_char) -> c_int;
    pub fn jailparam_import_raw(jp: *mut c_void,
                                value: *mut c_void, valuelen: size_t)
     -> c_int;
    pub fn jailparam_set(jp: *mut c_void, njp: c_uint,
                         flags: c_int) -> c_int;
    pub fn jailparam_get(jp: *mut c_void, njp: c_uint,
                         flags: c_int) -> c_int;
    pub fn jailparam_export(jp: *mut JailParam) -> *mut c_char;
    pub fn jailparam_free(jp: *mut c_void, njp: c_uint) -> ();
}

#[allow(unused)]
pub enum Flag {
    Get     = 0x00,
    Create  = 0x01,
    Update  = 0x02
}
