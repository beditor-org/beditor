use flume::{Receiver, Sender};

use crate::codec::Codec;

#[derive(Debug)]
pub enum ConnectionError<E: std::error::Error> {
	Codec(E),
	Disconnected,
}

impl<E: std::error::Error + std::fmt::Display> std::fmt::Display for ConnectionError<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ConnectionError::Codec(e) => write!(f, "Codec error: {}", e),
			ConnectionError::Disconnected => write!(f, "Channel disconnected"),
		}
	}
}

impl<E: std::error::Error + 'static> std::error::Error for ConnectionError<E> {}

pub struct Connection<C: Codec> {
	receiver: Receiver<Vec<u8>>,
	sender: Sender<Vec<u8>>,
	_codec: std::marker::PhantomData<C>,
}

impl<C: Codec> Connection<C> {
	pub fn new(receiver: Receiver<Vec<u8>>, sender: Sender<Vec<u8>>) -> Self {
		Self {
			receiver,
			sender,
			_codec: std::marker::PhantomData,
		}
	}

	pub fn send(&self, message: &C::Message) -> Result<(), ConnectionError<C::Error>> {
		let encoded = C::encode(message);
		self.sender.send(encoded).map_err(|_| ConnectionError::Disconnected)
	}

	pub fn try_recv(&self) -> Result<Option<C::Message>, ConnectionError<C::Error>> {
		match self.receiver.try_recv() {
			Ok(data) => C::decode(&data).map(Some).map_err(ConnectionError::Codec),
			Err(flume::TryRecvError::Empty) => Ok(None),
			Err(flume::TryRecvError::Disconnected) => Err(ConnectionError::Disconnected),
		}
	}

	pub fn recv(&self) -> Result<C::Message, ConnectionError<C::Error>> {
		match self.receiver.recv() {
			Ok(data) => C::decode(&data).map_err(ConnectionError::Codec),
			Err(_) => Err(ConnectionError::Disconnected),
		}
	}

	pub async fn recv_async(&self) -> Result<C::Message, ConnectionError<C::Error>> {
		match self.receiver.recv_async().await {
			Ok(data) => C::decode(&data).map_err(ConnectionError::Codec),
			Err(_) => Err(ConnectionError::Disconnected),
		}
	}
}
