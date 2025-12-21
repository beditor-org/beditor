use std::{io::Write, sync::mpsc::Receiver};

use crate::{codec::Codec, multiplexer::ChannelWriter};

pub struct Connection<C: Codec, W: Write> {
	codec: C,
	pub reader: Receiver<Vec<u8>>,
	writer: ChannelWriter<W>,
}

impl<C: Codec, W: Write> Connection<C, W> {
	pub fn new(codec: C, reader: Receiver<Vec<u8>>, writer: ChannelWriter<W>) -> Self {
		Self { codec, reader, writer }
	}
	pub fn send(&mut self, message: C::Message) {
		let encoded = self.codec.encode(&message);
		let _ = self.writer.send(&encoded);
	}

	pub fn try_recv(&mut self) -> Result<Option<C::Message>, Box<dyn std::error::Error>> {
		match self.reader.try_recv() {
			Ok(data) => match self.codec.decode(&data) {
				Ok(message) => Ok(Some(message)),
				Err(e) => Err(Box::new(e)),
			},
			Err(std::sync::mpsc::TryRecvError::Empty) => Ok(None),
			Err(e) => Err(Box::new(e)),
		}
	}
}
