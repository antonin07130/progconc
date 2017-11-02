extern crate libc;

use std::time::Duration;
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct PerfMeasure {
    pub utime: Duration,
    pub stime: Duration,
    pub maxrss: i64,
    pub clock_t: u64,
}

impl PerfMeasure {
    pub fn New() -> PerfMeasure {
        let measure: (libc::timeval, libc::timeval, i64) = unsafe {
            let mut out: libc::rusage = mem::zeroed();
            libc::getrusage(libc::RUSAGE_SELF, &mut out);
            (out.ru_utime, out.ru_stime, out.ru_maxrss )
        };

        let clock_t = unsafe { clock() };

        let utime : Duration = Duration::new(measure.0.tv_sec as u64, measure.0.tv_usec as u32);
        let stime : Duration = Duration::new(measure.1.tv_sec as u64, measure.1.tv_usec as u32);

        PerfMeasure { utime, stime, maxrss: measure.2, clock_t }
    }

    pub fn get_maxrss_as_MB(&self) -> f32 {
        self.get_maxrss_as_kB() / 1024.
    }

    pub fn get_maxrss_as_kB(&self) -> f32 {
        self.maxrss as f32 / 1024.
    }
}

extern {
    pub fn clock() -> libc::clock_t;
}



#[cfg(test)]
mod tests {
    use statistics;


    #[test]
    fn test_stats() {
        let new_measure = statistics::PerfMeasure::New();
        println!("Testing memory usage measure {}MB", new_measure.get_maxrss_as_MB());
        println!("Testing system time measure {:?}", new_measure.stime);
        println!("Testing user time measure {:?}", new_measure.utime);
    }


    #[test]
    fn test_stats_2_pts() {
        let measure_1 = statistics::PerfMeasure::New();

        let mut j = 123456;
        for i in 1..100 {
            j = j+i;
        }
        println!("dummy computation result : {}", j);

        let measure_2 = statistics::PerfMeasure::New();

        assert!(measure_2.stime.gt(&measure_1.stime));
        assert!(measure_2.utime.gt(&measure_1.utime));
        assert!(measure_2.get_maxrss_as_kB() > measure_1.get_maxrss_as_kB());
    }
}
