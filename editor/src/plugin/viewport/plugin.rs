use dioxus::prelude::*;

use std::process::{ChildStdin, ChildStdout};
use std::sync::{Arc, Mutex};

use bridge::{
	codec::base64::Base64Codec, connection::Connection, multiplexer::Multiplexer, protocol::frame_stream::FrameStreamProtocol,
};
use tracing::info;

use crate::plugin::core::CorePluginPanel;
use crate::plugin::viewport::frame_counter::FrameCounter;
use crate::tool::ToolPlacement;
use crate::{
	plugin::{viewport::viewport::Viewport, Plugin, PluginRegistry},
	PanelConfig, PanelDisplayMode, PanelSocket,
};
use crate::{Tool, ToolAlignment};

pub struct FrameCounterPlugin;
pub struct ViewportPlugin;
const PLUGIN_NAME: &str = "Viewport Stream";

pub struct ViewportState {
	pub is_opened: bool,
	pub frame_count: usize,
}

pub fn viewport_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		entry: Some(entry),
		setup_context: Some(setup_context),
		panels: vec![PanelConfig {
			name: "Viewport".to_string(),
			socket: PanelSocket::Center,
			display_mode: PanelDisplayMode::Tabbed,
			is_visible: true,
			tools: vec![],
		}
		.with_tools(vec![("Viewport", Viewport, Default::default())])],
		description: "Viewport plugin responsible for reading frames from the game process".to_string(),
		tools: vec![Tool {
			placement: ToolPlacement::PanelByName(CorePluginPanel::StatusBar.to_string()),
			name: "Dumy tool".to_string(),
			component: FrameCounter,
			alignment: ToolAlignment::End,
		}],
		..Default::default()
	}
}

fn setup_context() -> Element {
	use_context_provider(|| {
		Signal::new(ViewportState {
			is_opened: false,
			frame_count: 0,
		})
	});
	use_context_provider(|| Signal::new(None::<Arc<Mutex<FrameStreamProtocol>>>));
	rsx!()
}

fn entry() -> Element {
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let mut multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let mut frame_stream = use_context::<Signal<Option<Arc<Mutex<FrameStreamProtocol>>>>>();
	let viewport_state = use_context::<Signal<ViewportState>>();

	use_effect(move || {
		let is_opened = viewport_state.read().is_opened;
		let has_mux = multiplexer.read().is_some();
		let has_no_stream = frame_stream.read().is_none();

		if is_opened && has_mux && has_no_stream {
			if let Some(mux) = multiplexer.write().as_mut() {
				info!("Connecting to stream");
				let connection = Connection::new(
					Base64Codec,
					mux.register_for_type::<FrameStreamProtocol>(),
					mux.get_writer_for_type::<FrameStreamProtocol>(),
				);
				frame_stream.set(Some(Arc::new(Mutex::new(FrameStreamProtocol { connection }))));
			}
		}
	});
	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});

	rsx!()
}
