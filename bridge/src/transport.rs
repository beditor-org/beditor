use std::sync::mpsc::Receiver;

pub trait Transport {
	fn start(&mut self) -> Receiver<Vec<u8>>;
}
