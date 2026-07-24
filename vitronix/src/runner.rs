use crate::{components::app::App, config::Config};

pub fn run(config: Config) {
	let mut window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.window.title.to_string())
		.with_decorations(config.window.decorations)
		.with_resizable(config.window.resizable)
		.with_visible(false);
	if let Some((width, height)) = config.window.size {
		window = window.with_inner_size(dioxus::desktop::LogicalSize::new(width, height));
	}
	if let Some((x, y)) = config.window.position {
		window = window.with_position(dioxus::desktop::LogicalPosition::new(x, y));
	}

	let window_cfg = dioxus::desktop::Config::new()
		.with_window(window)
		.with_background_color((0, 0, 0, 0));
	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(config)
		.launch(App);
}
