use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_128_GCM};
use aws_lc_rs::error::Unspecified;
use base64::prelude::*;

pub struct Encryption {
    key_bytes: Vec<u8>,
}

const KEYFILE_PATH: &'static str = ".input-key";
const KEYFILE_ENV: &'static str = "AOC_INPUT_KEY";

impl Encryption {
    pub fn new() -> Option<Self> {
        let key_str = std::env::var(KEYFILE_ENV).ok().or(read_keyfile())?;
        let key_bytes = BASE64_STANDARD.decode(&key_str.trim()).unwrap();
        let _key = UnboundKey::new(&AES_128_GCM, &key_bytes).unwrap();

        Some(Self { key_bytes })
    }

    pub fn encrypt_day(
        &self,
        event: u32,
        day: u32,
        mut data: Vec<u8>,
    ) -> Result<Vec<u8>, Unspecified> {
        let nonce = Nonce::from(&[0, event, day]);
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
    ) -> Result<Vec<u8>, Unspecified> {
        let nonce = Nonce::from(&[0, event, day]);
        let key =
            UnboundKey::new(&AES_128_GCM, &self.key_bytes).expect("key validated in constructor");
        let key = LessSafeKey::new(key);

        let plaintext = key.open_in_place(nonce, Aad::empty(), &mut data)?;
        let len = plaintext.len();
        data.truncate(len);

        Ok(data)
    }
}

fn read_keyfile() -> Option<String> {
    std::fs::read_to_string(KEYFILE_PATH).ok()
}
