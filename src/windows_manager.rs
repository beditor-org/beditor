use crate::config::WindowConfig;

pub struct WindowsManagerConfig {
	pub window_border_size: u32,
	pub window_title_height: u32,
}

impl Default for WindowsManagerConfig {
	fn default() -> Self {
		Self {
			window_border_size: 2,
			window_title_height: 35,
		}
	}
}

#[derive(Default)]
pub struct WindowsManager {
	config: WindowsManagerConfig,
	windows: Vec<dioxus::desktop::DesktopContext>,
}

impl WindowsManager {
	pub fn create_window(&self, window_config: WindowConfig) -> dioxus::desktop::WindowBuilder {
		dioxus::desktop::WindowBuilder::new()
			.with_title(window_config.title.clone())
			// .with_inner_size(dioxus::desktop::LogicalSize::new(window_config.width, window_config.height))
			// .with_position(dioxus::desktop::LogicalPosition::new(window_config.x, window_config.y))
			.with_decorations(true)
			.with_resizable(false)
	}
}
