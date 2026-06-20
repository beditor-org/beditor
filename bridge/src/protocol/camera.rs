use std::process::ChildStdin;

use serde::{Deserialize, Serialize};

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseEvent {
	pub x: f32,
	pub y: f32,
}

pub struct CameraInputProtocol {
	pub connection: Connection<JsonCodec<MouseEvent>>,
}

impl TypeName for CameraInputProtocol {}

impl Protocol for CameraInputProtocol {
	type Codec = JsonCodec<MouseEvent>;
	type Writer = ChildStdin;

	// fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer> {
	// 	&mut self.connection
	// }
}
