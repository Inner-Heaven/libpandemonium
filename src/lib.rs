#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
#![deny(warnings)]
#![allow(dead_code)]

extern crate time;

#[cfg(feature = "sysinfo")]
pub mod sysinfo;
#[cfg(feature = "zfs")]
pub mod zfs;
