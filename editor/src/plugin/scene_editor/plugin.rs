use bridge::protocol::bep::{BepProtocol, ComponentData};
use dioxus::prelude::*;

use crate::{
	plugin::{core::plugin::CORE_SCENE_EDITOR_WORKSPACE, scene_editor::entity_browser::EntityBrowser, Plugin, PluginRegistry},
	PanelConfig, PanelDisplayMode, PanelSocket, ToolAlignment,
};

const PLUGIN_NAME: &str = "SceneEditor";

pub fn scene_editor_plugin() -> Plugin {
	let entity_browser = PanelConfig {
		name: "Entity Browser".to_string(),
		socket: PanelSocket::Left,
		display_mode: PanelDisplayMode::Tabbed,
		is_visible: true,
		is_active: false,
		tools: vec![],
		workspaces: vec![CORE_SCENE_EDITOR_WORKSPACE.clone()],
	}
	.with_tools(vec![("Entity Browser", EntityBrowser, ToolAlignment::default())]);

	let inspector_panel = PanelConfig {
		name: "Inspector".to_string(),
		socket: PanelSocket::Left,
		display_mode: PanelDisplayMode::Tabbed,
		is_visible: true,
		is_active: false,
		tools: vec![],
		workspaces: vec![CORE_SCENE_EDITOR_WORKSPACE.clone()],
	}
	.with_tools(vec![(
		"Inspector",
		crate::plugin::scene_editor::inspector::Inspector,
		ToolAlignment::default(),
	)]);

	Plugin {
		name: PLUGIN_NAME.to_string(),
		panels: vec![entity_browser, inspector_panel],
		entry: Some(entry),
		setup_context: Some(setup_context),
		..Default::default()
	}
}

fn setup_context() -> Element {
	use_context_provider(|| Signal::<Vec<ComponentData>>::new(Vec::new()));
	use_context_provider(|| Signal::<Option<u32>>::new(None));

	rsx!()
}

fn entry() -> Element {
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let selected_entity = use_context::<Signal<Option<u32>>>();
	let mut entity_components = use_context::<Signal<Vec<ComponentData>>>();

	use_effect(move || {
		let id = selected_entity.read();
		if let Some(protocol) = try_use_context::<Signal<Option<BepProtocol>>>().and_then(|s| s.read().clone()) {
			if let Some(entity) = *id {
				protocol.select_entity(Some(entity));
			} else {
				protocol.select_entity(None);
				entity_components.set(vec![]);
			}
		}
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
	});
	rsx!()
}
