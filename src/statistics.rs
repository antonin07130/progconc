extern crate libc;

use std::time::Duration;
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct Rusage {
    pub utime: Duration,
    pub stime: Duration,
    pub maxrss: i64,
}

impl Rusage {
    pub fn New() -> Rusage {
        let measure: (libc::timeval, libc::timeval, i64) = unsafe {
            let mut out: libc::rusage = mem::zeroed();
            libc::getrusage(libc::RUSAGE_SELF, &mut out);
            (out.ru_utime, out.ru_stime, out.ru_maxrss )
        };

        let utime : Duration = Duration::new(measure.0.tv_sec as u64, measure.0.tv_usec as u32);
        let stime : Duration = Duration::new(measure.1.tv_sec as u64, measure.1.tv_usec as u32);

        Rusage { utime, stime, maxrss: measure.2 }
    }

    pub fn get_maxrss_as_MB(&self) -> f32 {
        self.get_maxrss_as_kB() / 1024.
    }

    pub fn get_maxrss_as_kB(&self) -> f32 {
        self.maxrss as f32 / 1024.
    }
}

