use std::{
	collections::HashMap,
	hash::{DefaultHasher, Hash, Hasher},
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc, RwLock,
	},
};

use flume::{Receiver, Sender};
use tokio::{
	io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
	task::JoinHandle,
};
use tracing::{error, warn};

use crate::{connection::Connection, framer::MuxFrame, protocol::Protocol, TypeName};

/// Frame format: [channel_id: u64 (8 bytes)][length: u32 BE (4 bytes)][payload: bytes]
const HEADER_SIZE: usize = 12;

pub struct Multiplexer<R, W> {
	pub reader: Option<R>,
	pub writer: Option<W>,
	channels: Arc<RwLock<HashMap<u64, Sender<Vec<u8>>>>>,
	write_tx: Sender<MuxFrame>,
	write_rx: Receiver<MuxFrame>,
	pub bytes_sent: Arc<AtomicU64>,
	pub bytes_received: Arc<AtomicU64>,
}

impl<R, W> Multiplexer<R, W>
where
	R: AsyncRead + Unpin + Send + 'static,
	W: AsyncWrite + Unpin + Send + 'static,
{
	pub fn new(reader: R, writer: W) -> Self {
		let (write_tx, write_rx) = flume::unbounded();
		Self {
			reader: Some(reader),
			writer: Some(writer),
			channels: Arc::new(RwLock::new(HashMap::new())),
			write_tx,
			write_rx,
			bytes_sent: Arc::new(AtomicU64::new(0)),
			bytes_received: Arc::new(AtomicU64::new(0)),
		}
	}

	/// Generate a unique channel ID from a type using type name hash
	pub fn channel_id_for_type<T: TypeName>() -> u64 {
		let type_name = T::type_name();
		let mut hasher = DefaultHasher::new();
		type_name.hash(&mut hasher);
		let id = hasher.finish();
		tracing::debug!("Channel ID for {}: {} (0x{:016x})", type_name, id, id);
		id
	}

	pub fn register_for_type<T: TypeName>(&self) -> (Receiver<Vec<u8>>, Sender<Vec<u8>>) {
		let channel_id = Self::channel_id_for_type::<T>();
		tracing::info!("Registering channel for {}: {}", T::type_name(), channel_id);
		(self.register_channel(channel_id), self.get_writer(channel_id))
	}

	pub fn register_protocol<T: TypeName + Protocol>(&self) -> T {
		let (reader, writer) = self.register_for_type::<T>();
		T::from_connection(Connection::new(reader, writer))
	}

	pub fn register_channel(&self, channel_id: u64) -> Receiver<Vec<u8>> {
		let (tx, rx) = flume::unbounded();
		self.channels.write().unwrap().insert(channel_id, tx);
		rx
	}

	pub fn get_writer(&self, channel_id: u64) -> Sender<Vec<u8>> {
		let (tx, rx) = flume::unbounded::<Vec<u8>>();
		let write_tx = self.write_tx.clone();
		std::thread::spawn(move || {
			while let Ok(data) = rx.recv() {
				if write_tx
					.send(MuxFrame {
						channel_id,
						payload: data,
					})
					.is_err()
				{
					break;
				}
			}
		});
		tx
	}

	/// Start multiplexer — spawns reader and writer tasks, returns their handles
	pub fn start(&mut self) -> (JoinHandle<()>, JoinHandle<()>) {
		let reader = self.reader.take().expect("Multiplexer already started");
		let writer = self.writer.take().expect("Multiplexer already started");
		let channels = Arc::clone(&self.channels);
		let bytes_received = Arc::clone(&self.bytes_received);
		let bytes_sent = Arc::clone(&self.bytes_sent);
		let write_rx = self.write_rx.clone();

		let reader_handle = tokio::spawn(async move {
			let mut reader = BufReader::new(reader);
			let mut header = [0u8; HEADER_SIZE];

			loop {
				if let Err(e) = reader.read_exact(&mut header).await {
					error!("Read error: {}", e);
					break;
				}

				let channel_id = u64::from_le_bytes(header[..8].try_into().unwrap());
				let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as usize;

				let mut payload = vec![0u8; length];
				if let Err(e) = reader.read_exact(&mut payload).await {
					error!("Payload read error: {}", e);
					break;
				}
				bytes_received.fetch_add(length as u64, Ordering::Relaxed);

				let channels = channels.read().unwrap();
				if let Some(tx) = channels.get(&channel_id) {
					if tx.send(payload).is_err() {
						warn!(channel_id, "Channel receiver dropped");
					}
				} else {
					warn!(channel_id, "Unknown channel");
				}
			}
		});

		let writer_handle = tokio::spawn(async move {
			let mut writer = writer;
			while let Ok(frame) = write_rx.recv_async().await {
				let len = frame.payload.len();
				let result: std::io::Result<()> = async {
					writer.write_all(&frame.channel_id.to_le_bytes()).await?;
					writer.write_all(&(frame.payload.len() as u32).to_be_bytes()).await?;
					writer.write_all(&frame.payload).await?;
					writer.flush().await
				}
				.await;

				if let Err(e) = result {
					error!("Write error: {}", e);
					break;
				}
				bytes_sent.fetch_add(len as u64, Ordering::Relaxed);
			}
		});

		(reader_handle, writer_handle)
	}
}
