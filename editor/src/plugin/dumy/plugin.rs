use dioxus::prelude::*;
use strum::Display;
use tracing::info;

use crate::{
	plugin::{core::CorePluginPanel, dumy::dumy::Dumy, Plugin, PluginRegistry},
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
			tools: vec![],
		}
		.with_tools(vec![("Dumy tool", Dumy, ToolAlignment::default())])],

		//	this tools does not use curent plugin panel and want to be placed in another plugin panel
		tools: vec![Tool {
			//	TODO: maybe place reference to plugin panel by TypeId or something?
			placement: ToolPlacement::PanelByName(CorePluginPanel::StatusBar.to_string()),
			name: "Dumy tool".to_string(),
			component: Dumy,
			alignment: ToolAlignment::End,
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
