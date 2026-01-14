use std::process::ChildStdin;

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol};

pub struct CameraInputProtocol {
	pub connection: Connection<JsonCodec, ChildStdin>,
}

impl Protocol for CameraInputProtocol {
	type Codec = JsonCodec;
	type Writer = ChildStdin;

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}
