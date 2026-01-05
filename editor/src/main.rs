use std::sync::{Arc, RwLock};

use editor::{
	components::App,
	plugin::{
		asset_browser::asset_browser_plugin, core::plugin::core_plugin, dumy::plugin::dumy_plugin,
		game_process::game_process_plugin, transport::stdio::stdio_transport_plugin, viewport::plugin::viewport_plugin,
		PluginBuilder,
	},
	EditorConfig, EditorContext,
};
fn main() {
	let config = EditorConfig::load();
	let editor_state = EditorContext::default();

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.window.title.to_string())
		.with_decorations(config.window.decorations)
		.with_resizable(config.window.resizable);

	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	let plugins: Vec<PluginBuilder> = vec![
		dumy_plugin,
		core_plugin,
		stdio_transport_plugin,
		game_process_plugin,
		viewport_plugin,
		asset_browser_plugin,
	];
	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.with_context(plugins)
		.with_context(config)
		.launch(App);
}
