use crate::{codec::raw::RawCodec, connection::Connection, protocol::Protocol, TypeName};

pub struct FrameStreamProtocol {
	pub connection: Connection<RawCodec>,
}

impl TypeName for FrameStreamProtocol {}

impl Protocol for FrameStreamProtocol {
	type Codec = RawCodec;
	fn from_connection(connection: Connection<Self::Codec>) -> Self {
		Self { connection }
	}
}
