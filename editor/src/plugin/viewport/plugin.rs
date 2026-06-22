use bridge::protocol::camera::CameraInputProtocol;
use bridge::protocol::frame_stream::FrameStreamProtocol;
use dioxus::prelude::*;
use memmap2::Mmap;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::process::{ChildStdin, ChildStdout};

use bridge::multiplexer::Multiplexer;
use tracing::info;

use crate::plugin::core::plugin::{CORE_SCENE_EDITOR_WORKSPACE, CORE_STATUS_BAR_PANEL};
use crate::plugin::game_process::viewport_shm_path;
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
	use_context_provider(|| Signal::new(None::<Arc<Mutex<CameraInputProtocol>>>));
	rsx!()
}

fn entry() -> Element {
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let mut controls_stream = use_context::<Signal<Option<Arc<Mutex<CameraInputProtocol>>>>>();

	// Register protocols as soon as multiplexer is available.
	// Camera input uses multiplexer directly.
	// Viewport frame notifications also come through multiplexer (tiny signal),
	// while the actual JPEG data is read from the shared-memory file.
	use_effect(move || {
		if let Some(mux) = multiplexer.read().as_ref() {
			info!("Multiplexer is available, registering viewport + camera protocols");

			let camera_input_protocol = mux.register_protocol::<CameraInputProtocol>();
			*controls_stream.write() = Some(Arc::new(Mutex::new(camera_input_protocol)));

			let frame_stream_protocol = mux.register_protocol::<FrameStreamProtocol>();
			let shm_path = viewport_shm_path();

			spawn(async move {
				// Map the shared-memory file created by game_process::spawn().
				// The game writes [len: u32 BE][jpeg...] there and then sends an
				// empty signal through FrameStreamProtocol to notify us.
				let mmap: Mmap = {
					let file = match std::fs::File::open(&shm_path) {
						Ok(f) => f,
						Err(e) => {
							tracing::error!("Cannot open viewport shm {shm_path:?}: {e}");
							return;
						}
					};
					match unsafe { Mmap::map(&file) } {
						Ok(m) => m,
						Err(e) => {
							tracing::error!("Cannot mmap viewport shm: {e}");
							return;
						}
					}
				};
				info!("Viewport shm mapped ({} bytes)", mmap.len());

				loop {
					// Block until game signals a new frame is ready
					match frame_stream_protocol.connection.recv_async().await {
						Ok(_signal) => {}
						Err(_) => break,
					}
					// Drain any queued signals — only the latest matters
					while frame_stream_protocol.connection.try_recv().ok().flatten().is_some() {}

					// Read frame from shm: [len: u32 BE][jpeg data...]
					if mmap.len() < 4 {
						continue;
					}
					let len = u32::from_be_bytes(mmap[0..4].try_into().unwrap()) as usize;
					if len == 0 || 4 + len > mmap.len() {
						continue;
					}
					let data = &mmap[4..4 + len];

					let now = Instant::now();
					use base64::{engine::general_purpose, Engine as _};
					let mut state = viewport_state.write();
					state.frame = Some(general_purpose::STANDARD.encode(data));
					state.frame_count += 1;
					state.frame_timestamps.push_back(now);
					let cutoff = now - std::time::Duration::from_secs(1);
					while state.frame_timestamps.front().map_or(false, |t| *t < cutoff) {
						state.frame_timestamps.pop_front();
					}
					state.fps = state.frame_timestamps.len() as f32;
				}
				info!("Viewport frame stream ended");
			});
		} else {
			info!("Multiplexer is not available yet");
		}
	});

	// Periodically prune stale timestamps so FPS counter drops to 0 when game disconnects.
	use_future(move || async move {
		loop {
			tokio::time::sleep(std::time::Duration::from_millis(500)).await;
			let mut state = viewport_state.write();
			let now = Instant::now();
			let cutoff = now - std::time::Duration::from_secs(1);
			while state.frame_timestamps.front().map_or(false, |t| *t < cutoff) {
				state.frame_timestamps.pop_front();
			}
			state.fps = state.frame_timestamps.len() as f32;
		}
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});

	rsx!()
}
