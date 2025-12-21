use tracing::info;

use crate::{codec::Codec, transport::Transport};

pub struct RpcClient<P: Codec, T: Transport> {
	pub protocol: P,
	pub transport: T,
}

impl<P: Codec, T: Transport> RpcClient<P, T> {
	pub fn start(&mut self) {
		info!("rpc connected");
		self.transport.start();
	}
}
