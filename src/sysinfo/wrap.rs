extern crate libc;
use self::libc::{c_int, timespec};

extern {
    fn clock_gettime(clock_id: c_int, tp: *mut libc::timespec) -> c_int;
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
pub unsafe fn os_get_time(clock_id: ClockId) -> (i64, i32) {
    let mut tv = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(clock_id as c_int, &mut tv);
    (tv.tv_sec as i64, tv.tv_nsec as i32)
}
