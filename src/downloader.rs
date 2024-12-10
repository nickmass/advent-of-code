use crate::Input;

const USER_AGENT: &str = "aoc-submission-nickmass";

pub struct InputDownloader {
    input: Input,
    session_key: Option<String>,
    http_client: ureq::Agent,
}

impl InputDownloader {
    pub fn new() -> Self {
        let session_key = std::fs::read_to_string(".session-key")
            .ok()
            .map(|s| s.trim().to_string());

        let http_client = ureq::builder().user_agent(USER_AGENT).build();

        Self {
            input: Input::new(),
            session_key,
            http_client,
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
            eprintln!("downloading {} day {}.", event, day);
            let session_key = self
                .session_key
                .as_ref()
                .ok_or("file '.session-key' not found")?;
            let url = format!("https://adventofcode.com/{}/day/{}/input", event, day);
            let auth = format!("session={}", session_key);
            let res = self.http_client.get(&url).set("cookie", &auth).call()?;

            let input = res.into_string()?;
            self.input.save(event, day, &input)?;

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
        let session_key = self
            .session_key
            .as_ref()
            .ok_or("file '.session-key' not found")?;

        let url = format!("https://adventofcode.com/{}/day/{}/answer", event, day);
        let auth = format!("session={}", session_key);
        let res = self
            .http_client
            .post(&url)
            .set("cookie", &auth)
            .send_form(&[("level", &part.to_string()), ("answer", answer.as_ref())])?;
        let text = res.into_string()?;

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
