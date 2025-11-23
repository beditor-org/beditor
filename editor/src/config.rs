#[derive(Clone, Debug, Default)]
pub struct WindowConfig {
	pub width: u32,
	pub height: u32,
	pub title: String,
	pub decorations: bool,
	pub resizable: bool,
}

#[derive(Clone, Debug)]
pub struct EditorConfig {
	pub top_bar: WindowConfig,
	pub left_panel: WindowConfig,
	pub right_panel: WindowConfig,
	pub screen_size_fallback: (u32, u32), //	If screen size detection fails, use this size
	pub window_border_size: u32,
	pub window_title_height: u32,
}

const APP_NAME: &str = "Beditor";

impl Default for EditorConfig {
	fn default() -> Self {
		Self {
			top_bar: WindowConfig {
				width: 800,
				height: 600,
				title: format!("{APP_NAME} v0.1.0"),
				..Default::default()
			},
			left_panel: WindowConfig {
				width: 400,
				height: 300,
				title: format!("{APP_NAME} - Hierarchy"),
				..Default::default()
			},
			right_panel: WindowConfig {
				width: 400,
				height: 300,
				title: format!("{APP_NAME} - Inspector"),
				..Default::default()
			},
			screen_size_fallback: (1920, 1080),
			window_border_size: 2,
			window_title_height: 35,
		}
	}
}
