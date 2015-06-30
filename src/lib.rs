//! # FreebSD system libraries binding
//!
//! Technically should seprate create per lib, but right now it's easy that way. Also, it does
//! pretty much nothing and mostly used by me to learn rust.:w

#![allow(non_camel_case_types)]
#![deny(warnings)]
#![allow(dead_code)]

extern crate time;
extern crate libc;
extern crate errno;
pub mod errors;

#[cfg(feature = "sysinfo")]
pub mod sysinfo;
#[cfg(feature = "jail")]
pub mod jail;

