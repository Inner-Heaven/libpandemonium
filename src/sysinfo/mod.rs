extern crate time;
use time::Timespec;

mod wrap;

pub fn uptime() -> Timespec {
     unsafe {
        let (sec, nsec) = wrap::os_get_time(wrap::ClockId::Uptime);
        return Timespec::new(sec, nsec);
    }
}


#[test]
fn it_show_uptime() {
    let uptime = uptime();
    assert!(uptime.sec > 0);
}

