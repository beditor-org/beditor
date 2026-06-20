use bridge::protocol::camera::CameraInputProtocol;
use dioxus::prelude::*;

use std::sync::{Arc, Mutex};
use tokio::process::{ChildStdin, ChildStdout};

use bridge::{multiplexer::Multiplexer, protocol::frame_stream::FrameStreamProtocol};
use tracing::info;

use crate::plugin::core::plugin::{CORE_SCENE_EDITOR_WORKSPACE, CORE_STATUS_BAR_PANEL};
use crate::plugin::viewport;
use crate::plugin::viewport::frame_counter::FrameCounter;
use crate::tool::ToolPlacement;
use crate::{
	plugin::{viewport::viewport::Viewport, Plugin, PluginRegistry},
	PanelConfig, PanelDisplayMode, PanelSocket,
};
use crate::{Tool, ToolAlignment};

pub struct FrameCounterPlugin;
pub struct ViewportPlugin;
const PLUGIN_NAME: &str = "Viewport";

pub struct ViewportState {
	pub is_opened: bool,
	pub frame_count: usize,
	pub frame: Option<String>,
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
			is_active: false,
			workspaces: vec![CORE_SCENE_EDITOR_WORKSPACE.clone()],
		}
		.with_tools(vec![("Viewport", Viewport, Default::default())])],
		description: "Viewport plugin responsible for reading frames from the game process".to_string(),
		tools: vec![Tool {
			placement: ToolPlacement::ByResourceId(CORE_STATUS_BAR_PANEL.clone()),
			name: "Frame counter".to_string(),
			component: FrameCounter,
			alignment: ToolAlignment::End,
			workspaces: vec![CORE_SCENE_EDITOR_WORKSPACE.clone()],
		}],
		..Default::default()
	}
}

fn setup_context() -> Element {
	use_context_provider(|| {
		Signal::new(ViewportState {
			is_opened: false,
			frame_count: 0,
			frame: None,
		})
	});
	// use_context_provider(|| Signal::new(None::<Arc<Mutex<FrameStreamProtocol>>>));
	use_context_provider(|| Signal::new(None::<Arc<Mutex<CameraInputProtocol>>>));
	rsx!()
}

fn entry() -> Element {
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let mut viewport_state = use_context::<Signal<ViewportState>>();

	// let mut frame_stream = use_context::<Signal<Option<Arc<Mutex<FrameStreamProtocol>>>>>();
	let mut controls_stream = use_context::<Signal<Option<Arc<Mutex<CameraInputProtocol>>>>>();

	// Register channel as soon as multiplexer is available, not waiting for viewport to open
	use_effect(move || {
		if let Some(mux) = multiplexer.read().as_ref() {
			info!("Multiplexer is available, registering things");
			let viewport_stream_protocol = mux.register_protocol::<FrameStreamProtocol>();
			spawn(async move {
				loop {
					match viewport_stream_protocol.connection.recv_async().await {
						Ok(frame) => viewport_state.write().frame = Some(frame),
						Err(_) => break,
					}
				}
			});
		} else {
			info!("Multiplexer is not available, cannot register things yet");
		}

		// let has_no_stream = frame_stream.read().is_none();
		// let has_no_controls = controls_stream.read().is_none();
		// if has_mux {
		// 	if has_no_stream {
		// 		let viewport_stream_protocol = mux.register_protocol::<FrameStreamProtocol>();

		// 		if let Some(mux) = multiplexer.write().as_mut() {
		// 			info!("Registering FrameStream channel");
		// 			// frame_stream.set(Some(Arc::new(Mutex::new())));
		// 		}
		// 	}
		// 	if has_no_controls {
		// 		if let Some(mux) = multiplexer.write().as_mut() {
		// 			info!("Registering Camera channel");
		// 			controls_stream.set(Some(Arc::new(Mutex::new(mux.register_protocol::<CameraInputProtocol>()))));
		// 		}
		// 	}
		// }
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});

	rsx!()
}
