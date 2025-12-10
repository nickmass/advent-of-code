use std::{
    cell::Cell,
    time::{Duration, Instant},
};

use crate::Input;

const USER_AGENT: &str = "aoc-submission-github/nickmass";
const REQUEST_DELAY: Duration = Duration::from_secs(15);

pub struct InputDownloader {
    input: Input,
    session_key: Option<String>,
    http_client: ureq::Agent,
    last_request_time: Cell<Option<Instant>>,
}

impl InputDownloader {
    pub fn new() -> Self {
        let session_key = std::fs::read_to_string(".session-key")
            .ok()
            .map(|s| s.trim().to_string());

        let http_client = ureq::Agent::config_builder()
            .user_agent(USER_AGENT)
            .build()
            .into();

        Self {
            input: Input::new(),
            session_key,
            http_client,
            last_request_time: Cell::new(None),
        }
    }

    pub fn download_input_if_absent(
        &self,
        event: u32,
        day: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(input) = self.input.read(event, day)? {
            Ok(input)
        } else {
            self.wait_if_needed();
            eprintln!("downloading {event} day {day}.");
            let session_key = self
                .session_key
                .as_ref()
                .ok_or("file '.session-key' not found")?;
            let url = format!("https://adventofcode.com/{event}/day/{day}/input");
            let auth = format!("session={session_key}");

            self.update_request_time();
            let res = self.http_client.get(&url).header("cookie", &auth).call()?;
            let input = res.into_body().read_to_string()?;

            self.input.save(event, day, &input)?;

            eprintln!("Sample input:");
            for line in input.lines().take(10) {
                eprintln!("{line}");
            }

            Ok(input)
        }
    }

    fn wait_if_needed(&self) {
        if let Some(last_req) = self.last_request_time.get() {
            let mut logged = false;
            while last_req.elapsed() < REQUEST_DELAY {
                if !logged {
                    logged = true;
                    let dur = REQUEST_DELAY
                        .saturating_sub(last_req.elapsed())
                        .max(Duration::from_secs(1));
                    eprintln!("waiting {} seconds between requests...", dur.as_secs())
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }

    fn update_request_time(&self) {
        self.last_request_time.set(Some(Instant::now()));
    }

    pub fn submit_answer<S: AsRef<str>>(
        &self,
        answer: S,
        event: u32,
        day: u32,
        part: u32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.wait_if_needed();
        let session_key = self
            .session_key
            .as_ref()
            .ok_or("file '.session-key' not found")?;

        let url = format!("https://adventofcode.com/{event}/day/{day}/answer");
        let auth = format!("session={session_key}");
        let part = part.to_string();

        self.update_request_time();
        let res = self
            .http_client
            .post(&url)
            .header("cookie", &auth)
            .send_form([("level", part.as_ref()), ("answer", answer.as_ref())])?;
        let text = res.into_body().read_to_string()?;

        let mut output = false;
        for line in text.lines() {
            if line.contains("<article>") {
                output = true;
            }
            if output {
                eprintln!("{line}");
            }
            if line.contains("</article>") {
                output = false;
            }
        }

        if text.contains("That's the right answer") {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
