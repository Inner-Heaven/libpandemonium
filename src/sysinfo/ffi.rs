extern crate libc;
use self::libc::{c_int, timespec};

extern {
    fn clock_gettime(clock_id: c_int, tp: *mut libc::timespec) -> c_int;
    fn getosreldate() -> c_int;
}

#[allow(dead_code)]
pub enum ClockId {
    Virtual         = 1,
    Prof            = 2,
    Monotonic       = 4,
    Uptime          = 5,
    UptimePrecise   = 7,
    UptimeFast,
    RealtimePrecise,
    RealtimeFast,
    MonotonicPrecise,
    MonotonicFast,
    Second,
    ThreadCputimeId,
    Precess_CputimeId
}
pub fn ffi_os_gettime(clock_id: ClockId) -> Result<timespec, i32> {
    let mut tv = timespec { tv_sec: 0, tv_nsec: 0 };
    let gettime_result = unsafe { clock_gettime(clock_id as c_int, &mut tv) };
    if gettime_result == 0 {
        return Ok(tv);
    } else {
        return Err(gettime_result as i32)
    };
}
pub unsafe fn ffi_os_get_reldate() -> i32 {
    getosreldate() as i32
}
