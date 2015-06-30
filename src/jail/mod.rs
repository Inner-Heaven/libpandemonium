//! # libjail binding for Rust.
//!
//! Used to manipulate jails in system.
mod prison;
pub mod ffi;
pub use jail::prison::*;
