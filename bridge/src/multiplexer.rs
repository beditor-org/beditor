use std::{
	collections::HashMap,
	hash::{DefaultHasher, Hash, Hasher},
	io::{BufReader, Read, Write},
	sync::{
		mpsc::{channel, Receiver, Sender},
		Arc, Mutex, RwLock,
	},
};

use tracing::{error, info_span, warn};

/// Frame format: [channel_id: u64 (8 bytes)][length: u32 BE (4 bytes)][payload: bytes]
const HEADER_SIZE: usize = 12;

pub struct Multiplexer<R: Read + Send + 'static, W: Write + Send + 'static> {
	pub reader: Option<R>,
	writer: Arc<Mutex<W>>,
	// u64 from type name hash - each protocol gets its own channel
	channels: Arc<RwLock<HashMap<u64, Sender<Vec<u8>>>>>,
}

impl<R: Read + Send + 'static, W: Write + Send + 'static> Multiplexer<R, W> {
	pub fn new(reader: R, writer: W) -> Self {
		Self {
			reader: Some(reader),
			writer: Arc::new(Mutex::new(writer)),
			channels: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	/// Generate a unique channel ID from a type using type name hash
	/// This is stable across different processes/binaries
	pub fn channel_id_for_type<T: 'static>() -> u64 {
		let type_name = std::any::type_name::<T>();
		let mut hasher = DefaultHasher::new();
		type_name.hash(&mut hasher);
		hasher.finish()
	}

	/// Register a protocol channel using its type as the channel ID
	pub fn register_for_type<T: 'static>(&self) -> Receiver<Vec<u8>> {
		self.register_channel(Self::channel_id_for_type::<T>())
	}

	/// Get a writer for a protocol using its type as the channel ID
	pub fn get_writer_for_type<T: 'static>(&self) -> ChannelWriter<W> {
		self.get_writer(Self::channel_id_for_type::<T>())
	}

	/// Register a protocol on a specific channel
	pub fn register_channel(&self, channel_id: u64) -> Receiver<Vec<u8>> {
		let (tx, rx) = channel();
		self.channels.write().unwrap().insert(channel_id, tx);
		rx
	}

	/// Get writer for sending messages on a channel
	pub fn get_writer(&self, channel_id: u64) -> ChannelWriter<W> {
		ChannelWriter {
			channel_id,
			writer: Arc::clone(&self.writer),
		}
	}

	/// Start multiplexer - spawns reader thread
	pub fn start(&mut self) {
		let reader = self.reader.take().expect("Multiplexer already started");
		let channels = Arc::clone(&self.channels);

		std::thread::spawn(move || {
			let reader_type = std::any::type_name::<R>().rsplit("::").next().unwrap_or("Unknown");
			let writer_type = std::any::type_name::<W>().rsplit("::").next().unwrap_or("Unknown");
			let span = info_span!("multiplexer", reader = reader_type, writer = writer_type);
			let _enter = span.enter();

			let mut reader = BufReader::new(reader);
			let mut header = [0u8; HEADER_SIZE];

			loop {
				// Read frame header
				if let Err(e) = reader.read_exact(&mut header) {
					error!("Read error: {}", e);
					break;
				}

				// Parse channel_id (u64, 8 bytes) - LITTLE ENDIAN
				let channel_id = u64::from_le_bytes([
					header[0], header[1], header[2], header[3], header[4], header[5], header[6], header[7],
				]);

				// Parse length (u32, 4 bytes)
				let length = u32::from_be_bytes([header[8], header[9], header[10], header[11]]) as usize;

				// Read payload
				let mut payload = vec![0u8; length];
				if let Err(e) = reader.read_exact(&mut payload) {
					error!("Payload read error: {}", e);
					break;
				}

				// Route to channel
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
	}
}

pub struct ChannelWriter<W: Write> {
	channel_id: u64,
	writer: Arc<Mutex<W>>,
}

impl<W: Write> ChannelWriter<W> {
	pub fn send(&self, data: &[u8]) -> std::io::Result<()> {
		let mut writer = self.writer.lock().unwrap();

		// Write frame: [channel_id: u64 LE][length: u32 BE][payload]
		writer.write_all(&self.channel_id.to_le_bytes())?;
		writer.write_all(&(data.len() as u32).to_be_bytes())?;
		writer.write_all(data)?;
		writer.flush()?;

		Ok(())
	}
}
