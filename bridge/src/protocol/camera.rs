use serde::{Deserialize, Serialize};

use crate::{codec::json::JsonCodec, connection::Connection, protocol::Protocol, TypeName};

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseEvent {
	pub x: f32,
	pub y: f32,
	/// Scroll wheel delta — used for camera dolly (move along forward vector).
	/// Positive = scroll down (zoom out), negative = scroll up (zoom in).
	#[serde(default)]
	pub scroll: f32,
	/// Pan delta — move camera along its right/up axes without changing rotation.
	/// Sent when Shift + MMB drag.
	#[serde(default)]
	pub pan_x: f32,
	#[serde(default)]
	pub pan_y: f32,
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
