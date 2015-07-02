use errno::{errno, set_errno, Errno};
use libc::{c_char, c_int, size_t};
#[derive(Debug)]
pub struct Error {
    pub klass:      i32,
    pub message:     Option<String>
}
impl Error {
    pub fn last_error() -> Error {
        let e =errno();
        set_errno(e);
        return Error {klass: e.0 as i32, message: errno_to_string(e)};
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
