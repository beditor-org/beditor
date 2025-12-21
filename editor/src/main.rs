use std::sync::{Arc, RwLock};

use beditor::{
	components::App,
	event::Events,
	plugins::{
		game_process::GameProcessPlugin, transport::stdio::StdioTransportPlugin, viewport::plugin::ViewportPlugin, CorePlugin,
		DumyPlugin,
	},
	resource::ResourceRegistry,
	EditorConfig, EditorContext, PluginRegistry,
};
use tracing::info;

fn main() {
	let config = EditorConfig::default();
	let editor_state = EditorContext::default();

	let mut registry = PluginRegistry::new();
	registry.register(CorePlugin);
	registry.register(DumyPlugin);
	registry.register(GameProcessPlugin);
	registry.register(StdioTransportPlugin);
	registry.register(ViewportPlugin);
	let resources = ResourceRegistry::new();
	let events = Events::new();
	resources.register(events);
	for (_type_id, plugin) in registry.plugins.iter_mut() {
		info!("Loading plugin: {}", plugin.get_name());
		plugin.on_load(resources.clone());
	}
	resources.get::<Events>().unwrap().publish(beditor::event::DumyEvent);

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.top_bar.title.to_string())
		.with_decorations(false)
		.with_resizable(true);

	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.with_context(Arc::new(registry))
		.with_context(Arc::new(resources))
		.launch(App);
}
