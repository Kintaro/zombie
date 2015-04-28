extern crate libc;

use libc::types::os::arch::posix88::pid_t;
use libc::c_int;

const WNOHANG : c_int = 0x00000001;

extern {
    pub fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
}

pub fn collect_zombies() {
    unsafe {
        while waitpid(-1, std::ptr::null_mut(), WNOHANG) > 0 {
        }
    }
}
