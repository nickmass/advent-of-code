use std::io::Write;
use std::time::Duration;

pub use ahash::{AHashMap as HashMap, AHashSet as HashSet};

mod downloader;
mod profiler;
mod solutions;

use downloader::InputDownloader;
use profiler::{Metrics, Profiler};

const DEFAULT_EVENT: u32 = 2021;

enum EventSelection {
    Specific(u32),
    All,
}

fn main() {
    let events: HashMap<_, _> = vec![
        (2019, solutions::days_2019()),
        (2020, solutions::days_2020()),
        (2021, solutions::days_2021()),
    ]
    .into_iter()
    .collect();

    let mut args = std::env::args();
    let _ = args.next();

    let first = args.next();

    let (testing, event) = if first == Some(String::from("--submit")) {
        (true, args.next())
    } else {
        (false, first)
    };

    let event = if let Some(arg) = event {
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
    let mut profiler = Profiler::new();

    match event {
        EventSelection::Specific(event) => {
            if let Some(days) = events.get(&event) {
                run_event(
                    &downloader,
                    &mut profiler,
                    event,
                    &days,
                    &day_filter,
                    testing,
                );
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
                overall_duration += run_event(
                    &downloader,
                    &mut profiler,
                    event.0,
                    &event.1,
                    &day_filter,
                    testing,
                );
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
    ($day:expr, $part_one:path, $part_two:path) => {
        Solution::new(
            $day,
            Box::new(|input| Box::new($part_one(input))),
            Box::new(|input| Box::new($part_two(input))),
        )
    };
}

fn run_event(
    downloader: &InputDownloader,
    profiler: &mut Profiler,
    event: u32,
    days: &[Solution],
    day_filter: &[u32],
    testing: bool,
) -> Duration {
    println!("Advent of Code - {}", event);
    println!();

    let mut total_duration = Duration::new(0, 0);

    for day in days
        .iter()
        .filter(|d| day_filter.is_empty() || day_filter.contains(&d.day))
    {
        total_duration += run_day(downloader, profiler, event, day, testing)
    }
    println!("Total duration{:>26}ms", total_duration.as_millis());

    total_duration
}

fn run_day(
    downloader: &InputDownloader,
    profiler: &mut Profiler,
    event: u32,
    day: &Solution,
    testing: bool,
) -> Duration {
    match downloader.download_input_if_absent(event, day.day) {
        Ok(input) => {
            profiler.start();
            let part_one = (day.part_one)(&input);
            let part_one_metrics = profiler.stop();
            let part_one = part_one.to_string();

            print_line(day.day, 1, &part_one, &part_one_metrics);

            profiler.start();
            let part_two = (day.part_two)(&input);
            let part_two_metrics = profiler.stop();
            let part_two = part_two.to_string();

            print_line(day.day, 2, &part_two, &part_two_metrics);

            if testing {
                submit_day_part(downloader, event, day, 1, &part_one);
                submit_day_part(downloader, event, day, 2, &part_two);
            }

            part_one_metrics.duration + part_two_metrics.duration
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

fn submit_day_part(
    downloader: &InputDownloader,
    event: u32,
    day: &Solution,
    part: u32,
    answer: &str,
) {
    print!("Submit part {}? [ycN] ", part);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    let entry = buffer.trim().to_ascii_lowercase();
    let answer = if entry == "c" {
        print!("Enter custom submission: ");
        std::io::stdout().flush().unwrap();
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        Some(buffer.trim())
    } else if entry == "y" {
        Some(answer.into())
    } else {
        None
    };

    if let Some(answer) = answer {
        let res = downloader.submit_answer(answer, event, day.day, part);

        match res {
            Ok(true) => println!("Correct"),
            Ok(false) => println!("Incorrect"),
            Err(err) => {
                eprintln!(
                    "unable to submit answer for '{}' day '{}' part '{}'. {:?}",
                    event, day.day, part, err
                );
            }
        }
    }
}

fn print_line<S: AsRef<str>>(day: u32, part: u32, answer: S, metrics: &Metrics) {
    let answer = answer.as_ref();
    if answer.len() <= 25 {
        println!(
            "{:>2}-{}:{:>25}{:>10}ms{:>10} allocations {:>10} peak memory",
            day,
            part,
            answer,
            metrics.duration.as_millis(),
            metrics.allocations,
            Bytes(metrics.peak_memory),
        );
    } else {
        println!(
            "{:>2}-{}:{:>25}{:>10}ms{:>10} allocations {:>10} peak memory",
            day,
            part,
            "",
            metrics.duration.as_millis(),
            metrics.allocations,
            Bytes(metrics.peak_memory)
        );
        println!("{}", answer);
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
