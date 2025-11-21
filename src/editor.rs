use crate::config::{EditorConfig, WindowConfig};

pub struct Editor {
	config: EditorConfig,
}

impl Editor {
	pub fn new(config: EditorConfig) -> Self {
		Self { config }
	}

	pub fn create_window(&self, window_config: WindowConfig) -> dioxus::desktop::WindowBuilder {
		dioxus::desktop::WindowBuilder::new()
			.with_title(window_config.title.clone())
			.with_inner_size(dioxus::desktop::LogicalSize::new(window_config.width, window_config.height))
			.with_position(dioxus::desktop::LogicalPosition::new(window_config.x, window_config.y))
			.with_decorations(true)
			.with_resizable(false)
	}
}
