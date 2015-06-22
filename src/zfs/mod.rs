
mod types;
#[allow(unused_imports)]
use zfs::types::*;

#[cfg(feature = "zfs_open3")]
mod open3;
#[cfg(feature = "zfs_open3")]
pub use zfs::open3::*;


#[cfg(feature = "zfs_libzfs")]
mod libzfs;
#[cfg(feature = "zfs_libzfs")]
pub use zfs::libzfs::*;


#[test]
fn it_should_list_datasets() {
    let datasets_result = list_datasets();
    assert_eq!(datasets_result.is_ok(), true);
    let datasets: Vec<Dataset> = datasets_result.unwrap();
    assert_eq!(datasets.len(), 4);
    assert_eq!(datasets[0].name, "tank".to_string());
    assert_eq!(datasets[0].mounted, false);
}
