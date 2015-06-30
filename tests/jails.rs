extern crate pandemonium;
use pandemonium::jail::*;

#[test]
fn test_low_lever_iterator() {
    let iter = ffi::jid_iter();
    let jail_names: Vec<_> = iter.collect();
    assert_eq!(jailNames.len(), 1);
}
#[test]
fn test_crud() {
    let name = ffi::name_for_jid(1).unwrap();
    assert_eq!(name, "test0");
}
