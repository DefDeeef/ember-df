mod decode;
mod encode;

pub use decode::decode_template;
#[allow(unused_imports)]
pub use encode::encode_template;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let json = r#"{"blocks":[]}"#;
        let encoded = encode_template(json).unwrap();
        assert_eq!(decode_template(&encoded).unwrap(), json);
    }
}