use std::io::Write;

use crate::{codec::Codec, connection::Connection};

pub mod brp;
pub mod camera;
pub mod frame_stream;

pub trait Protocol {
	type Codec: Codec;
	type Writer: Write;

	// fn connection(&mut self) -> &mut Connection<Self::Codec, Self::Writer>;

	// fn send(&mut self, message: <Self::Codec as Codec>::Message) {
	// 	self.connection().send(message);
	// }

	// fn handle(&self, message: <Self::Codec as Codec>::Message) {}
}
