use zfs::types::{Dataset, ZfsError, ZfsResult};

struct LibZfsHandle;
/*
#[link_args = "-lzfs"]
extern {
    /// Initialized library
    fn libzfs_init() -> *mut LibZfsHandle;
}*/

pub fn list_datasets() -> ZfsResult<Vec<Dataset>> {
	let result: ZfsResult<Vec<Dataset>>;
	result = Err(ZfsError::NotImplemented);
	return result;
}
