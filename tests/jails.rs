extern crate pandemonium;
use pandemonium::jail::*;

#[test]
fn test_crud() {
    let name = ffi::get_name(1).unwrap();
    assert_eq!(name, "test0");
}
