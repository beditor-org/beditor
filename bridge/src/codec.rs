pub mod base64;
pub mod json;
pub mod raw;

pub trait Codec {
	type Message;
	type Error: std::error::Error + Send + Sync + 'static;

	fn encode(msg: &Self::Message) -> Vec<u8>;
	fn decode(data: &[u8]) -> Result<Self::Message, Self::Error>;
}
