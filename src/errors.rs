/// NotFound - jail with givin id was not found
#[derive(Debug)]
pub enum JailError {
    JailNotFound,
    ParamNotFound
}
