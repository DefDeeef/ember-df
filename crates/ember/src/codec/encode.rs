use base64::{engine::general_purpose::STANDARD, Engine as _};
use flate2::{write::GzEncoder, Compression};
use std::io::Write;

use crate::error::Result;

pub fn encode_template(json: &str) -> Result<String> {
    let mut enc = GzEncoder::new(Vec::new(), Compression::default());
    enc.write_all(json.as_bytes())?;
    Ok(STANDARD.encode(enc.finish()?))
}