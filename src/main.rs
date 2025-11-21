mod config;
mod top_bar;
mod windows_manager;

use bevy::remote::builtin_methods::{BrpQuery, BrpQueryFilter, BrpQueryParams, ComponentSelector, BRP_QUERY_METHOD};
use bevy::remote::http::{DEFAULT_ADDR, DEFAULT_PORT};
use bevy::remote::BrpRequest;
use serde_json::json;
use top_bar::TopBar;
// Multi-window Dioxus Desktop Editor
use anyhow::Result;
use dioxus::prelude::*;
use std::process::Command;
use std::sync::{Arc, RwLock};

use config::EditorConfig;

#[derive(Clone, Debug)]
struct ScreenLayout {
	screen_width: u32,
	screen_height: u32,
	top_bar_x: i32,
	top_bar_y: i32,
	top_bar_width: u32,
	top_bar_height: u32,
	left_panel_x: i32,
	left_panel_y: i32,
	left_panel_width: u32,
	left_panel_height: u32,
	right_panel_x: i32,
	right_panel_y: i32,
	right_panel_width: u32,
	right_panel_height: u32,
	viewport_x: i32,
	viewport_y: i32,
	viewport_width: u32,
	viewport_height: u32,
}

impl ScreenLayout {
	fn from_screen_size(width: u32, height: u32, config: &EditorConfig) -> Self {
		let top_bar_height: u32 = 100;
		let left_panel_width: u32 = 400;
		let right_panel_width: u32 = 385;
		let margin: u32 = 5;

		// Враховуємо декорації вікна (border + title bar)
		let window_decoration_height = config.window_title_height + config.window_border_size * 2;
		let window_decoration_width = config.window_border_size * 2;

		let top_bar_bottom: u32 = top_bar_height + window_decoration_height + margin;

		let layout = Self {
			screen_width: width,
			screen_height: height,
			top_bar_x: margin as i32,
			top_bar_y: margin as i32,
			top_bar_width: width.saturating_sub(margin * 2),
			top_bar_height,
			left_panel_x: margin as i32,
			left_panel_y: (top_bar_bottom + margin) as i32,
			left_panel_width,
			left_panel_height: height.saturating_sub(top_bar_bottom + margin + window_decoration_height),
			right_panel_x: (width.saturating_sub(right_panel_width + window_decoration_width + margin)) as i32,
			right_panel_y: (top_bar_bottom + margin) as i32,
			right_panel_width,
			right_panel_height: height.saturating_sub(top_bar_bottom + margin + window_decoration_height),
			viewport_x: (left_panel_width + window_decoration_width + margin * 2) as i32,
			viewport_y: (top_bar_bottom + margin) as i32,
			viewport_width: width.saturating_sub(left_panel_width + right_panel_width + window_decoration_width * 2 + margin * 4),
			viewport_height: height.saturating_sub(top_bar_bottom + margin + window_decoration_height),
		};

		println!("📐 Layout calculated:");
		println!(
			"  TopBar: {}x{} at ({}, {})",
			layout.top_bar_width, layout.top_bar_height, layout.top_bar_x, layout.top_bar_y
		);
		println!(
			"  LeftPanel: {}x{} at ({}, {})",
			layout.left_panel_width, layout.left_panel_height, layout.left_panel_x, layout.left_panel_y
		);
		println!(
			"  RightPanel: {}x{} at ({}, {})",
			layout.right_panel_width, layout.right_panel_height, layout.right_panel_x, layout.right_panel_y
		);
		println!(
			"  Viewport: {}x{} at ({}, {})",
			layout.viewport_width, layout.viewport_height, layout.viewport_x, layout.viewport_y
		);

		layout
	}

