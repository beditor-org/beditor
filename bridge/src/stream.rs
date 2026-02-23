use anyhow::bail;
use flume::{unbounded, Receiver, Sender};
use tokio::{
	io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
	spawn,
	task::JoinHandle,
};

//	 stream reader/writer to flume channels adapter
pub struct StreamAdapter<R: AsyncRead + Unpin + Send + 'static, W: AsyncWrite + Unpin + Send + 'static> {
	pub receiver: Receiver<Vec<u8>>,
	_sender: Option<Sender<Vec<u8>>>,
	pub sender: Sender<Vec<u8>>,
	_receiver: Option<Receiver<Vec<u8>>>,

	reader: Option<R>,
	writer: Option<W>,
}

impl<R, W> StreamAdapter<R, W>
where
	R: AsyncRead + Unpin + Send + 'static,
	W: AsyncWrite + Unpin + Send + 'static,
{
	pub fn new(reader: R, writer: W) -> Self {
		let (_sender, receiver) = unbounded();
		let (sender, _receiver) = unbounded();

		Self {
			receiver,
			_sender: Some(_sender),
			sender: sender,
			_receiver: Some(_receiver),
			reader: Some(reader),
			writer: Some(writer),
		}
	}

	pub fn listen(&mut self) -> (JoinHandle<anyhow::Result<()>>, JoinHandle<anyhow::Result<()>>) {
		let reader = self.reader.take().expect("Already listening");
		let sender = self._sender.take().expect("Already listening");
		let mut reader_buff = BufReader::new(reader);

		let rh: JoinHandle<anyhow::Result<()>> = spawn(async move {
			let mut buff = String::new();
			while reader_buff.read_line(&mut buff).await? != 0 {
				sender.send(buff.clone().into_bytes())?;
				buff.clear();
			}
			bail!("EOF reached")
		});

		let receiver = self._receiver.take().expect("Already listening");
		let mut writer = self.writer.take().expect("Already listening");
		let wh: JoinHandle<anyhow::Result<()>> = spawn(async move {
			while let Ok(data) = receiver.recv_async().await {
				writer.write_all(&data).await?;
			}
			bail!("Sender disconnected")
		});
		(rh, wh)
	}

	pub async fn send(&self, data: Vec<u8>) {
		self.sender.send(data).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use tokio::{io::AsyncWriteExt, net::UnixStream, time::Duration};

	use super::*;

	#[tokio::test]
	async fn should_read() {
		let data = b"foo\n";
		let (mut s1, s2) = UnixStream::pair().unwrap();
		let (r2, w2) = s2.into_split();
		let mut stream_adapter = StreamAdapter::new(r2, w2);
		stream_adapter.listen();
		s1.write_all(data).await.unwrap();

		let received_data = stream_adapter.receiver.recv_async().await.unwrap();

		assert_eq!(received_data, data);
	}

	#[tokio::test]
	async fn should_write() {
		let data = b"foo\n";
		let (mut s1, s2) = UnixStream::pair().unwrap();
		let (r2, w2) = s2.into_split();
		let mut res = String::new();
		let mut stream_adapter = StreamAdapter::new(r2, w2);
		stream_adapter.listen();
		stream_adapter.send(data.to_vec()).await;
		BufReader::new(&mut s1).read_line(&mut res).await.unwrap();

		assert_eq!(res, "foo\n");
	}

	#[tokio::test]
	async fn handle_eof() {
		let (s1, s2) = UnixStream::pair().unwrap();
		let (r2, w2) = s2.into_split();
		let mut stream_adapter = StreamAdapter::new(r2, w2);
		stream_adapter.listen();

		drop(s1);
		tokio::time::sleep(Duration::from_millis(50)).await;

		assert!(stream_adapter.receiver.is_disconnected());
		assert!(stream_adapter.receiver.try_recv().is_err());
	}

	#[tokio::test]
	async fn handle_read_error() {
		use std::io;
		use tokio_test::io::Builder;

		let mock_reader = Builder::new()
			.read_error(io::Error::new(io::ErrorKind::Other, "simulated read error"))
			.build();

		let (_, w) = UnixStream::pair().unwrap();
		let (_, w2) = w.into_split();

		let mut stream_adapter = StreamAdapter::new(mock_reader, w2);
		stream_adapter.listen();

		tokio::time::sleep(Duration::from_millis(50)).await;

		assert!(stream_adapter.receiver.is_disconnected());
	}

	#[tokio::test]
	async fn handle_receiver_dropped() {
		let (mut s1, s2) = UnixStream::pair().unwrap();
		let (r2, w2) = s2.into_split();
		let mut stream_adapter = StreamAdapter::new(r2, w2);
		stream_adapter.listen();

		drop(stream_adapter.receiver);

		s1.write_all(b"test\n").await.unwrap();

		tokio::time::sleep(Duration::from_millis(50)).await;
	}

	#[tokio::test]
	async fn handle_write_error() {
		use std::io;
		use tokio_test::io::Builder;

		let mock_writer = Builder::new()
			.write_error(io::Error::new(io::ErrorKind::Other, "simulated write error"))
			.build();

		let (r, _) = UnixStream::pair().unwrap();
		let (r2, _) = r.into_split();

		let mut stream_adapter = StreamAdapter::new(r2, mock_writer);
		stream_adapter.listen();
		stream_adapter.sender.send(b"test\n".to_vec()).unwrap();

		tokio::time::sleep(Duration::from_millis(50)).await;
		assert!(stream_adapter.sender.is_disconnected());
	}
}
