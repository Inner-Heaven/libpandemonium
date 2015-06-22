extern crate time;
use time::Timespec;

use std::process::{Command};

mod ffi;
use sysinfo::ffi::*;

/// Rust wrapper around os_gettime
pub fn os_get_time(clock_id: ClockId) -> Result<Timespec, i32> {
    match ffi_os_gettime(clock_id) {
        Ok(tv) => {
            Ok(Timespec::new(tv.tv_sec as i64, tv.tv_nsec as i32))
        }
        Err(e) => {
            Err(e)
        }
    }
}
/// Shows system uptime using default system clock
pub fn uptime() -> Timespec {
    return os_get_time(ClockId::Uptime).unwrap();
}

/// Returns Kernel version using getosreldate() call.
/// If OSVERSION enviroment varible is set. it will override the getosreldate() return value.
/// Returns -1 in case of error.
pub fn get_kernel_version() -> i32 {
    unsafe {
        return ffi_os_get_reldate()
    }
}

/// Return Userland version using `uname -U` command.
/// The only possible way of getting this value...Sorry Darth Lis...

pub fn get_userland_version() -> i32 {
    let cmd = Command::new("uname")
        .arg("-U")
        .output();
    match cmd {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8(result.stdout).unwrap();
                let ver = stdout.trim().parse().unwrap();
                return ver;
            } else {
                -1
            }
        }
        Err(_) => {
            -1
        }
    }
}

#[test]
fn it_show_uptime() {
    let uptime = uptime();
    assert!(uptime.sec > 0);
}


#[test]
fn it_should_show_kernel_version() {
    let kernver = get_kernel_version();
    assert!(kernver > 1);
}
#[test]
fn it_should_show_userland_version() {
    let userver = get_userland_version();
    assert!(userver > 1);
}
#[test]
fn system_is_not_fucked() {
    let userver = get_userland_version();
    let kernver = get_kernel_version();
    assert!(userver <= kernver);
}
