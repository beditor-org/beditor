use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
	plugin::PluginRegistry,
	tool::{Tool, ToolPlacement},
	ToolAlignment,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SocketAlignment {
	LeftToRight,
	RightToLeft,
	TopToBottom,
	BottomToTop,
}
pub type SocketsConfig = HashMap<PanelSocket, SocketAlignment>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PanelSocket {
	Left,
	Right,
	Top,
	Bottom,
	Center,
	CenterTop,
	CenterBottom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PanelDisplayMode {
	Tabbed,
	Stacked,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanelConfig {
	pub name: String,
	pub is_visible: bool,
	pub socket: PanelSocket,
	pub display_mode: PanelDisplayMode,
	pub tools: Vec<Tool>,
}

impl PanelConfig {
	pub fn with_tools(mut self, tools: Vec<(&str, fn() -> Element, ToolAlignment)>) -> Self {
		self.tools.extend(
			tools
				.iter()
				.map(|(name, component, alignment)| Tool {
					placement: ToolPlacement::PanelByName(self.name.clone()),
					name: name.to_string(),
					component: *component,
					alignment: *alignment,
				})
				.collect::<Vec<Tool>>(),
		);
		self
	}
}
#[derive(Clone, Default)]
pub struct PanelsManager {
	pub panels: Vec<PanelConfig>,
}

impl PanelsManager {
	pub fn from_plugins(registry: &PluginRegistry, // , manager: &PluginsManager
	) -> Self {
		let mut panels = Self::default();
		let mut tools = Vec::new();
		//	collecting pannels from plugins
		registry
			.plugins
			.iter()
			.filter(|(_, state)| state.is_enabled)
			.for_each(|(_, plugin)| {
				panels.panels.extend(plugin.panels.clone());
				tools.extend(plugin.tools.clone());
			});
		info!("processing pannels");

		// now place tools which want to be placed in panels from other plugins
		tools.iter().for_each(|tool| match &tool.placement {
			ToolPlacement::PanelByName(ref panel_name) => {
				if let Some(panel) = panels.panels.iter_mut().find(|p| &p.name == panel_name) {
					info!("Adding tool '{}' to panel '{}'", tool.name, panel_name);
					panel.tools.push(tool.clone());
				} else {
					warn!("Panel with name '{}' not found for tool '{}'", panel_name, tool.name);
				}
			}
			ToolPlacement::PanelByAlignment(_alignment) => todo!(),
			ToolPlacement::OwnPanel(_panel_config) => todo!(),
		});

		panels
	}
}
