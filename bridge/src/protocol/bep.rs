use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum EntityKind {
	#[default]
	Entity,
	Resource,
}

/// Minimal entity info for hierarchy display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
	pub id: u32,
	pub name: String,
	/// Parent entity id. `None` means root-level entity.
	pub parent: Option<u32>,
	#[serde(default)]
	pub kind: EntityKind,
}

/// A single serialized component attached to an entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentData {
	pub type_name: String,
	pub short_name: String,
	pub fields: Vec<FieldData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldData {
	pub name: String,
	pub field_type: String,
	pub value: FieldValue,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldValue {
	F32(f32),
	F64(f64),
	I32(i32),
	U32(u32),
	I64(i64),
	U64(u64),
	Bool(bool),
	String(String),
	Vec2 {
		x: f32,
		y: f32,
	},
	Vec3 {
		x: f32,
		y: f32,
		z: f32,
	},
	Vec4 {
		x: f32,
		y: f32,
		z: f32,
		w: f32,
	},
	Quat {
		x: f32,
		y: f32,
		z: f32,
		w: f32,
	},
	Color {
		r: f32,
		g: f32,
		b: f32,
		a: f32,
	},
	Struct(Vec<FieldData>),
	List(Vec<FieldValue>),
	Enum {
		variant: String,
		value: Option<Box<FieldValue>>,
	},
	// Fallback
	Unknown(Value),
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
	/// Sent by the editor to select an entity (`Some(id)`) or deselect (`None`).
	/// The game responds with `EntityComponentsUpdate` and keeps pushing updates until deselected.
	SelectEntity { entity: Option<u32> },
	/// Sent by the game with all components of the currently selected entity.
	/// Re-sent whenever the selected entity's components change.
	EntityComponentsUpdate { entity: u32, components: Vec<ComponentData> },
}

#[derive(Clone)]
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

	pub fn select_entity(&self, entity: Option<u32>) {
		let _ = self.connection.send(&BepMessage::SelectEntity { entity });
	}
}

impl TypeName for BepProtocol {
	fn type_name() -> &'static str {
		"bridge::protocol::bep::BepProtocol"
	}
}
