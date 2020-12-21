use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

mod solutions;

const DEFAULT_EVENT: u32 = 2020;

enum EventSelection {
    Specific(u32),
    All,
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc::new();

fn main() {
    let events: HashMap<_, _> = vec![
        (2019, solutions::days_2019()),
        (2020, solutions::days_2020()),
    ]
    .into_iter()
    .collect();

    let mut args = std::env::args();
    let _ = args.next();
    let event = if let Some(arg) = args.next() {
        if arg.to_lowercase() == "all" {
            EventSelection::All
        } else {
            match arg.parse() {
                Ok(event) => {
                    if events.contains_key(&event) {
                        EventSelection::Specific(event)
                    } else {
                        eprintln!("event '{}' not found", event);
                        std::process::exit(1)
                    }
                }
                Err(_err) => {
                    eprintln!("invalid event '{}'", arg);
                    std::process::exit(1)
                }
            }
        }
    } else {
        EventSelection::Specific(DEFAULT_EVENT)
    };

    let day_filter: Vec<_> = args.filter_map(|arg| arg.parse().ok()).collect();

    let downloader = InputDownloader::new();

    match event {
        EventSelection::Specific(event) => {
            if let Some(days) = events.get(&event) {
                run_event(&downloader, event, &days, &day_filter);
            } else {
                eprintln!("event '{}' not configured", event);
                std::process::exit(1)
            }
        }
        EventSelection::All => {
            let mut events: Vec<_> = events.into_iter().collect();
            events.sort_by_key(|e| e.0);

            let mut overall_duration = Duration::new(0, 0);

            for event in &events {
                overall_duration += run_event(&downloader, event.0, &event.1, &day_filter);
                println!();
                println!();
            }

            println!("Overall duration{:>24}ms", overall_duration.as_millis())
        }
    }
}

type SolutionFunc = Box<dyn Fn(&str) -> Box<dyn std::fmt::Display>>;

pub struct Solution {
    day: u32,
    part_one: SolutionFunc,
    part_two: SolutionFunc,
}

impl Solution {
    pub fn new(day: u32, part_one: SolutionFunc, part_two: SolutionFunc) -> Self {
        Self {
            day,
            part_one,
            part_two,
        }
    }
}

#[macro_export]
macro_rules! solution {
    ($day:expr, $part_one:ident, $part_two:ident) => {
        Solution::new(
            $day,
            Box::new(|input| Box::new($part_one(input))),
            Box::new(|input| Box::new($part_two(input))),
        )
    };
}

fn run_event(
    downloader: &InputDownloader,
    event: u32,
    days: &[Solution],
    day_filter: &[u32],
) -> Duration {
    println!("Advent of Code - {}", event);
    println!();

    let mut total_duration = Duration::new(0, 0);

    for day in days
        .iter()
        .filter(|d| day_filter.is_empty() || day_filter.contains(&d.day))
    {
        total_duration += run_day(downloader, event, day)
    }
    println!("Total duration{:>26}ms", total_duration.as_millis());

    total_duration
}

fn run_day(downloader: &InputDownloader, event: u32, day: &Solution) -> Duration {
    match downloader.download_input_if_absent(event, day.day) {
        Ok(input) => {
            let time_part_one = Instant::now();
            GLOBAL.reset_counts();
            let part_one = (day.part_one)(&input);
            let (allocs, peak_mem) = GLOBAL.current_counts();
            let part_one = part_one.to_string();
            let time_part_one = time_part_one.elapsed();

            print_line(day.day, 1, part_one, time_part_one, allocs, peak_mem);

            let time_part_two = Instant::now();
            GLOBAL.reset_counts();
            let part_two = (day.part_two)(&input);
            let (allocs, peak_mem) = GLOBAL.current_counts();
            let part_two = part_two.to_string();
            let time_part_two = time_part_two.elapsed();

            print_line(day.day, 2, part_two, time_part_two, allocs, peak_mem);

            time_part_one + time_part_two
        }
        Err(error) => {
            eprintln!(
                "unable to get input for '{}' day '{}'. {:?}",
                event, day.day, error
            );
            std::process::exit(1)
        }
    }
}

fn print_line(
    day: u32,
    part: u32,
    answer: String,
    duration: Duration,
    allocs: u64,
    peak_mem: usize,
) {
    if answer.len() <= 25 {
        println!(
            "{:>2}-{}:{:>25}{:>10}ms{:>10} allocations {:>10} peak memory",
            day,
            part,
            answer,
            duration.as_millis(),
            allocs,
            Bytes(peak_mem)
        );
    } else {
        println!(
            "{:>2}-{}:{:>25}{:>10}ms{:>10} allocations {:>10} peak memory",
            day,
            part,
            "",
            duration.as_millis(),
            allocs,
            Bytes(peak_mem)
        );
        println!("{}", answer);
    }
}

struct InputDownloader {
    session_key: Option<String>,
    http_client: reqwest::blocking::Client,
}

impl InputDownloader {
    fn new() -> Self {
        let session_key = std::fs::read_to_string("./.session-key").ok();
        let http_client = reqwest::blocking::Client::new();

        Self {
            session_key,
            http_client,
        }
    }

    fn download_input_if_absent(
        &self,
        event: u32,
        day: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from(format!("input/{}/day{}.txt", event, day));
        if path.exists() {
            let input = std::fs::read_to_string(&path)?;
            Ok(input)
        } else {
            eprintln!("downloading {} day {}.", event, day);
            let session_key = self.session_key.as_ref().ok_or(".session-key not found")?;
            let url = format!("https://adventofcode.com/{}/day/{}/input", event, day);
            let req = self.http_client.get(&url);
            let req = req.header("cookie", format!("session={}", session_key));
            let res = req.send()?;
            let res = res.error_for_status()?;
            let input = res.text()?;
            std::fs::create_dir_all(&path.parent().expect("input path should have parent"))?;
            std::fs::write(&path, &input)?;

            Ok(input)
        }
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

#[derive(Debug, Copy, Clone)]
struct Bytes(usize);

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
