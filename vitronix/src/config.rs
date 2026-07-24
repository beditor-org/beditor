use dioxus::core::Element;

#[derive(Clone)]
pub struct WindowConfig {
	pub title: String,
	pub decorations: bool,
	pub resizable: bool,
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
			fullscreen: false,
			size: None,
			position: None,
		}
	}
}

#[derive(Clone)]
pub struct Config {
	pub window: WindowConfig,
	pub app: fn() -> Element,
}
