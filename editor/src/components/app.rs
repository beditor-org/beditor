use dioxus::{core::Element, prelude::*};

use crate::{
	components::EditorLayout,
	event::Events,
	init_theme,
	plugin::{Plugin, PluginRegistry},
	workspace::WorkspaceRegistry,
};

#[component]
pub fn App() -> Element {
	info!("rendering App component");
	let plugins = use_context::<Vec<fn() -> Plugin>>();
	use_context_provider(Events::new);

	let registry = use_context_provider(|| Signal::new(Into::<PluginRegistry>::into(plugins)));

	// Initialize WorkspaceRegistry from plugins BEFORE calling entry() functions
	let workspaces = WorkspaceRegistry::from_plugins(&registry.read());
	use_context_provider(|| Signal::new(workspaces));

	init_theme();

	let all_initialised = use_memo(move || {
		registry
			.read()
			.plugins
			.values()
			.all(|plugin| plugin.entry.is_none() || plugin.is_initialized)
	});
	info!("Plugins all_initialised: {all_initialised}");
	let plugins = registry.read().plugins.clone();
	rsx! {
		// Phase 1: Init contexts for all plugins
		for (_, plugin) in &plugins {
			if let Some(setup_context) = &plugin.setup_context {
				{setup_context()}
			}
		}

		// Phase 2: All initialize plugins
		for (_, plugin) in &plugins {
			if let Some(entry) = &plugin.entry {
				{entry()}
			}
		}

		style { {include_str!("../../public/main.css")} }
		if all_initialised() {
			EditorLayout {}
		} else {
			div { "Loading plugins..." }
		}
	}
}
