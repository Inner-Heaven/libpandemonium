extern crate libjail_sys as raw;
use std::iter::Iterator;

pub mod operation;
pub use self::operation::*;

pub fn name_for_jid(jid: i32) -> Option<String> {
    let c_name = unsafe { raw::jail_getname(jid) };
    return ::helpers::from_char_to_string(c_name);
}
/// Nothing more than a shortcut to get iterator for jails.
pub fn jid_iter() -> NameIter {
    return NameIter::new();
}

/// Low leven iterator for jails. The idea is to use this iterator for every other iterator since
/// the only thing it returns is jail name.
pub struct NameIter {
    last_jid: i32
}
impl NameIter {
    pub fn new() -> NameIter {
        NameIter { last_jid: 0 }
    }
}

impl Iterator for NameIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.last_jid = self.last_jid + 1;
        return name_for_jid(self.last_jid);
    }
}
