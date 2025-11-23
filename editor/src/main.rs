mod components;
mod config;
mod editor;
mod tool;
mod windows_manager;

use anyhow::Result;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use serde::de;
use std::sync::{Arc, Mutex, RwLock};

use components::{App, PanelAligment, PanelState};
use config::EditorConfig;
use tool::Tool;

#[derive(Clone, Debug)]
struct EditorState {
	pub selected_entity: Option<String>,
	// pub entities: Vec<EntityInfo>,
	pub game_connected: bool,
	pub config: EditorConfig,
	pub panels: Vec<components::PanelState>,
	// pub game_process: Option<Arc<Mutex<GameProcess>>>,
}

impl Default for EditorState {
	fn default() -> Self {
		Self {
			selected_entity: None,
			panels: Vec::new(),
			// entities: vec![],
			game_connected: false,
			config: EditorConfig::default(),
			// game_process: None,
		}
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	let config = EditorConfig::default();

	let editor_state = EditorState {
		panels: vec![
			PanelState {
				alignment: PanelAligment::Top,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Bottom,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Left,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Right,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Right,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Right,
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::CenterBottom,
				..Default::default()
			},
		],
		..Default::default()
	};

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(format!("{}", config.top_bar.title))
		.with_decorations(true)
		.with_resizable(true);
	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.launch(App::<EditorState>);

	Ok(())
}
