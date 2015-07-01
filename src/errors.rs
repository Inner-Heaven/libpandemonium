use errno::{errno, set_errno};
#[derive(Debug)]
pub struct Error {
    klass:      i32,
    message:     Option<String>
}
impl Error {
    pub fn last_error() -> Error {
        let e =errno();
        set_errno(e);
        println!("Error {}: {}", code, e);
        return Error {klass: e.0 as i32, message: None};
    }
}
