use std::io::Stdout;

use bevy::ecs::resource::Resource;

use crate::{
	codec::{json::JsonCodec, Codec},
	connection::Connection,
	protocol::Protocol,
};

pub struct EditorBrpProtocol {
	connection: Connection<JsonCodec, Stdout>,
}

impl Protocol for EditorBrpProtocol {
	type Codec = JsonCodec;
	type Writer = Stdout;

	fn handle(&self, message: <Self::Codec as Codec>::Message) {
		todo!()
	}

	fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
		&mut self.connection
	}
}

impl EditorBrpProtocol {
	pub fn list_entities(&self) {
		todo!()
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
