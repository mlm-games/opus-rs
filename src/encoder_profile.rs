// Internal profiling module for encoder
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

/// Global profiling counters
pub static COUNTERS: ProfileCounters = ProfileCounters {
    mdct_forward_calls: AtomicUsize::new(0),
    mdct_forward_time: AtomicU64::new(0),
    
    compute_band_energies_calls: AtomicUsize::new(0),
    compute_band_energies_time: AtomicU64::new(0),
    
    quant_coarse_energy_calls: AtomicUsize::new(0),
    quant_coarse_energy_time: AtomicU64::new(0),
    
    quant_all_bands_calls: AtomicUsize::new(0),
    quant_all_bands_time: AtomicU64::new(0),
    
    pvq_search_calls: AtomicUsize::new(0),
    pvq_search_time: AtomicU64::new(0),
    
    icwrs_calls: AtomicUsize::new(0),
    icwrs_time: AtomicU64::new(0),
    icwrs_ncwrs_fallbacks: AtomicUsize::new(0),
    
    encode_pulses_calls: AtomicUsize::new(0),
    encode_pulses_time: AtomicU64::new(0),
};

pub struct ProfileCounters {
    pub mdct_forward_calls: AtomicUsize,
    pub mdct_forward_time: AtomicU64,
    
    pub compute_band_energies_calls: AtomicUsize,
    pub compute_band_energies_time: AtomicU64,
    
    pub quant_coarse_energy_calls: AtomicUsize,
    pub quant_coarse_energy_time: AtomicU64,
    
    pub quant_all_bands_calls: AtomicUsize,
    pub quant_all_bands_time: AtomicU64,
    
    pub pvq_search_calls: AtomicUsize,
    pub pvq_search_time: AtomicU64,
    
    pub icwrs_calls: AtomicUsize,
    pub icwrs_time: AtomicU64,
    pub icwrs_ncwrs_fallbacks: AtomicUsize,
    
    pub encode_pulses_calls: AtomicUsize,
    pub encode_pulses_time: AtomicU64,
}

impl ProfileCounters {
    pub fn reset(&self) {
        self.mdct_forward_calls.store(0, Ordering::Relaxed);
        self.mdct_forward_time.store(0, Ordering::Relaxed);
        self.compute_band_energies_calls.store(0, Ordering::Relaxed);
        self.compute_band_energies_time.store(0, Ordering::Relaxed);
        self.quant_coarse_energy_calls.store(0, Ordering::Relaxed);
        self.quant_coarse_energy_time.store(0, Ordering::Relaxed);
        self.quant_all_bands_calls.store(0, Ordering::Relaxed);
        self.quant_all_bands_time.store(0, Ordering::Relaxed);
        self.pvq_search_calls.store(0, Ordering::Relaxed);
        self.pvq_search_time.store(0, Ordering::Relaxed);
        self.icwrs_calls.store(0, Ordering::Relaxed);
        self.icwrs_time.store(0, Ordering::Relaxed);
        self.icwrs_ncwrs_fallbacks.store(0, Ordering::Relaxed);
        self.encode_pulses_calls.store(0, Ordering::Relaxed);
        self.encode_pulses_time.store(0, Ordering::Relaxed);
    }
    
    pub fn report(&self, num_frames: usize) {
        eprintln!("\n=== Encoder Profile Report ({} frames) ===", num_frames);
        
        let report_line = |name: &str, calls: usize, time_ns: u64| {
            let time_us = time_ns as f64 / 1000.0;
            let avg_us = if calls > 0 { time_us / calls as f64 } else { 0.0 };
            let per_frame_us = time_us / num_frames as f64;
            eprintln!("  {:25}: {:8} calls, {:8.1} µs total, {:6.2} µs/call, {:6.2} µs/frame",
                name, calls, time_us, avg_us, per_frame_us);
        };
        
        report_line("MDCT forward",
            self.mdct_forward_calls.load(Ordering::Relaxed),
            self.mdct_forward_time.load(Ordering::Relaxed));
        
        report_line("Compute band energies",
            self.compute_band_energies_calls.load(Ordering::Relaxed),
            self.compute_band_energies_time.load(Ordering::Relaxed));
        
        report_line("Quant coarse energy",
            self.quant_coarse_energy_calls.load(Ordering::Relaxed),
            self.quant_coarse_energy_time.load(Ordering::Relaxed));
        
        report_line("Quant all bands",
            self.quant_all_bands_calls.load(Ordering::Relaxed),
            self.quant_all_bands_time.load(Ordering::Relaxed));
        
        report_line("PVQ search",
            self.pvq_search_calls.load(Ordering::Relaxed),
            self.pvq_search_time.load(Ordering::Relaxed));
        
        report_line("icwrs",
            self.icwrs_calls.load(Ordering::Relaxed),
            self.icwrs_time.load(Ordering::Relaxed));
        
        let fallback_rate = if self.icwrs_calls.load(Ordering::Relaxed) > 0 {
            (self.icwrs_ncwrs_fallbacks.load(Ordering::Relaxed) as f64 / 
             self.icwrs_calls.load(Ordering::Relaxed) as f64) * 100.0
        } else { 0.0 };
        eprintln!("    icwrs ncwrs fallback rate: {:.1}%", fallback_rate);
        
        report_line("encode_pulses",
            self.encode_pulses_calls.load(Ordering::Relaxed),
            self.encode_pulses_time.load(Ordering::Relaxed));
    }
}

/// Profile a function call
#[inline(always)]
pub fn profile_call<T>(counter: &AtomicUsize, timer: &AtomicU64, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed().as_nanos() as u64;
    counter.fetch_add(1, Ordering::Relaxed);
    timer.fetch_add(elapsed, Ordering::Relaxed);
    result
}

/// Macro for easy profiling
#[macro_export]
macro_rules! profile {
    ($counter_name:ident, $expr:expr) => {
        $crate::encoder_profile::profile_call(
            &$crate::encoder_profile::COUNTERS.$counter_name,
            &$crate::encoder_profile::COUNTERS.$counter_name,
            || $expr
        )
    };
}
