use dioxus::core::Element;

use crate::theme::Theme;

#[derive(Clone)]
pub struct WindowConfig {
	pub title: String,
	pub decorations: bool,
	pub resizable: bool,
	pub maximized: bool,
	pub fullscreen: bool,
	pub size: Option<(f32, f32)>,
	pub position: Option<(f32, f32)>,
}

impl Default for WindowConfig {
	fn default() -> Self {
		Self {
			title: "Vitronix App".to_string(),
			decorations: false,
			resizable: true,
			maximized: true,
			fullscreen: false,
			size: None,
			position: None,
		}
	}
}

#[derive(Clone, Default)]
pub struct Config {
	pub window: WindowConfig,
	pub startup: Option<fn() -> Element>, //	for custom startup flow
	pub initial_theme: Theme,
}
