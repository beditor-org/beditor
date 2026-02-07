use std::{io::Stdout, process::ChildStdin};

use crate::{
	codec::{json::JsonCodec, Codec},
	connection::Connection,
	protocol::Protocol,
};

pub struct BrpProtocol {
	connection: Connection<JsonCodec, ChildStdin>,
}

impl Protocol for BrpProtocol {
	type Codec = JsonCodec;
	type Writer = ChildStdin;

	fn handle(&self, message: <Self::Codec as Codec>::Message) {
		todo!()
	}

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}

impl BrpProtocol {
	pub fn new(connection: Connection<JsonCodec, ChildStdin>) -> Self {
		Self { connection }
	}

	pub fn list_entities(&mut self) {
		self.connection.send(serde_json::Value::String("list_entities".to_string()));
	}
}

pub struct GameBrpProtocol {
	pub connection: Connection<JsonCodec, Stdout>,
}

impl GameBrpProtocol {
	pub fn new(connection: Connection<JsonCodec, Stdout>) -> Self {
		Self { connection }
	}
}

impl Protocol for GameBrpProtocol {
	type Codec = JsonCodec;
	type Writer = Stdout;

	fn handle(&self, message: <Self::Codec as Codec>::Message) {
		todo!()
	}

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}
