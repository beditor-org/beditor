use std::future::Future;

use anyhow::bail;
use tokio::{
	io::{AsyncRead, AsyncWrite},
	spawn,
	task::JoinHandle,
};
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;

struct LineFramer;

pub struct StreamHandler<R, W>
where
	R: AsyncRead + Unpin + Send + 'static,
	W: AsyncWrite + Unpin + Send + 'static,
{
	reader: Option<R>,
	writer: Option<W>,
}

impl<R, W> StreamHandler<R, W>
where
	R: AsyncRead + Unpin + Send + 'static,
	W: AsyncWrite + Unpin + Send + 'static,
{
	fn new(reader: R, writer: W) -> Self {
		Self {
			reader: Some(reader),
			writer: Some(writer),
		}
	}

	fn listen<F, Fut>(mut self, mut handler: F) -> JoinHandle<anyhow::Result<()>>
	where
		F: FnMut(Vec<u8>) -> Fut + Send + 'static,
		Fut: Future<Output = anyhow::Result<()>> + Send,
	{
		spawn(async move {
			let reader = self.reader.take().expect("Already listening");
			let framed = FramedRead::new(reader, LineFramer);

			framed.try_for_each(|raw_bytes| async { handler(raw_bytes).await }).await?;

			Ok(())
		})
	}
}

#[cfg(test)]
mod tests {
	use tokio::net::UnixStream;

	use super::*;

	#[tokio::test]
	async fn test_stream_handler() {
		let (mut s1r, s1w) = UnixStream::pair().unwrap();
		StreamHandler::new(s1r, s1w)
			.listen(|data| async move {
				assert_eq!(data, b"foo\n");
				Ok(())
			})
			.await
			.unwrap()
			.unwrap();
	}
}
