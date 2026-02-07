use std::io::Write;

use crate::{
	codec::{json::JsonCodec, Codec},
	connection::Connection,
	protocol::Protocol,
	TypeName,
};

pub struct BrpProtocol<W: Write> {
	pub connection: Connection<JsonCodec, W>,
}

impl<W: Write> Protocol for BrpProtocol<W> {
	type Codec = JsonCodec;
	type Writer = W;

	fn handle(&self, message: <Self::Codec as Codec>::Message) {
		todo!()
	}

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}

impl<W: Write> BrpProtocol<W> {
	pub fn new(connection: Connection<JsonCodec, W>) -> Self {
		Self { connection }
	}

	pub fn list_entities(&mut self) {
		self.connection.send(serde_json::Value::String("list_entities".to_string()));
	}
}

impl<W: Write> TypeName for BrpProtocol<W> {
	fn type_name() -> &'static str {
		"bridge::protocol::brp::BrpProtocol"
	}
}
