use serde_json::Value;

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

pub struct WorldProtocol {
	pub connection: Connection<JsonCodec<Value>>,
}

impl Protocol for WorldProtocol {
	type Codec = JsonCodec<Value>;
	fn from_connection(connection: Connection<Self::Codec>) -> Self {
		Self::new(connection)
	}
}

impl WorldProtocol {
	pub fn new(connection: Connection<JsonCodec<Value>>) -> Self {
		Self { connection }
	}

	/// Notify the game that the editor is ready to receive updates.
	pub fn editor_ready(&self) {
		let _ = self.connection.send(&serde_json::json!({
			"method": "editor_ready",
			"params": null,
		}));
	}
}

impl TypeName for WorldProtocol {
	fn type_name() -> &'static str {
		"bridge::protocol::world::WorldProtocol"
	}
}
