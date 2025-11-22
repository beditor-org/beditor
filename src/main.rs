mod config;
mod top_bar;
mod windows_manager;

use bevy::remote::builtin_methods::{BrpQuery, BrpQueryFilter, BrpQueryParams, ComponentSelector, BRP_QUERY_METHOD};
use bevy::remote::http::{DEFAULT_ADDR, DEFAULT_PORT};
use bevy::remote::BrpRequest;
use serde_json::json;
use top_bar::TopBar;

use anyhow::Result;
use dioxus::desktop::tao::dpi::PhysicalPosition;
use dioxus::prelude::*;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::process::Command;
use std::sync::{Arc, RwLock};

use config::EditorConfig;

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
	pub entities: Vec<String>,
	pub brp_connected: bool,
	pub config: EditorConfig,
}

impl Default for EditorState {
	fn default() -> Self {
		Self {
			selected_entity: None,
			entities: vec![],
			brp_connected: false,
			config: EditorConfig::default(),
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	let config = EditorConfig::default();

	let editor_state = Arc::new(RwLock::new(EditorState::default()));

	// let _query_all_req = BrpRequest {
	// 	jsonrpc: String::from("2.0"),
	// 	method: String::from(BRP_QUERY_METHOD),
	// 	id: Some(serde_json::to_value(1)?),
	// 	params: Some(
	// 		serde_json::to_value(BrpQueryParams {
	// 			data: BrpQuery {
	// 				components: Vec::default(),
	// 				option: ComponentSelector::All,
	// 				has: Vec::default(),
	// 			},
	// 			strict: false,
	// 			filter: BrpQueryFilter::default(),
	// 		})
	// 		.expect("Unable to convert query parameters to a valid JSON value"),
	// 	),
	// };
	// println!("...1");

	// let request_body = json!({
	// 	"jsonrpc": "2.0",
	// 	"id": 1,
	// 	"method": "world.query",
	// 	"params": {
	// 		"data": {
	// 			"components": [],
	// 			"option": "all",
	// 			"has": []
	// 		},
	// 		"filter": {
	// 			"with": [],
	// 			"without": []
	// 		},
	// 		"strict": false
	// 	}
	// });

	// let client = reqwest::Client::new();
	// println!("🔌 Connecting to Bevy game at {}:{}...", DEFAULT_ADDR, DEFAULT_PORT);
	// let url = format!("http://{DEFAULT_ADDR}:{DEFAULT_PORT}/");

	// match client.post(url.clone()).json(&request_body).send().await {
	// 	Ok(res) => {
	// 		match res.json::<serde_json::Value>().await {
	// 			Ok(json) => {
	// 				println!("✓ BRP connected! Parsing entities...");

	// 				// Витягуємо імена ентіті з відповіді
	// 				let mut entity_names = Vec::new();
	// 				if let Some(result) = json.get("result").and_then(|r| r.as_array()) {
	// 					for entity_data in result {
	// 						if let Some(components) = entity_data.get("components") {
	// 							if let Some(name_obj) = components.get("bevy_ecs::name::Name") {
	// 								if let Some(name) = name_obj.as_str() {
	// 									entity_names.push(name.to_string());
	// 								}
	// 							}
	// 						}
	// 					}
	// 				}

	// 				println!("📦 Found {} entities: {:?}", entity_names.len(), entity_names);

	// 				// Оновлюємо стан редактора
	// 				if let Ok(mut state) = editor_state.write() {
	// 					state.entities = entity_names;
	// 					state.brp_connected = true;
	// 				}
	// 			}
	// 			Err(e) => eprintln!("❌ Failed to parse BRP response: {}", e),
	// 		}
	// 	}
	// 	Err(e) => {
	// 		eprintln!("❌ Failed to connect to Bevy: {}", e);
	// 		if let Ok(mut state) = editor_state.write() {
	// 			state.brp_connected = false;
	// 		}
	// 	}
	// }

	// println!("...2");
	// // 		let get_transform_request = BrpRequest {
	// // 			jsonrpc: String::from("2.0"),
	// // 			method: String::from(BRP_QUERY_METHOD),
	// // 			id: Some(serde_json::to_value(1)?),
	// // 			params: Some(
	// // 				serde_json::to_value(BrpQueryParams {
	// // 					data: BrpQuery {
	// // 						components: vec![type_name::<Transform>().to_string()],
	// // 						..Default::default()
	// // 					},
	// // 					strict: false,
	// // 					filter: BrpQueryFilter::default(),
	// // 				})
	// // 				.expect("Unable to convert query parameters to a valid JSON value"),
	// // 			),
	// // 		};
	// // thread::spawn(move || {
	// // 	loop {
	// // 		match TcpStream::connect("127.0.0.1:5000") {
	// // 			Ok(mut stream) => {
	// // 				println!("✓ BRP connected to Bevy game!");
	// // 				if let Ok(mut state) = brp_state.write() {
	// // 					state.brp_connected = true;
	// // 				}
	// // 				// Запитуємо список ентіті
	// // 				let _ = stream.write_all(b"entities\n");
	// // 				let mut buf = vec![0u8; 4096];
	// // 				if let Ok(n) = stream.read(&mut buf) {
	// // 					let resp = String::from_utf8_lossy(&buf[..n]);
	// // 					let entities: Vec<String> =
	// // 						resp.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
	// // 					print!("📦 Received entities: {:?}\n", entities);
	// // 					if let Ok(mut state) = brp_state.write() {
	// // 						state.entities = entities;
	// // 					}
	// // 				}
	// // 			}
	// // 			Err(err) => {
	// // 				if let Ok(mut state) = brp_state.write() {
	// // 					eprint!("✗ BRP connection error: {}\n", err);
	// // 					state.brp_connected = false;
	// // 				}
	// // 				thread::sleep(Duration::from_secs(1));
	// // 			}
	// // 		}
	// // 		thread::sleep(Duration::from_secs(2));
	// // 	}
	// // });

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

			spawn_bevy_game_borderless(viewport_x as i32, viewport_y as i32, viewport_width, viewport_height);
			game_spawned.set(true);
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

	// Synchronize with global state
	let state_clone = state.clone();
	use_effect(move || {
		if let Some(sel) = selected() {
			if let Ok(mut s) = state_clone.write() {
				s.selected_entity = Some(sel);
			}
		}
	});

	let entities = state.read().map(|s| s.entities.clone()).unwrap_or_default();
	let brp_connected = state.read().map(|s| s.brp_connected).unwrap_or(false);

	rsx! {
		style { {include_str!("../assets/editor.css")} }
		div {
			class: "panel left-panel",
			h3 { class: "panel-title", "Hierarchy" }

			if !brp_connected {
				div {
					style: "padding: 20px; color: #888;",
					"⏳ Connecting to Bevy..."
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
							name: entity.clone(),
							selected: selected() == Some(entity.clone()),
							onclick: {
								let entity = entity.clone();
								move |_| selected.set(Some(entity.clone()))
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

fn spawn_bevy_game_borderless(x: i32, y: i32, width: u32, height: u32) {
	println!("🚀 Spawning borderless Bevy game window...");

	let game_path = "../bevy_demo_game/target/release/bevy_demo_game";

	std::thread::spawn(move || {
		match Command::new(game_path)
			.arg("--editor-mode")
			.arg("--no-decorations")
			.arg("--window-x")
			.arg(x.to_string())
			.arg("--window-y")
			.arg(y.to_string())
			.arg("--window-width")
			.arg(width.to_string())
			.arg("--window-height")
			.arg(height.to_string())
			.spawn()
		{
			Ok(mut child) => {
				println!("✓ Borderless game window started with PID: {:?}", child.id());
				println!("  Viewport: {}x{} at ({}, {})", width, height, x, y);
				match child.wait() {
					Ok(status) => println!("Game exited with status: {}", status),
					Err(e) => eprintln!("Error waiting for game: {}", e),
				}
			}
			Err(e) => eprintln!("❌ Failed to spawn game: {}", e),
		}
	});
}
