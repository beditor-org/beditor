use crate::{codec::Codec, connection::Connection};

pub mod bep;
pub mod camera;
pub mod frame_stream;
pub mod world;

pub trait Protocol: Sized {
	type Codec: Codec;
	fn from_connection(connection: Connection<Self::Codec>) -> Self;
}
