use dioxus::prelude::*;

use crate::{
	plugin::{core::plugin::CORE_SCENE_EDITOR_WORKSPACE, scene_editor::entity_browser::EntityBrowser, Plugin, PluginRegistry},
	PanelConfig, PanelDisplayMode, PanelSocket, ToolAlignment,
};

const PLUGIN_NAME: &str = "SceneEditor";

pub fn scene_editor_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		panels: vec![PanelConfig {
			name: "Entities".to_string(),
			socket: PanelSocket::Left,
			display_mode: PanelDisplayMode::Tabbed,
			is_visible: true,
			is_active: false,
			tools: vec![],
			workspaces: vec![CORE_SCENE_EDITOR_WORKSPACE.clone()],
		}
		.with_tools(vec![("Entity Browser", EntityBrowser, ToolAlignment::default())])],
		entry: Some(entry),
		..Default::default()
	}
}

fn entry() -> Element {
	let mut registry = use_context::<Signal<PluginRegistry>>();
	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
	});
	rsx!()
}
