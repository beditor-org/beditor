use std::{
	io::{stdin, stdout, Stdin, Stdout},
	sync::{Arc, Mutex},
};

use bevy::prelude::*;
use bridge::{codec::json::JsonCodec, connection::Connection, multiplexer::Multiplexer, protocol::bep::BepProtocol};
use flume::{unbounded, Receiver, Sender};

use crate::app::ResMultiplexer;

pub struct BepCommands {}

pub fn bep_handler(bep: Res<BepResource>) {
	while let Ok(Some(message)) = bep.protocol.connection.try_recv() {}
}
// let mut brp = world.get_resource_mut::<BrpConnection>().unwrap();
// while let Ok(data) = brp.connection.lock().unwrap().connection.reader.try_recv() {
// let request: RpcRequest = serde_json::from_slice(&data)?;

// let response = match request.method.as_str() {
// 	"bevy/list" => handle_list(world),
// 	"custom/foo" => handle_foo(world),
// 	_ => {
// 		error!("Unknown method: {}", request.method);
// 	}
// };

// brp.sender.send(&serde_json::to_vec(&response)?)?;
// }

#[derive(Resource)]
pub struct BepStream {
	pub rx: Receiver<String>,
	pub tx: Sender<String>,
}
#[derive(Resource)]
pub struct BepResource {
	pub protocol: BepProtocol,
}

pub struct BepPlugin;
impl Plugin for BepPlugin {
	fn build(&self, app: &mut App) {
		// TODO: initialize BepProtocol on client side
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bridge::multiplexer::Multiplexer;

	#[test]
	fn test_channel_id() {
		let channel_id = Multiplexer::<std::io::Stdin, std::io::Stdout>::channel_id_for_type::<BepProtocol>();
		eprintln!("BepProtocol Channel ID: {:#018x}", channel_id);
	}
}
