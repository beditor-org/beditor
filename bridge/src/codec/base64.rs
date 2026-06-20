use crate::codec::Codec;
use base64::{engine::general_purpose, Engine};

/// Codec for base64-encoded data.
/// Useful for sending binary data (like images) that needs to be embedded
/// in web contexts as data URLs.
#[derive(Debug, Clone, Copy)]
pub struct Base64Codec;

#[derive(Debug)]
pub enum Base64Error {
	Utf8(std::str::Utf8Error),
	Decode(base64::DecodeError),
}

impl std::fmt::Display for Base64Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Base64Error::Utf8(e) => write!(f, "UTF-8 error: {}", e),
			Base64Error::Decode(e) => write!(f, "Base64 decode error: {}", e),
		}
	}
}

impl std::error::Error for Base64Error {}

impl Codec for Base64Codec {
	type Message = String;
	type Error = Base64Error;

	fn encode(msg: &Self::Message) -> Vec<u8> {
		msg.as_bytes().to_vec()
	}

	fn decode(data: &[u8]) -> Result<Self::Message, Self::Error> {
		// Convert bytes to UTF-8 string (which should be base64)
		let s = std::str::from_utf8(data).map_err(Base64Error::Utf8)?;

		// Validate it's valid base64 by attempting decode
		general_purpose::STANDARD.decode(s).map_err(Base64Error::Decode)?;

		Ok(s.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_round_trip() {
		// Create some base64-encoded data
		let original_data = b"Hello, World!";
		let base64_string = general_purpose::STANDARD.encode(original_data);

		let encoded = Base64Codec::encode(&base64_string);
		let decoded = Base64Codec::decode(&encoded).unwrap();

		assert_eq!(decoded, base64_string);

		// Verify we can decode the base64 back to original
		let decoded_bytes = general_purpose::STANDARD.decode(&decoded).unwrap();
		assert_eq!(decoded_bytes, original_data);
	}

	#[test]
	fn test_decode_invalid_utf8() {
		let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];

		let result = Base64Codec::decode(&invalid_utf8);
		assert!(result.is_err());
		assert!(matches!(result.unwrap_err(), Base64Error::Utf8(_)));
	}

	#[test]
	fn test_decode_invalid_base64() {
		let invalid_base64 = b"not valid base64!!!";

		let result = Base64Codec::decode(invalid_base64);
		assert!(result.is_err());
		assert!(matches!(result.unwrap_err(), Base64Error::Decode(_)));
	}

	#[test]
	fn test_empty() {
		let empty = general_purpose::STANDARD.encode(b"");

		let encoded = Base64Codec::encode(&empty);
		let decoded = Base64Codec::decode(&encoded).unwrap();

		assert_eq!(decoded, empty);
	}

	#[test]
	fn test_image_like_data() {
		// Simulate PNG-like binary data
		let png_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
		let base64_string = general_purpose::STANDARD.encode(&png_bytes);

		let encoded = Base64Codec::encode(&base64_string);
		let decoded = Base64Codec::decode(&encoded).unwrap();

		assert_eq!(decoded, base64_string);

		// Verify roundtrip through base64
		let decoded_png = general_purpose::STANDARD.decode(&decoded).unwrap();
		assert_eq!(decoded_png, png_bytes);
	}
}
