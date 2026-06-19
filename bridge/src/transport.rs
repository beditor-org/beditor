use flume::{Receiver, Sender};
use tokio::{
	io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
	task::JoinHandle,
};
use tracing::error;

use crate::framer::Framer;

pub struct Transport<F: Framer, R, W> {
	framer: F,
	reader: Option<R>,
	writer: Option<W>,
	/// Incoming frames from the stream
	pub incoming: Receiver<F::Frame>,
	incoming_tx: Sender<F::Frame>,
	/// Outgoing frames to the stream
	pub outgoing: Sender<F::Frame>,
	outgoing_rx: Receiver<F::Frame>,
}

impl<F, R, W> Transport<F, R, W>
where
	F: Framer + Clone + Send + 'static,
	F::Frame: Send + 'static,
	R: AsyncRead + Unpin + Send + 'static,
	W: AsyncWrite + Unpin + Send + 'static,
{
	pub fn new(framer: F, reader: R, writer: W) -> Self {
		let (incoming_tx, incoming) = flume::unbounded();
		let (outgoing, outgoing_rx) = flume::unbounded();
		Self { framer, reader: Some(reader), writer: Some(writer), incoming, incoming_tx, outgoing, outgoing_rx }
	}

	/// Spawn reader and writer tasks. Returns their handles.
	pub fn start(mut self) -> (JoinHandle<()>, JoinHandle<()>) {
		let reader = self.reader.take().expect("Already started");
		let writer = self.writer.take().expect("Already started");
		let framer = self.framer;
		let framer_w = framer.clone();
		let incoming_tx = self.incoming_tx;
		let outgoing_rx = self.outgoing_rx;

		let reader_handle = tokio::spawn(async move {
			let mut buf_reader = BufReader::new(reader);
			loop {
				match framer.read_frame(&mut buf_reader).await {
					Ok(Some(frame)) => {
						if incoming_tx.send(frame).is_err() {
							break;
						}
					}
					Ok(None) => break, // EOF
					Err(e) => {
						error!("Transport read error: {}", e);
						break;
					}
				}
			}
		});

		let writer_handle = tokio::spawn(async move {
			let mut writer = writer;
			while let Ok(frame) = outgoing_rx.recv_async().await {
				let bytes = framer_w.write_frame(&frame);
				if let Err(e) = writer.write_all(&bytes).await {
					error!("Transport write error: {}", e);
					break;
				}
				if let Err(e) = writer.flush().await {
					error!("Transport flush error: {}", e);
					break;
				}
			}
		});

		(reader_handle, writer_handle)
	}
}


