use std::{
	io::{stdin, stdout},
	sync::{Arc, Mutex},
};

use bevy::{asset::ron::error, prelude::*};
use bridge::{connection::Connection, multiplexer::Multiplexer, protocol::brp::GameBrpProtocol};

pub fn brp_handler(world: &mut World) {
	let mut brp = world.get_resource_mut::<BrpConnection>().unwrap();
	while let Ok(data) = brp.connection.lock().unwrap().connection.reader.try_recv() {
		// let request: RpcRequest = serde_json::from_slice(&data)?;

		// let response = match request.method.as_str() {
		// 	"bevy/list" => handle_list(world),
		// 	"custom/foo" => handle_foo(world),
		// 	_ => {
		// 		error!("Unknown method: {}", request.method);
		// 	}
		// };

		// brp.sender.send(&serde_json::to_vec(&response)?)?;
	}
}

#[derive(Resource)]
pub struct BrpConnection {
	connection: Arc<Mutex<GameBrpProtocol>>,
}

pub fn brp_sender(connection: ResMut<BrpConnection>) {}

pub struct BrpProtocolPlugin;
impl Plugin for BrpProtocolPlugin {
	fn build(&self, app: &mut App) {
		let multiplexer = Multiplexer::new(stdin(), stdout());

		let connection = Connection::new(
			bridge::codec::json::JsonCodec,
			multiplexer.register_for_type::<GameBrpProtocol>(),
			multiplexer.get_writer_for_type::<GameBrpProtocol>(),
		);

		app.add_systems(Update, brp_handler).insert_resource(BrpConnection {
			connection: Arc::new(Mutex::new(GameBrpProtocol::new(connection))),
		});
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bridge::multiplexer::Multiplexer;

	#[test]
	fn test_channel_id() {
		let channel_id = Multiplexer::<std::io::Stdin, std::io::Stdout>::channel_id_for_type::<GameBrpProtocol>();
		eprintln!("BrpProtocol Channel ID: {:#018x}", channel_id);
	}
}
