use crate::encryption::Encryption;

pub struct Input {
    encryption: Option<Encryption>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            encryption: Encryption::new(),
        }
    }

    pub fn read(&self, event: u32, day: u32) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if let Some(enc) = self.encryption.as_ref() {
            let path = std::path::PathBuf::from(format!("input/year_{}/day_{:02}.aes", event, day));

            if path.exists() {
                let data = std::fs::read(path)?;

                let bytes = enc.decrypt_day(event, day, data)?;
                let input = String::from_utf8(bytes)?;

                return Ok(Some(input));
            }
        }

        let path = std::path::PathBuf::from(format!("input/year_{}/day_{:02}.txt", event, day));

        if path.exists() {
            let input = std::fs::read_to_string(&path)?;

            Ok(Some(input))
        } else {
            Ok(None)
        }
    }

    pub fn save(
        &self,
        event: u32,
        day: u32,
        input: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (bytes, path) = if let Some(enc) = self.encryption.as_ref() {
            let path = std::path::PathBuf::from(format!("input/year_{}/day_{:02}.aes", event, day));

            let data = input.as_bytes().to_vec();
            let data = enc.encrypt_day(event, day, data)?;

            (data, path)
        } else {
            let path = std::path::PathBuf::from(format!("input/year_{}/day_{:02}.txt", event, day));

            let data = input.as_bytes().to_vec();
            (data, path)
        };

        std::fs::create_dir_all(&path.parent().expect("input path should have parent"))?;
        std::fs::write(&path, &bytes)?;

        Ok(())
    }
}
