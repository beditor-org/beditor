use crate::{components::app::App, config::Config};

pub fn run(config: Config) {
	let window = dioxus::desktop::WindowBuilder::new()
		.with_visible(false)
		.with_decorations(false)
		.with_title(&config.window.title);

	let window_cfg = dioxus::desktop::Config::new().with_menu(None).with_window(window);
	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(config)
		.launch(App);
}
