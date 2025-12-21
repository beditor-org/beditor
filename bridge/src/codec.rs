pub mod base64;
pub mod json;

pub trait Codec {
	type Message;
	type Error: std::error::Error + Send + Sync + 'static;

	fn encode(&self, msg: &Self::Message) -> Vec<u8>;
	fn decode(&self, data: &[u8]) -> Result<Self::Message, Self::Error>;
}
