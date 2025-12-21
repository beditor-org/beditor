use std::{
	process::{ChildStdin, ChildStdout},
	sync::{Arc, Mutex},
};

use bridge::{
	codec::base64::Base64Codec, connection::Connection, multiplexer::Multiplexer, protocol::frame_stream::FrameStreamProtocol,
};
use tracing::info;

use crate::{
	event::Events,
	plugins::{game_process::RenderViewportEvent, transport::stdio::StdioTransportReadyEvent},
	resource::ResourceRegistry,
	Plugin,
};

pub struct FrameCounterPlugin;
pub struct ViewportPlugin;
impl Plugin for ViewportPlugin {
	fn get_name(&self) -> String {
		"ViewportStreamPlugin".to_string()
	}

	fn on_load(&mut self, resources: ResourceRegistry) {
		let events = resources.get::<Events>().unwrap();

		let resources_clone = resources.clone();
		let e_clone = events.clone();
		events.subscribe::<StdioTransportReadyEvent>(move |_| {
			info!("GameProcessStartedEvent in ViewportStreamPlugin");
			let multiplexer = resources_clone.get::<Multiplexer<ChildStdout, ChildStdin>>().unwrap();
			let connection = Connection::new(
				Base64Codec,
				multiplexer.register_for_type::<FrameStreamProtocol>(),
				multiplexer.get_writer_for_type::<FrameStreamProtocol>(),
			);
			let protocol = FrameStreamProtocol { connection };
			resources_clone.register(Arc::new(Mutex::new(protocol)));
			e_clone.publish(RenderViewportEvent {});
			info!("Registered FrameStreamProtocol for viewport streaming");
		});
	}

	fn get_description(&self) -> String {
		todo!()
	}
}
