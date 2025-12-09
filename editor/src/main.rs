use std::sync::{Arc, RwLock};

use beditor::{
	components::App,
	plugins::{CorePlugin, DumyPlugin},
	EditorConfig, EditorContext, PluginRegistry,
};

fn main() {
	let config = EditorConfig::default();
	let editor_state = EditorContext::default();

	let mut registry = PluginRegistry::new();
	registry.register(CorePlugin);
	registry.register(DumyPlugin);

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.top_bar.title.to_string())
		.with_decorations(false)
		.with_resizable(true);

	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.with_context(Arc::new(registry))
		.launch(App);
}
