mod components;
mod config;
mod editor;
mod windows_manager;

use anyhow::Result;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::sync::{Arc, Mutex, RwLock};

use components::App;
use config::EditorConfig;

#[derive(Clone, Debug)]
struct EditorState {
	pub selected_entity: Option<String>,
	// pub entities: Vec<EntityInfo>,
	pub game_connected: bool,
	pub config: EditorConfig,
	// pub game_process: Option<Arc<Mutex<GameProcess>>>,
}

impl Default for EditorState {
	fn default() -> Self {
		Self {
			selected_entity: None,
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

	let editor_state = Arc::new(RwLock::new(EditorState::default()));

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(format!("{}", config.top_bar.title))
		.with_decorations(true)
		.with_resizable(true);
	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(editor_state)
		.launch(App::<EditorState>);

	Ok(())
}
