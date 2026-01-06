use dioxus::prelude::*;
use strum::Display;
use tracing::info;

use crate::{
	plugin::{core::plugin::CORE_STATUS_BAR_PANEL, dumy::dumy::Dumy, Plugin, PluginRegistry},
	tool::ToolPlacement,
	PanelConfig, PanelDisplayMode, PanelSocket, Tool, ToolAlignment,
};

const PLUGIN_NAME: &str = "Dumy";

#[derive(Display)]
pub enum DumyPluginPanel {
	#[strum(to_string = "Status dumy bar")]
	StatusBar,
	#[strum(to_string = "Left dumy bar")]
	LeftBar,
}
pub struct DumyPlugin;
pub fn dumy_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: format!("{PLUGIN_NAME} plugin for testing purposes"),
		panels: vec![PanelConfig {
			socket: PanelSocket::Left,
			name: DumyPluginPanel::LeftBar.to_string(),
			display_mode: PanelDisplayMode::Tabbed,
			is_visible: true,
			is_active: false,
			tools: vec![],
			workspaces: vec![],
		}
		.with_tools(vec![("Dumy tool", Dumy, ToolAlignment::default())])],

		// Tool placed in another plugin's panel (Core's status bar)
		tools: vec![Tool {
			placement: ToolPlacement::ByResourceId(CORE_STATUS_BAR_PANEL.clone()),
			name: "Dumy tool".to_string(),
			component: Dumy,
			alignment: ToolAlignment::End,
			workspaces: vec![],
		}],
		entry: Some(dumy_entry),
		..Default::default()
	}
}

fn dumy_entry() -> Element {
	info!("{PLUGIN_NAME} plugin rendering");
	let mut registry = use_context::<Signal<PluginRegistry>>();
	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("hello from {PLUGIN_NAME} plugin!");
	});
	rsx!()
}
