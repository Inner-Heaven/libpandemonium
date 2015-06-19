#![crate_name = "libpandemonium"]
#![crate_type = "dylib"]
#![cfg(freebsd)]
#![allow(non_camel_case_types)]
#![deny(warnings)]

extern crate libc;
pub use libc::{c_int, c_void};

#[test]
fn it_works() {
    assert!(true,true);
}
