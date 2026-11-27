use base64::{engine::general_purpose::STANDARD, Engine as _};
use flate2::read::GzDecoder;
use std::io::Read;

pub fn decode_template(s: &str) -> Result<String, Box<dyn std::error::Error>> {
    let compressed = STANDARD.decode(s.trim())?;
    let mut text = String::new();
    GzDecoder::new(&compressed[..])
        .read_to_string(&mut text)?;
    Ok(text)
}