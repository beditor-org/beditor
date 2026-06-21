use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::codec::Codec;

#[derive(Debug, Clone)]
pub struct JsonCodec<T>(PhantomData<T>);

#[derive(Debug)]
pub enum JsonError {
	Utf8(std::str::Utf8Error),
	Json(serde_json::Error),
}

impl std::fmt::Display for JsonError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsonError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
			JsonError::Json(e) => write!(f, "JSON error: {}", e),
		}
	}
}

impl std::error::Error for JsonError {}

impl<T: Serialize + DeserializeOwned> Codec for JsonCodec<T> {
	type Message = T;
	type Error = JsonError;

	fn encode(msg: &Self::Message) -> Vec<u8> {
		serde_json::to_vec(msg).expect("serialization failed")
	}

	fn decode(data: &[u8]) -> Result<Self::Message, Self::Error> {
		let s = std::str::from_utf8(data).map_err(JsonError::Utf8)?;
		serde_json::from_str(s).map_err(JsonError::Json)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::{json, Value};

	#[test]
	fn test_round_trip() {
		let original = json!({
			"users": [
				{"id": 1, "name": "Alice"},
				{"id": 2, "name": "Bob"}
			],
			"count": 2,
			"active": true,
			"value": null
		});

		let encoded = JsonCodec::<Value>::encode(&original);
		let decoded = JsonCodec::<Value>::decode(&encoded).unwrap();

		assert_eq!(original, decoded);
	}

	#[test]
	fn test_decode_invalid_utf8() {
		let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];

		let result = JsonCodec::<Value>::decode(&invalid_utf8);
		assert!(result.is_err());
		assert!(matches!(result.unwrap_err(), JsonError::Utf8(_)));
	}

	#[test]
	fn test_decode_invalid_json() {
		let invalid_json = b"{invalid json}";

		let result = JsonCodec::<Value>::decode(invalid_json);
		assert!(result.is_err());
		assert!(matches!(result.unwrap_err(), JsonError::Json(_)));
	}

	#[test]
	fn test_error_display() {
		let utf8_err = JsonError::Utf8(std::str::from_utf8(&[0xFF]).unwrap_err());
		assert!(format!("{}", utf8_err).contains("UTF-8 error"));

		let json_err = JsonError::Json(serde_json::from_str::<Value>("{invalid}").unwrap_err());
		assert!(format!("{}", json_err).contains("JSON error"));
	}
}
