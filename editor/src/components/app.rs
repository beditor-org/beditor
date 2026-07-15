use dioxus::{core::Element, prelude::*};

use crate::{
	components::EditorLayout,
	event::{Events, SwitchWorkspaceEvent},
	use_init_theme,
	plugin::{Plugin, PluginRegistry},
	workspace::WorkspaceRegistry,
	EditorConfig,
};

#[component]
pub fn App() -> Element {
	info!("rendering App component");
	let plugins = use_context::<Vec<fn() -> Plugin>>();
	let config = use_context::<EditorConfig>();
	let events = use_context_provider(Events::new);
	use_context_provider(|| Signal::new(config));

	let registry = use_context_provider(|| Signal::new(Into::<PluginRegistry>::into(plugins)));

	// Initialize WorkspaceRegistry from plugins BEFORE calling entry() functions
	let workspaces = WorkspaceRegistry::from_plugins(&registry.read());
	let workspace_registry = use_context_provider(|| Signal::new(workspaces));

	// Subscribe to workspace switch events
	use_effect(move || {
		let events = events.clone();
		let mut workspace_registry = workspace_registry.clone();
		events.subscribe::<SwitchWorkspaceEvent>(move |event| {
			workspace_registry.write().set_current(event.0.clone());
		});
	});

	use_init_theme();

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
