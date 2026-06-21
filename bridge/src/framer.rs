use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

pub trait Framer {
	type Frame;
	type Error: std::error::Error + Send + Sync + 'static;

	/// Read one frame from the reader. Returns Ok(None) on EOF.
	fn read_frame<R: AsyncRead + Unpin + Send>(
		&self,
		reader: &mut BufReader<R>,
	) -> impl std::future::Future<Output = Result<Option<Self::Frame>, Self::Error>> + Send;

	/// Serialize a frame to bytes for writing.
	fn write_frame(&self, frame: &Self::Frame) -> Vec<u8>;
}

#[derive(Clone)]
pub struct LineFramer {
	pub delimiter: u8,
}

impl LineFramer {
	pub fn new() -> Self {
		Self { delimiter: b'\n' }
	}

	pub fn with_delimiter(delimiter: u8) -> Self {
		Self { delimiter }
	}
}

impl Framer for LineFramer {
	type Frame = Vec<u8>;
	type Error = std::io::Error;

	async fn read_frame<R: AsyncRead + Unpin + Send>(
		&self,
		reader: &mut BufReader<R>,
	) -> Result<Option<Self::Frame>, Self::Error> {
		let mut buf = Vec::new();
		let n = reader.read_until(self.delimiter, &mut buf).await?;
		if n == 0 {
			return Ok(None); // EOF
		}
		Ok(Some(buf))
	}

	fn write_frame(&self, frame: &Self::Frame) -> Vec<u8> {
		frame.clone()
	}
}

pub struct MuxFrame {
	pub channel_id: u64,
	pub payload: Vec<u8>,
}

#[derive(Clone)]
pub struct MuxFramer;

impl Framer for MuxFramer {
	type Frame = MuxFrame;
	type Error = std::io::Error;

	async fn read_frame<R: AsyncRead + Unpin + Send>(
		&self,
		reader: &mut BufReader<R>,
	) -> Result<Option<Self::Frame>, Self::Error> {
		let mut header = [0u8; 12];
		match reader.read_exact(&mut header).await {
			Ok(_) => {}
			Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
			Err(e) => return Err(e),
		}
		let channel_id = u64::from_le_bytes(header[..8].try_into().unwrap());
		let length = u32::from_be_bytes(header[8..12].try_into().unwrap()) as usize;
		let mut payload = vec![0u8; length];
		reader.read_exact(&mut payload).await?;
		Ok(Some(MuxFrame { channel_id, payload }))
	}

	fn write_frame(&self, frame: &Self::Frame) -> Vec<u8> {
		let mut buf = Vec::with_capacity(12 + frame.payload.len());
		buf.extend_from_slice(&frame.channel_id.to_le_bytes());
		buf.extend_from_slice(&(frame.payload.len() as u32).to_be_bytes());
		buf.extend_from_slice(&frame.payload);
		buf
	}
}
