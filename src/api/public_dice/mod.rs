use anyhow::{anyhow, Error};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};

pub(crate) mod endpoint_update;
pub(crate) mod list;
pub(crate) mod register;
pub(crate) mod tick_update;

#[derive(Serialize, Deserialize)]
pub(crate) struct KeyPayload {
    pub version: String,
    pub sign: Vec<u8>,
}

impl KeyPayload {
    /// `from_key`: parse payload from key
    ///
    /// pattern is SEAL#<base64(payload)>, payload is rmp-serde encoded KeyPayload
    pub(crate) fn from_key(key: String) -> Result<Self, Error> {
        let stripped_key = if key.starts_with("SEAL~") || key.starts_with("SEAL#") {
            key.get(5..).ok_or_else(|| anyhow!("invalid key"))
        } else {
            return Err(anyhow!("invalid key"));
        }?;
        let decoded_bytes = STANDARD.decode(stripped_key)?;
        let payload: KeyPayload = rmp_serde::from_slice(&decoded_bytes)?;

        Ok(payload)
    }
}
