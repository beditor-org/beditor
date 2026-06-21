use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

/// Minimal entity info for hierarchy display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
	pub id: u32,
	pub name: String,
	/// Parent entity id. `None` means root-level entity.
	pub parent: Option<u32>,
}

/// A single serialized component attached to an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
	/// Fully-qualified Rust type name, e.g. `"bevy_transform::components::transform::Transform"`.
	pub type_name: String,
	pub value: Value,
}

/// All messages exchanged over the BEP channel.
///
/// Serializes to/from `{"method": "<variant>", "params": <data>}`.
/// Unit variants omit the "params" key entirely.
///
/// # Examples
/// ```json
/// {"method": "game_ready"}
/// {"method": "editor_ready"}
/// {"method": "select_entity", "params": {"entity": 42}}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum BepMessage {
	/// Sent by the game after Bevy PostStartup — all systems are ready.
	GameReady,
	/// Full snapshot of all entities currently in the world (id + name + parent only).
	EntitiesListUpdate { entities: Vec<EntityInfo> },
	/// Sent by the editor to select an entity. The game responds with `EntityComponentsUpdate`.
	SelectEntity { entity: u32 },
	/// Sent by the game with all components of the currently selected entity.
	/// Re-sent whenever the selected entity's components change.
	EntityComponentsUpdate { entity: u32, components: Vec<ComponentData> },
}

pub struct BepProtocol {
	pub connection: Connection<JsonCodec<BepMessage>>,
}

impl Protocol for BepProtocol {
	type Codec = JsonCodec<BepMessage>;
	fn from_connection(connection: Connection<Self::Codec>) -> Self {
		Self::new(connection)
	}
}

impl BepProtocol {
	pub fn new(connection: Connection<JsonCodec<BepMessage>>) -> Self {
		Self { connection }
	}

	pub fn game_ready(&self) {
		let _ = self.connection.send(&BepMessage::GameReady);
	}

	pub fn update_entities_list(&self, entities: Vec<EntityInfo>) {
		let _ = self.connection.send(&BepMessage::EntitiesListUpdate { entities });
	}

	pub fn entity_components_update(&self, entity: u32, components: Vec<ComponentData>) {
		let _ = self
			.connection
			.send(&BepMessage::EntityComponentsUpdate { entity, components });
	}

	pub fn select_entity(&self, entity: u32) {
		let _ = self.connection.send(&BepMessage::SelectEntity { entity });
	}
}

impl TypeName for BepProtocol {
	fn type_name() -> &'static str {
		"bridge::protocol::bep::BepProtocol"
	}
}
