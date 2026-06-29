use serde::{Deserialize, Serialize};

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MouseEvent {
	pub x: f32,
	pub y: f32,
	/// Scroll wheel delta — dolly along camera forward.
	#[serde(default)]
	pub scroll: f32,
	/// Pan delta (Shift + MMB).
	#[serde(default)]
	pub pan_x: f32,
	#[serde(default)]
	pub pan_y: f32,
	/// Absolute mouse position in game-image space [0, 1].
	/// Computed by the editor accounting for letterboxing.
	#[serde(default)]
	pub abs_x: f32,
	#[serde(default)]
	pub abs_y: f32,
	/// Left mouse button edge events.
	#[serde(default)]
	pub lmb_pressed: bool,
	#[serde(default)]
	pub lmb_released: bool,
	/// True when LMB is held and mouse moves (used for gizmo drag).
	#[serde(default)]
	pub lmb_held: bool,
}

pub struct CameraInputProtocol {
	pub connection: Connection<JsonCodec<MouseEvent>>,
}

impl TypeName for CameraInputProtocol {}

impl Protocol for CameraInputProtocol {
	type Codec = JsonCodec<MouseEvent>;
	fn from_connection(connection: Connection<Self::Codec>) -> Self {
		Self { connection }
	}
}
