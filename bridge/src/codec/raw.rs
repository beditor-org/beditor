use crate::codec::Codec;

/// Codec that passes raw bytes through unchanged.
/// Used for binary protocols (e.g. JPEG frame stream) where no text encoding is needed.
/// Framing is handled by MuxFramer (12-byte length-prefixed header).
#[derive(Debug, Clone, Copy)]
pub struct RawCodec;

impl Codec for RawCodec {
	type Message = Vec<u8>;
	type Error = std::convert::Infallible;

	fn encode(msg: &Self::Message) -> Vec<u8> {
		msg.clone()
	}

	fn decode(data: &[u8]) -> Result<Self::Message, Self::Error> {
		Ok(data.to_vec())
	}
}
