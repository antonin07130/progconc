extern crate libc;

use std::fmt;
use std::cmp;
use std::time::{Duration, Instant};
use std::mem;

#[derive(Debug, Copy, Clone)]
pub struct PerfMeasure {
    pub top : Instant,
    pub utime: Duration,
    pub stime: Duration,
    pub maxrss: i64,
    pub clock_t: u64,
}

impl PerfMeasure {
    pub fn new() -> PerfMeasure {
        // measures using posix functions
        let measure: (libc::timeval, libc::timeval, i64) = unsafe {
            let mut out: libc::rusage = mem::zeroed();
            libc::getrusage(libc::RUSAGE_SELF, &mut out);
            (out.ru_utime, out.ru_stime, out.ru_maxrss )
        };

        let clock_t = unsafe { clock() };

        let utime : Duration = Duration::new(measure.0.tv_sec as u64, measure.0.tv_usec as u32 * 1000);
        let stime : Duration = Duration::new(measure.1.tv_sec as u64, measure.1.tv_usec as u32 * 1000);

        let top : Instant = Instant::now();
        PerfMeasure {top, utime, stime, maxrss: measure.2, clock_t}
    }

    pub fn get_maxrss_as_megabytes(&self) -> f32 {
        self.get_maxrss_as_kilobytes() / 1024.
    }

    pub fn get_maxrss_as_kilobytes(&self) -> f32 {
        self.maxrss as f32 / 1024.
    }


    pub fn minus(&self, other: &PerfMeasure) -> PerfResult {
        PerfResult::new(other, self)
    }
}

extern {
    pub fn clock() -> libc::clock_t;
}


#[derive(Debug, Copy, Clone)]
pub struct PerfResult {
    pub time: Duration,
    pub utime: Duration,
    pub stime: Duration,
    pub maxrss: f64,
    pub clock_t: u64,
}

impl PerfResult {
    pub fn new(mes1: &PerfMeasure, mes2: &PerfMeasure) -> PerfResult {
        PerfResult {
            time: mes2.top.duration_since(mes1.top),
            utime: mes2.utime - mes1.utime,
            stime: mes2.stime - mes1.stime,
            maxrss: cmp::max(mes2.maxrss, mes1.maxrss) as f64,
            clock_t: (mes2.clock_t - mes1.clock_t),
        }
    }


    pub fn take_3_median_results(measures: &[PerfResult]) -> [PerfResult; 3] {
        let mut extract_sort = measures.iter()
            .map(|m| { (m.time, *m) })
            .collect::<Vec<_>>();
        extract_sort.sort_by_key(|p| { p.0 });

        [extract_sort[1].1, extract_sort[2].1, extract_sort[3].1]
    }

    pub fn get_maxrss_as_kilobytes(&self) -> f32 {
        self.maxrss as f32 / 1024.
    }


    pub fn compute_mean_result(perf_results: &[PerfResult]) -> PerfResult {
        assert_eq!(perf_results.len(), 3);

        let sum: PerfResult = perf_results.iter().fold(
            PerfResult { time: Duration::from_millis(0), utime: Duration::from_millis(0), stime: Duration::from_millis(0), maxrss: 0., clock_t: 0 },
            |mut acc, mes| {
                acc.time += mes.time;
                acc.clock_t += mes.clock_t;
                acc.maxrss += mes.maxrss;
                acc.stime += mes.stime;
                acc.utime += mes.utime;
                acc
            });

        PerfResult {
            time: sum.time / 3,
            utime: sum.utime / 3,
            stime: sum.stime / 3,
            maxrss: sum.maxrss as f64 / 3_f64,
            clock_t: sum.clock_t / 3,
        }
    }
}

impl fmt::Display for PerfResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PerfResult : {{ \n  time : {:?},\n  utime : {:?},\n  stime : {:?},\n  maxrss : {}kB,\n  clock_t : {} ticks }}",
               self.time, self.utime, self.stime, self.get_maxrss_as_kilobytes(), self.clock_t)
    }
}


#[cfg(test)]
mod tests {
    use statistics;


    #[test]
    fn test_stats() {
        let new_measure = statistics::PerfMeasure::new();
        println!("Testing memory usage measure {}MB", new_measure.get_maxrss_as_megabytes());
        println!("Testing system time measure {:?}", new_measure.stime);
        println!("Testing user time measure {:?}", new_measure.utime);
    }


    #[test]
    fn test_stats_2_pts() {
        let measure_1 = statistics::PerfMeasure::new();

        let mut j = 123456;
        for i in 1..100 {
            j = j+i;
        }
        println!("dummy computation result : {}", j);

        let measure_2 = statistics::PerfMeasure::new();

        assert!(measure_2.stime.gt(&measure_1.stime));
        assert!(measure_2.utime.gt(&measure_1.utime));
        assert!(measure_2.get_maxrss_as_kilobytes() > measure_1.get_maxrss_as_kilobytes());
    }
}
