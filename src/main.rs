use std::collections::HashMap;

mod solutions;

const DEFAULT_EVENT: u32 = 2020;

enum EventSelection {
    Specific(u32),
    All,
}

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

    let downloader = InputDownloader::new();

    match event {
        EventSelection::Specific(event) => {
            if let Some(days) = events.get(&event) {
                run_event(&downloader, event, &days)
            } else {
                eprintln!("event '{}' not configured", event);
                std::process::exit(1)
            }
        }
        EventSelection::All => {
            let mut events: Vec<_> = events.into_iter().collect();
            events.sort_by_key(|e| e.0);

            for event in &events {
                run_event(&downloader, event.0, &event.1);
                println!();
                println!();
            }
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

fn run_event(downloader: &InputDownloader, event: u32, days: &[Solution]) {
    println!("Advent of Code - {}", event);
    println!();
    for day in days {
        run_day(downloader, event, day)
    }
}

fn run_day(downloader: &InputDownloader, event: u32, day: &Solution) {
    match downloader.download_input_if_absent(event, day.day) {
        Ok(input) => {
            let time_a = std::time::Instant::now();
            let a = (day.part_one)(&input).to_string();
            let time_a = time_a.elapsed();

            let time_b = std::time::Instant::now();
            let b = (day.part_two)(&input).to_string();
            let time_b = time_b.elapsed();

            if a.len() <= 25 {
                println!("{:>2}-1:{:>25}{:>10}ms", day.day, a, time_a.as_millis());
            } else {
                println!("{:>2}-1:{:>25}{:>10}ms", day.day, "", time_a.as_millis());
                println!("{}", a);
            }

            if b.len() <= 25 {
                println!("{:>2}-2:{:>25}{:>10}ms", day.day, b, time_b.as_millis());
            } else {
                println!("{:>2}-2:{:>25}{:>10}ms", day.day, "", time_b.as_millis());
                println!("{}", b);
            }
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
        let path = std::path::PathBuf::from(format!("problems/{}/day{}.txt", event, day));
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
            std::fs::create_dir_all(&path.parent().expect("problem path should have parent"))?;
            std::fs::write(&path, &input)?;

            Ok(input)
        }
    }
}
