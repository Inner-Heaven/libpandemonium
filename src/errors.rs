use errno::{errno, set_errno, Errno};
use libc::{c_char, c_int, size_t};
use std::fmt;
use std::error::Error as StdError;
#[derive(Debug)]
pub enum Error {
    FFI { klass:      i32,
          message:     Option<String> },
    JailStaleOperation,
    JailEmptyOperation
}
impl Error {
    pub fn last_error() -> Error{
        let e =errno();
        set_errno(e);
        Error::FFI {klass: e.0 as i32,
                    message: errno_to_string(e)}
    }
}


fn errno_to_string(errno: Errno) -> Option<String> {
    let mut buf = [0 as c_char; 1024];
    unsafe {
        if strerror_r(errno.0, buf.as_mut_ptr(), buf.len() as size_t) < 0 {
            return None;
        }
    }
    return ::helpers::from_char_to_string(buf.as_ptr());
}
extern {
    fn strerror_r(errnum: c_int, buf: *mut c_char,
                  buflen: size_t) -> c_int;
}

#[derive(Debug)]
pub struct LibError<T> {
    pub kind: T,
    pub detail: Option<String>
}

impl <T> LibError<T> {
    fn detail(&self) -> Option<String> {
        self.detail.clone()
    }
}
impl <T: StdError> StdError for LibError<T> {
    /// Since kind has to implement Error trait we can just pass funcrtion call to it
    fn description(&self) -> &str {
        &self.kind.description()
    }
    fn cause(&self) -> Option<&StdError> {
        self.kind.cause()
    }
}
impl <T: StdError> fmt::Display for LibError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.description())
    }
}

/// Generic Error From FFI calls
#[derive(Debug)]
pub struct CError {
    pub kind: i32,
    pub detail: Option<String>
}
impl StdError for CError {
    fn description(&self) -> &'static str {
        match self.kind {
            _ => "unknown error"
        }
    }
    fn cause(&self) -> Option<&StdError> {
        None
    }
}
impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.description())
    }
}