	fn detect_screen_size(fallback: (u32, u32)) -> (u32, u32) {
		display_info::DisplayInfo::all()
			.ok()
			.and_then(|displays| {
				displays
					.iter()
					.find(|d| d.is_primary)
					.or_else(|| displays.first())
					.map(|display| (display.width, display.height))
			})
			.unwrap_or_else(|| {
				eprintln!("⚠️  Could not detect display, using fallback: {}x{}", fallback.0, fallback.1);
				fallback
			})
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
	println!("🎨 Starting Multi-Window Dioxus Editor...\n");

	let config = EditorConfig::default();
	// Автоматично визначаємо розмір екрану
	let (screen_width, screen_height) = ScreenLayout::detect_screen_size(config.screen_size_fallback);
	let layout = ScreenLayout::from_screen_size(screen_width, screen_height, &config);
	let layout = Arc::new(layout);

	// println!("📐 Screen layout: {}x{}", layout.screen_width, layout.screen_height);

	// // Запускаємо Bevy game як окремий процес
	// spawn_bevy_game(&layout);

	// // Глобальний стан редактора
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

	// // Запускаємо Dioxus UI (3 панелі)
	// // let cfg = dioxus::desktop::Config::new();
	// // Чекаємо трохи, щоб Bevy встигла запуститись
	// std::thread::sleep(std::time::Duration::from_secs(2));

	// // Запускаємо Dioxus UI
	// println!("🚀 Starting Dioxus UI windows...");

	// // Головне вікно (Top Bar)
	let top_bar_cfg = dioxus::desktop::Config::new().with_window(
		dioxus::desktop::WindowBuilder::new()
			.with_title(config.top_bar.title.clone())
			.with_inner_size(dioxus::desktop::LogicalSize::new(layout.top_bar_width, layout.top_bar_height))
			.with_position(dioxus::desktop::LogicalPosition::new(layout.top_bar_x, layout.top_bar_y))
			.with_decorations(true)
			.with_resizable(false),
	);

	// Запускаємо головне вікно
	dioxus::LaunchBuilder::desktop()
		.with_cfg(top_bar_cfg)
		.with_context(editor_state)
		.with_context(layout.clone())
		.launch(App);

	Ok(())
}

#[component]
fn App() -> Element {
	let state = use_context::<Arc<RwLock<EditorState>>>();
	let layout = use_context::<Arc<ScreenLayout>>();

	// Створюємо додаткові вікна через use_effect (після першого рендеру)
	use_effect(move || {
		println!("🪟 Creating additional windows...");
		let window = dioxus::desktop::window();
		let state_clone = state.clone();
		let config = state_clone.read().unwrap().config.clone();
		let layout_clone = layout.clone();

		// Left Panel (Hierarchy)
		let left_cfg = dioxus::desktop::Config::new().with_window(
			dioxus::desktop::WindowBuilder::new()
				.with_title(config.left_panel.title.clone())
				.with_inner_size(dioxus::desktop::LogicalSize::new(
					layout_clone.left_panel_width,
					layout_clone.left_panel_height,
				))
				.with_position(dioxus::desktop::LogicalPosition::new(
					layout_clone.left_panel_x,
					layout_clone.left_panel_y,
				))
				.with_decorations(true)
				.with_resizable(true),
		);
		let mut left_vdom = VirtualDom::new(LeftPanel);
		left_vdom.insert_any_root_context(Box::new(state_clone.clone()));
		window.new_window(left_vdom, left_cfg);

		// Right Panel (Inspector)
		let right_cfg = dioxus::desktop::Config::new().with_window(
			dioxus::desktop::WindowBuilder::new()
				.with_title(config.right_panel.title.clone())
				.with_inner_size(dioxus::desktop::LogicalSize::new(
					layout_clone.right_panel_width,
					layout_clone.right_panel_height,
				))
				.with_position(dioxus::desktop::LogicalPosition::new(
					layout_clone.right_panel_x,
					layout_clone.right_panel_y,
				))
				.with_decorations(true)
				.with_resizable(true),
		);
		let mut right_vdom = VirtualDom::new(RightPanel);
		right_vdom.insert_any_root_context(Box::new(state_clone));
		window.new_window(right_vdom, right_cfg);

		println!("✅ Additional windows created!");
	});

	rsx! {
		style { {include_str!("../assets/editor.css")} }
		TopBar {}
	}
}

#[component]
fn LeftPanel() -> Element {
	let state = use_context::<Arc<RwLock<EditorState>>>();
	let mut selected = use_signal(|| None::<String>);

	// Синхронізувати з глобальним станом
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

fn spawn_bevy_game(layout: &ScreenLayout) {
	println!("🚀 Spawning Bevy game process...");

	let game_path = "../bevy_demo_game/target/debug/bevy_demo_game";
	let viewport_x = layout.viewport_x;
	let viewport_y = layout.viewport_y;
	let viewport_width = layout.viewport_width;
	let viewport_height = layout.viewport_height;

	std::thread::spawn(move || {
		match Command::new(game_path)
			.arg("--editor-mode")
			.arg("--window-x")
			.arg(viewport_x.to_string())
			.arg("--window-y")
			.arg(viewport_y.to_string())
			.arg("--window-width")
			.arg(viewport_width.to_string())
			.arg("--window-height")
			.arg(viewport_height.to_string())
			.spawn()
		{
			Ok(mut child) => {
				println!("✓ Game process started with PID: {:?}", child.id());
				println!(
					"  Viewport: {}x{} at ({}, {})",
					viewport_width, viewport_height, viewport_x, viewport_y
				);
				match child.wait() {
					Ok(status) => println!("Game exited with status: {}", status),
					Err(e) => eprintln!("Error waiting for game: {}", e),
				}
			}
			Err(e) => eprintln!("❌ Failed to spawn game: {}", e),
		}
	});
}
