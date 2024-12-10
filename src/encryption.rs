use base64::prelude::*;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_128_GCM};
use ring::error::Unspecified;

pub struct Encryption {
    key_bytes: Vec<u8>,
}

const KEYFILE_PATH: &str = ".input-key";
const KEYFILE_ENV: &str = "AOC_INPUT_KEY";

#[derive(Debug, Copy, Clone)]
pub struct EncryptError;

impl From<Unspecified> for EncryptError {
    fn from(_value: Unspecified) -> Self {
        EncryptError
    }
}

impl std::fmt::Display for EncryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unspecified")
    }
}

impl std::error::Error for EncryptError {}

impl Encryption {
    pub fn new() -> Option<Self> {
        let key_str = std::env::var(KEYFILE_ENV).ok().or(read_keyfile())?;
        let key_bytes = BASE64_STANDARD.decode(key_str.trim()).unwrap();
        let _key = UnboundKey::new(&AES_128_GCM, &key_bytes).unwrap();

        Some(Self { key_bytes })
    }

    pub fn encrypt_day(
        &self,
        event: u32,
        day: u32,
        mut data: Vec<u8>,
    ) -> Result<Vec<u8>, EncryptError> {
        let nonce = day_nonce(event, day);
        let key =
            UnboundKey::new(&AES_128_GCM, &self.key_bytes).expect("key validated in constructor");
        let key = LessSafeKey::new(key);

        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut data)?;

        Ok(data)
    }

    pub fn decrypt_day(
        &self,
        event: u32,
        day: u32,
        mut data: Vec<u8>,
    ) -> Result<Vec<u8>, EncryptError> {
        let nonce = day_nonce(event, day);
        let key =
            UnboundKey::new(&AES_128_GCM, &self.key_bytes).expect("key validated in constructor");
        let key = LessSafeKey::new(key);

        let plaintext = key.open_in_place(nonce, Aad::empty(), &mut data)?;
        let len = plaintext.len();
        data.truncate(len);

        Ok(data)
    }
}

fn day_nonce(event: u32, day: u32) -> Nonce {
    let event = event.to_le_bytes();
    let day = day.to_le_bytes();
    Nonce::assume_unique_for_key([
        0, 0, 0, 0, event[0], event[1], event[2], event[3], day[0], day[1], day[2], day[3],
    ])
}

fn read_keyfile() -> Option<String> {
    std::fs::read_to_string(KEYFILE_PATH).ok()
}
