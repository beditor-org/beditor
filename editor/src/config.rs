use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, info};

pub const APP_NAME: &str = "Beditor";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
lazy_static! {
	pub static ref WINDOW_TITLE: String = format!("{} v{}", APP_NAME, APP_VERSION);
}
use crate::{
	panel::{SocketAlignment, SocketsConfig},
	PanelSocket,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WindowConfig {
	pub title: String,
	pub decorations: bool,
	pub resizable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecentProject {
	pub name: String,
	pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorConfig {
	pub recent_projects: Vec<RecentProject>,
	pub sockets: SocketsConfig,
	pub window: WindowConfig,
}

impl Default for EditorConfig {
	fn default() -> Self {
		Self {
			window: WindowConfig {
				title: WINDOW_TITLE.clone(),
				resizable: true,
				..Default::default()
			},
			sockets: HashMap::from_iter([
				(PanelSocket::Left, SocketAlignment::LeftToRight),
				(PanelSocket::Right, SocketAlignment::RightToLeft),
				(PanelSocket::Top, SocketAlignment::TopToBottom),
				(PanelSocket::Bottom, SocketAlignment::BottomToTop),
				(PanelSocket::Center, SocketAlignment::LeftToRight),
				(PanelSocket::CenterTop, SocketAlignment::TopToBottom),
				(PanelSocket::CenterBottom, SocketAlignment::TopToBottom),
			]),
			recent_projects: vec![
				RecentProject {
					name: "Project Alpha".to_string(),
					path: "/path/to/alpha".to_string(),
				},
				RecentProject {
					name: "Project Beta".to_string(),
					path: "/path/to/beta".to_string(),
				},
				RecentProject {
					name: "Project Gamma".to_string(),
					path: "/path/to/gamma".to_string(),
				},
			],
		}
	}
}

impl EditorConfig {
	pub fn get_path() -> String {
		let config_dir = dirs::config_dir().expect("Can't detect OS config dir");
		format!("{}/{}.ron", config_dir.display(), APP_NAME.to_lowercase())
	}

	pub fn load() -> Self {
		let config_path = Self::get_path();

		if Path::new(&config_path).exists() {
			// Read and deserialize existing config
			match fs::read_to_string(&config_path) {
				Ok(contents) => match ron::from_str::<EditorConfig>(&contents) {
					Ok(config) => {
						info!("Config loaded from: {}", config_path);
						return config;
					}
					Err(e) => {
						error!("Failed to parse config: {}, using defaults", e);
					}
				},
				Err(e) => {
					error!("Failed to read config file: {}, using defaults", e);
				}
			}
		}

		// Create default config and save it
		let config = Self::default();
		config.save();
		info!("Created default config at: {}", config_path);
		config
	}

	pub fn save(&self) {
		let config_path = Self::get_path();

		// Create parent directory if it doesn't exist
		if let Some(parent) = Path::new(&config_path).parent() {
			let _ = fs::create_dir_all(parent);
		}

		match ron::ser::to_string_pretty(self, Default::default()) {
			Ok(serialized) => {
				if let Err(e) = fs::write(&config_path, serialized) {
					error!("Failed to save config: {}", e);
				}
			}
			Err(e) => {
				error!("Failed to serialize config: {}", e);
			}
		}
	}

	pub fn add_recent_project(&mut self, project: RecentProject) {
		// Remove if already exists (to move it to front)
		self.recent_projects.retain(|p| p.path != project.path);
		// Add to front
		self.recent_projects.insert(0, project);
		// Keep only last 10
		self.recent_projects.truncate(10);
		// Save config
		self.save();
	}
}
