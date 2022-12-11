use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc::new();

#[cfg(any(not(target_os = "linux"), feature = "basic_profiler"))]
pub use fallback::Profiler;
#[cfg(all(target_os = "linux", not(feature = "basic_profiler")))]
pub use linux::Profiler;

#[cfg(all(target_os = "linux", not(feature = "basic_profiler")))]
mod linux {
    use perf_event::{events, Builder, Counter, Group};

    use super::{Duration, Metrics, GLOBAL};

    #[derive(Debug)]
    pub struct Profiler {
        perf_group: Group,
        cycle_counter: Counter,
        instruction_counter: Counter,
        clock_counter: Counter,
    }

    impl Profiler {
        pub fn new() -> Self {
            let mut perf_group = Group::new().unwrap();
            let cycle_counter = Builder::new()
                .group(&mut perf_group)
                .kind(events::Hardware::CPU_CYCLES)
                .build()
                .unwrap();
            let instruction_counter = Builder::new()
                .group(&mut perf_group)
                .kind(events::Hardware::INSTRUCTIONS)
                .build()
                .unwrap();
            let clock_counter = Builder::new()
                .group(&mut perf_group)
                .kind(events::Software::TASK_CLOCK)
                .build()
                .unwrap();

            Self {
                perf_group,
                cycle_counter,
                instruction_counter,
                clock_counter,
            }
        }

        pub fn start(&mut self) {
            self.perf_group.reset().unwrap();
            GLOBAL.reset_counts();
            self.perf_group.enable().unwrap();
        }

        pub fn stop(&mut self) -> Metrics {
            self.perf_group.disable().unwrap();
            let (allocations, peak_memory) = GLOBAL.current_counts();
            let counts = self.perf_group.read().unwrap();
            let cycles = counts[&self.cycle_counter];
            let instructions = counts[&self.instruction_counter];

            let clock = counts[&self.clock_counter];
            let duration = Duration::from_nanos(clock);

            Metrics {
                instructions,
                cycles,
                duration,
                allocations,
                peak_memory,
            }
        }
    }
}

#[cfg(any(not(target_os = "linux"), feature = "basic_profiler"))]
mod fallback {
    use super::{Metrics, GLOBAL};

    use std::time::Instant;

    #[derive(Debug)]
    pub struct Profiler {
        start_time: Instant,
    }

    impl Profiler {
        pub fn new() -> Self {
            Self {
                start_time: Instant::now(),
            }
        }

        pub fn start(&mut self) {
            GLOBAL.reset_counts();
            self.start_time = Instant::now();
        }

        pub fn stop(&mut self) -> Metrics {
            let duration = self.start_time.elapsed();
            let (allocations, peak_memory) = GLOBAL.current_counts();

            Metrics {
                duration,
                allocations,
                peak_memory,
                instructions: 0,
                cycles: 0,
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metrics {
    pub instructions: u64,
    pub cycles: u64,
    pub duration: std::time::Duration,
    pub allocations: u64,
    pub peak_memory: usize,
}

impl Metrics {
    pub fn display(&self, detailed: bool) -> MetricsDisplay {
        MetricsDisplay(detailed, &self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MetricsDisplay<'a>(bool, &'a Metrics);

impl<'a> std::fmt::Display for MetricsDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(
                f,
                "{:>12}{:>10} allocs.{:>10} mem.{:>15} cyc.{:>15} instr.",
                Time(self.1.duration),
                self.1.allocations,
                Bytes(self.1.peak_memory),
                self.1.cycles,
                self.1.instructions
            )
        } else {
            write!(
                f,
                "{:>12}{:>10} allocations{:>10} peak memory",
                Time(self.1.duration),
                self.1.allocations,
                Bytes(self.1.peak_memory)
            )
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Time(pub Duration);

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let micros = self.0.as_micros();
        let s = if micros > 2000 {
            format!("{}ms", micros / 1000)
        } else {
            format!("{}us", micros)
        };

        f.pad(s.as_str())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bytes(pub usize);

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut size = self.0 as f64;
        if size < 512.0 {
            let s = format!("{}b ", size);
            return f.pad(s.as_str());
        }
        size /= 1024.0;
        if size < 512.0 {
            let s = format!("{:.1}kb", size);
            return f.pad(s.as_str());
        }
        size /= 1024.0;
        if size < 512.0 {
            let s = format!("{:.1}mb", size);
            return f.pad(s.as_str());
        }
        size /= 1024.0;
        let s = format!("{:.1}gb", size);
        return f.pad(s.as_str());
    }
}

struct CountingAlloc {
    allocations: AtomicU64,
    peak_mem: AtomicUsize,
    current_mem: AtomicUsize,
}

impl CountingAlloc {
    const fn new() -> Self {
        CountingAlloc {
            allocations: AtomicU64::new(0),
            peak_mem: AtomicUsize::new(0),
            current_mem: AtomicUsize::new(0),
        }
    }

    fn reset_counts(&self) {
        self.allocations.store(0, Ordering::SeqCst);
        self.peak_mem.store(0, Ordering::SeqCst);
        self.current_mem.store(0, Ordering::SeqCst);
    }

    fn current_counts(&self) -> (u64, usize) {
        let allocs = self.allocations.load(Ordering::SeqCst);
        let peak_mem = self.peak_mem.load(Ordering::SeqCst);

        (allocs, peak_mem)
    }
}

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::Relaxed);
        let ret = System.alloc(layout);

        if !ret.is_null() {
            let size = layout.size();
            let current = self.current_mem.load(Ordering::Relaxed) + size;
            let peak = self.peak_mem.load(Ordering::Relaxed).max(current);
            self.current_mem.store(current, Ordering::Relaxed);
            self.peak_mem.store(peak, Ordering::Relaxed);
        }

        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let current = self
            .current_mem
            .load(Ordering::Relaxed)
            .saturating_sub(size);
        self.current_mem.store(current, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::Relaxed);
        let ret = System.alloc_zeroed(layout);

        if !ret.is_null() {
            let size = layout.size();
            let current = self.current_mem.load(Ordering::Relaxed) + size;
            let peak = self.peak_mem.load(Ordering::Relaxed).max(current);
            self.current_mem.store(current, Ordering::Relaxed);
            self.peak_mem.store(peak, Ordering::Relaxed);
        }

        ret
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::Relaxed);

        let ret = System.realloc(ptr, layout, new_size);

        if !ret.is_null() {
            let size = layout.size();
            if new_size > size {
                let diff = new_size - size;
                let current = self.current_mem.load(Ordering::Relaxed) + diff;
                let peak = self.peak_mem.load(Ordering::Relaxed).max(current);
                self.current_mem.store(current, Ordering::Relaxed);
                self.peak_mem.store(peak, Ordering::Relaxed);
            } else {
                let diff = size - new_size;
                let current = self.current_mem.load(Ordering::Relaxed) - diff;
                self.current_mem.store(current, Ordering::Relaxed);
            }
        }

        ret
    }
}
