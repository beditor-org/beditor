use std::process::ChildStdin;

use crate::{codec::base64::Base64Codec, connection::Connection, protocol::Protocol};

pub struct FrameStreamProtocol {
	pub connection: Connection<Base64Codec, ChildStdin>,
}

impl Protocol for FrameStreamProtocol {
	type Codec = Base64Codec;
	type Writer = ChildStdin;

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}
