use bridge::protocol::camera::CameraInputProtocol;
use dioxus::prelude::*;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::process::{ChildStdin, ChildStdout};

use bridge::{multiplexer::Multiplexer, protocol::frame_stream::FrameStreamProtocol};
use tracing::info;

use crate::plugin::core::plugin::{CORE_SCENE_EDITOR_WORKSPACE, CORE_STATUS_BAR_PANEL};
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
	pub fps: f32,
	/// Timestamps of frames received in the last second (sliding window).
	pub frame_timestamps: VecDeque<Instant>,
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
			fps: 0.0,
			frame_timestamps: VecDeque::new(),
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
			let camera_input_protocol = mux.register_protocol::<CameraInputProtocol>();
			*controls_stream.write() = Some(Arc::new(Mutex::new(camera_input_protocol)));
			spawn(async move {
				loop {
					// Wait for at least one frame
					let first = match viewport_stream_protocol.connection.recv_async().await {
						Ok(frame) => frame,
						Err(_) => break,
					};
					// Drain any frames that arrived while Dioxus was busy rendering
					// (e.g. editor was backgrounded). Only the latest matters visually.
					let latest = std::iter::once(first)
						.chain(std::iter::from_fn(|| {
							viewport_stream_protocol.connection.try_recv().ok().flatten()
						}))
						.last()
						.unwrap();

					let now = Instant::now();
					let mut state = viewport_state.write();
					state.frame = Some(latest);
					state.frame_count += 1;
					state.frame_timestamps.push_back(now);
					let cutoff = now - std::time::Duration::from_secs(1);
					while state.frame_timestamps.front().map_or(false, |t| *t < cutoff) {
						state.frame_timestamps.pop_front();
					}
					state.fps = state.frame_timestamps.len() as f32;
				}
			});
			info!("Registered CameraInput channel");
		} else {
			info!("Multiplexer is not available, cannot register things yet");
		}
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});

	rsx!()
}
