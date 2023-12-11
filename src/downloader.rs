const USER_AGENT: &'static str = "aoc-submission-nickmass";

pub struct InputDownloader {
    session_key: Option<String>,
    http_client: reqwest::blocking::Client,
}

impl InputDownloader {
    pub fn new() -> Self {
        let session_key = std::fs::read_to_string(".session-key").ok();
        let http_client = reqwest::blocking::Client::new();

        Self {
            session_key,
            http_client,
        }
    }

    pub fn download_input_if_absent(
        &self,
        event: u32,
        day: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = std::path::PathBuf::from(format!("input/year_{}/day_{:02}.txt", event, day));
        if path.exists() {
            let input = std::fs::read_to_string(&path)?;
            Ok(input)
        } else {
            eprintln!("downloading {} day {}.", event, day);
            let session_key = self
                .session_key
                .as_ref()
                .ok_or("file '.session-key' not found")?;
            let url = format!("https://adventofcode.com/{}/day/{}/input", event, day);
            let res = self
                .http_client
                .get(&url)
                .header("cookie", format!("session={}", session_key))
                .header("user-agent", USER_AGENT)
                .send()?
                .error_for_status()?;
            let input = res.text()?;
            std::fs::create_dir_all(&path.parent().expect("input path should have parent"))?;
            std::fs::write(&path, &input)?;

            eprintln!("Sample input:");
            for line in input.lines().take(10) {
                eprintln!("{}", line);
            }

            Ok(input)
        }
    }

    pub fn submit_answer<S: AsRef<str>>(
        &self,
        answer: S,
        event: u32,
        day: u32,
        part: u32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let session_key = self.session_key.as_ref().ok_or(".session-key not found")?;

        let body = format!("level={}&answer={}", part, answer.as_ref());
        let url = format!("https://adventofcode.com/{}/day/{}/answer", event, day);
        let res = self
            .http_client
            .post(&url)
            .header("cookie", format!("session={}", session_key))
            .header("content-type", "application/x-www-form-urlencoded")
            .header("user-agent", USER_AGENT)
            .body(body)
            .send()?
            .error_for_status()?;

        let text = res.text()?;

        let mut output = false;
        for line in text.lines() {
            if line.contains("<article>") {
                output = true;
            }
            if output {
                eprintln!("{}", line);
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
