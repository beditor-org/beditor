use bevy::prelude::*;
use bridge::protocol::bep::{BepProtocol, EntityInfo};

use crate::app::ResMultiplexer;

#[derive(Resource)]
pub struct BepResource {
	pub protocol: BepProtocol,
}

pub struct BepPlugin;
impl Plugin for BepPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostStartup, send_game_ready_and_entities);
	}
}

fn send_game_ready_and_entities(
	mut commands: Commands,
	multiplexer: Res<ResMultiplexer>,
	entities: Query<(Entity, Option<&Name>, Option<&ChildOf>)>,
) {
	let protocol = multiplexer.multiplexer.register_protocol::<BepProtocol>();

	protocol.game_ready();

	let entity_list: Vec<EntityInfo> = entities
		.iter()
		.map(|(entity, name, parent)| EntityInfo {
			id: entity.index(),
			name: name
				.map(|n| n.as_str().to_string())
				.unwrap_or_else(|| format!("Entity {}", entity.index())),
			parent: parent.map(|p| p.parent().index()),
		})
		.collect();

	protocol.update_entities_list(entity_list);

	commands.insert_resource(BepResource { protocol });
}

#[cfg(test)]
mod tests {
	use super::*;
	use bridge::multiplexer::Multiplexer;

	#[test]
	fn test_channel_id() {
		let channel_id = Multiplexer::<tokio::io::DuplexStream, tokio::io::DuplexStream>::channel_id_for_type::<BepProtocol>();
		eprintln!("BepProtocol Channel ID: {:#018x}", channel_id);
	}
}
