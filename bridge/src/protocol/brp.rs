use std::io::Write;

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, rpc::JsonRpcClient, TypeName};

pub struct BrpProtocol<W: Write> {
	pub client: JsonRpcClient<W>,
}

impl<W: Write> Protocol for BrpProtocol<W> {
	type Codec = JsonCodec;
	type Writer = W;
}

impl<W: Write + Send + 'static> BrpProtocol<W> {
	pub fn new(connection: Connection<JsonCodec, W>) -> Self {
		let mut client = JsonRpcClient::new(connection);
		// client.run();
		Self { client }
	}

	pub async fn list_entities(&mut self) {
		let list = self.client.request::<(), serde_json::Value>("list_entities", ()).await;
		println!("Entities: {:?}", list);
	}

	pub async fn ping(&mut self) {
		let list = self.client.request::<(), serde_json::Value>("ping", ()).await;
	}

	pub fn game_process_ready(&mut self) {
		let _ = self.client.notify("game_process_ready", ());
	}
}

impl<W: Write> TypeName for BrpProtocol<W> {
	fn type_name() -> &'static str {
		"bridge::protocol::brp::BrpProtocol"
	}
}
