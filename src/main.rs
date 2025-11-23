mod config;
mod editor;
mod top_bar;
mod windows_manager;
use top_bar::TopBar;

use anyhow::Result;
use bevy::remote::BrpRequest;
use dioxus::desktop::tao::dpi::PhysicalPosition;
use dioxus::prelude::*;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use config::EditorConfig;

// Owned BRP response structures for deserialization
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OwnedBrpResponse {
	jsonrpc: String,
	id: u64,
	#[serde(flatten)]
	payload: OwnedBrpPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum OwnedBrpPayload {
	Result { result: serde_json::Value },
	Error { error: OwnedBrpError },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OwnedBrpError {
	code: i32,
	message: String,
}

// Simple entity info for UI display
#[derive(Debug, Clone)]
struct EntityInfo {
	id: u64,
	name: String,
}

// Request ID counter
static REQUEST_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

// Game process handle
#[derive(Debug)]
struct GameProcess {
	stdin: ChildStdin,
	_child: Child,
}

fn get_window_position(window: &dioxus::desktop::tao::window::Window) -> Option<(i32, i32)> {
	#[cfg(target_os = "linux")]
	{
		use std::ptr;
		use x11::xlib;

		unsafe {
			let handle = window.window_handle().ok()?;
			let raw_handle = handle.as_raw();

			// First try X11
			if let RawWindowHandle::Xlib(xlib_handle) = raw_handle {
				let display = xlib::XOpenDisplay(ptr::null());
				if !display.is_null() {
					let window_id = xlib_handle.window;
					let mut x = 0;
					let mut y = 0;
					let mut child = 0;
					let root = xlib::XDefaultRootWindow(display);

					xlib::XTranslateCoordinates(display, window_id, root, 0, 0, &mut x, &mut y, &mut child);
					xlib::XCloseDisplay(display);
					return Some((x, y));
				}
			}
			window.outer_position().ok().map(|pos| (pos.x, pos.y))
		}
	}

	#[cfg(not(target_os = "linux"))]
	{
		window.outer_position().ok().map(|pos| (pos.x, pos.y))
	}
}

#[derive(Clone, Debug)]
struct EditorState {
	pub selected_entity: Option<String>,
	pub entities: Vec<EntityInfo>,
	pub game_connected: bool,
	pub config: EditorConfig,
	pub game_process: Option<Arc<Mutex<GameProcess>>>,
}

impl Default for EditorState {
	fn default() -> Self {
		Self {
			selected_entity: None,
			entities: vec![],
			game_connected: false,
			config: EditorConfig::default(),
			game_process: None,
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	let config = EditorConfig::default();

	let editor_state = Arc::new(RwLock::new(EditorState::default()));

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(format!("{}", config.top_bar.title))
		.with_decorations(true)
		.with_resizable(true);
	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(editor_state)
		.launch(App);

	Ok(())
}

#[component]
fn App() -> Element {
	let state = use_context::<Arc<RwLock<EditorState>>>();

	let mut game_spawned = use_signal(|| false);

	use_effect(move || {
		if game_spawned() {
			return;
		}

		let state_for_spawn = state.clone();
		spawn(async move {
			// Minimum delay so that UI has time to render
			tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

			let window = &dioxus::desktop::window().window;
			let size = window.inner_size();

			let window_position = get_window_position(window.as_ref())
				.map(|(x, y)| PhysicalPosition::new(x, y))
				.unwrap_or_else(|| PhysicalPosition::new(0, 0));

			println!("✓ Window position: ({}, {})", window_position.x, window_position.y);
			println!("✓ Window size: {}x{}", size.width, size.height);

			let top_bar_height = 100;
			let left_panel_width = 300;
			let right_panel_width = 350;

			let viewport_x = window_position.x + left_panel_width as i32;
			let viewport_y = window_position.y + top_bar_height as i32 + 2;
			let viewport_width = size.width.saturating_sub(left_panel_width + right_panel_width);
			let viewport_height = size.height.saturating_sub(top_bar_height + 35);
			println!(
				"  Viewport screen position: {}x{} at ({}, {})",
				viewport_width, viewport_height, viewport_x, viewport_y
			);

			// spawn_bevy_game_borderless(
			// 	viewport_x as i32,
			// 	viewport_y as i32,
			// 	viewport_width,
			// 	viewport_height,
			// 	state_for_spawn,
			// );
			// game_spawned.set(true);
		});
	});

	rsx! {
		style { {include_str!("../assets/editor.css")} }

		div {
			class: "editor-container",
			style: "display: flex; flex-direction: column; height: 100vh; width: 100vw;",

			// Top Bar
			div {
				class: "top-bar",
				style: "height: 100px; background: #2d2d2d; border-bottom: 1px solid #1e1e1e;",
				TopBar {}
			}

			// Main content area
			div {
				class: "main-content",
				style: "display: flex; flex: 1; overflow: hidden;",

				// Left Panel (Hierarchy)
				div {
					class: "left-panel",
					style: "width: 300px; background: #252526; border-right: 1px solid #1e1e1e; overflow-y: auto;",
					LeftPanel {}
				}

				// Center Viewport (placeholder for Bevy game)
				div {
					class: "viewport",
					style: "flex: 1; background: #1e1e1e; display: flex; align-items: center; justify-content: center; color: #888;",
					"🎮 Game Viewport (Bevy window will overlay here)"
				}

				// Right Panel (Inspector)
				div {
					class: "right-panel",
					style: "width: 350px; background: #252526; border-left: 1px solid #1e1e1e; overflow-y: auto;",
					RightPanel {}
				}
			}
		}
	}
}

#[component]
fn LeftPanel() -> Element {
	let state = use_context::<Arc<RwLock<EditorState>>>();
	let mut selected = use_signal(|| None::<String>);
	let mut update_trigger = use_signal(|| 0u32);

	// Synchronize with global state
	let state_clone = state.clone();
	use_effect(move || {
		if let Some(sel) = selected() {
			if let Ok(mut s) = state_clone.write() {
				s.selected_entity = Some(sel);
			}
		}
	});

	// Poll for state changes periodically (less frequently to reduce CPU usage)
	use_effect(move || {
		spawn(async move {
			loop {
				tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
				update_trigger.set(update_trigger() + 1);
			}
		});
	});

	// Read state reactively
	let _ = update_trigger(); // Subscribe to updates
	let entities = state.read().map(|s| s.entities.clone()).unwrap_or_default();
	let game_connected = state.read().map(|s| s.game_connected).unwrap_or(false);

	rsx! {
		style { {include_str!("../assets/editor.css")} }
		div {
			class: "panel left-panel",
			h3 { class: "panel-title", "Hierarchy" }

			if !game_connected {
				div {
					style: "padding: 20px; color: #888;",
					"⏳ Connecting to Game..."
				}
			} else if entities.is_empty() {
				div {
					style: "padding: 20px; color: #888;",
					"📦 No entities found"
				}
			} else {
				div { class: "tree-view",
					for entity in entities.iter() {
						TreeItem {
							name: entity.name.clone(),
							selected: selected() == Some(entity.name.clone()),
							onclick: {
								let entity_name = entity.name.clone();
								move |_| selected.set(Some(entity_name.clone()))
							}
						}
					}
				}
			}
		}
	}
}

#[component]
fn TreeItem(name: String, selected: bool, onclick: EventHandler<MouseEvent>) -> Element {
	let class_name = if selected {
		"tree-item tree-item-selected"
	} else {
		"tree-item"
	};

	rsx! {
		div {
			class: class_name,
			onclick: move |evt| onclick.call(evt),
			"▸ {name}"
		}
	}
}

#[component]
fn RightPanel() -> Element {
	let state = use_context::<Arc<RwLock<EditorState>>>();

	let selected_name = use_memo(move || {
		state
			.read()
			.ok()
			.and_then(|s| s.selected_entity.clone())
			.unwrap_or_else(|| "Nothing selected".to_string())
	});

	rsx! {
		style { {include_str!("../assets/editor.css")} }
		div {
			class: "panel right-panel",
			h3 { class: "panel-title", "Inspector" }

			if selected_name() != "Nothing selected" {
				div { class: "properties",
					h4 { style: "color: #ccc; margin: 10px 0;", "{selected_name}" }

					PropertyGroup { title: "Transform" }
					Property { label: "Position X", value: "0.0" }
					Property { label: "Position Y", value: "0.0" }
					Property { label: "Position Z", value: "0.0" }

					Property { label: "Rotation X", value: "0.0" }
					Property { label: "Rotation Y", value: "0.0" }
					Property { label: "Rotation Z", value: "0.0" }
				}
			} else {
				div {
					style: "padding: 20px; color: #888;",
					"Select an entity to inspect"
				}
			}
		}
	}
}

#[component]
fn PropertyGroup(title: String) -> Element {
	rsx! {
		div {
			class: "property-group",
			style: "margin: 10px 0; padding: 5px 0; border-bottom: 1px solid #3c3c3c;",
			span {
				style: "color: #aaa; font-weight: bold; font-size: 12px;",
				"{title}"
			}
		}
	}
}

#[component]
fn Property(label: String, value: String) -> Element {
	rsx! {
		div {
			class: "property-row",
			style: "display: flex; justify-content: space-between; padding: 5px 0;",
			label {
				style: "color: #aaa; font-size: 12px;",
				"{label}:"
			}
			input {
				r#type: "number",
				value: "{value}",
				style: "width: 100px; background: #1e1e1e; border: 1px solid #3c3c3c; color: #ccc; padding: 2px 5px; border-radius: 3px;",
			}
		}
	}
}

fn send_brp_request(game_process: &Arc<Mutex<GameProcess>>, request: BrpRequest) {
	if let Ok(mut process) = game_process.lock() {
		if let Ok(json) = serde_json::to_string(&request) {
			if let Err(e) = writeln!(process.stdin, "{}", json) {
				eprintln!("❌ Failed to send BRP request to game: {}", e);
			}
		}
	}
}

fn send_brp_query_entities(game_process: &Arc<Mutex<GameProcess>>) {
	let request = BrpRequest {
		jsonrpc: "2.0".to_string(),
		method: "world.query".to_string(),
		id: Some(json!(REQUEST_ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
		params: Some(json!({
			"data": {
				"components": ["bevy_ecs::name::Name"],
				"option": "all",
				"has": []
			},
			"filter": {
				"with": [],
				"without": []
			},
			"strict": false
		})),
	};

	send_brp_request(game_process, request);
}

fn handle_brp_response(response: OwnedBrpResponse, state: &Arc<RwLock<EditorState>>) {
	match response.payload {
		OwnedBrpPayload::Result { result } => {
			// Try to parse as entity query result
			if let Some(entities_array) = result.as_array() {
				let mut entities = Vec::new();

				for entity_data in entities_array.iter() {
					if let Some(components) = entity_data.get("components") {
						// Extract entity ID
						let entity_id = entity_data.get("entity").and_then(|e| e.as_u64()).unwrap_or(0);

						// Extract Name component
						if let Some(name_val) = components.get("bevy_ecs::name::Name") {
							let name = name_val.as_str().unwrap_or("Unknown").to_string();
							entities.push(EntityInfo { id: entity_id, name });
						}
					}
				}

				if let Ok(mut s) = state.write() {
					s.entities = entities;
				}
			}
		}
		OwnedBrpPayload::Error { error } => {
			eprintln!("❌ BRP error: {}: {}", error.code, error.message);
		}
	}
}
