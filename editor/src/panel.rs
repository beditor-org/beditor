use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
	plugin::PluginRegistry,
	tool::{Tool, ToolPlacement},
	ResourceId, ToolAlignment,
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
	pub is_visible: bool, // is made visible by user
	pub is_active: bool,  // is present on current workspace
	pub socket: PanelSocket,
	pub display_mode: PanelDisplayMode,
	pub tools: Vec<Tool>,
	/// List of workspace IDs where this panel should be available
	pub workspaces: Vec<ResourceId>,
}

impl PanelConfig {
	pub fn with_tools(mut self, tools: Vec<(&str, fn() -> Element, ToolAlignment)>) -> Self {
		self.tools.extend(
			tools
				.iter()
				.map(|(name, component, alignment)| Tool {
					placement: ToolPlacement::ByResourceId(ResourceId::panel(&self.name, name)),
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
	pub panels: HashMap<ResourceId, PanelConfig>,
}

impl PanelsManager {
	pub fn make_active_for_workspace(&mut self, workspace: &crate::workspace::Workspace) {
		self.panels.iter_mut().for_each(|(id, panel)| {
			panel.is_active = workspace.panels.contains(id);
		});
	}

	pub fn from_plugins(registry: &PluginRegistry, // , manager: &PluginsManager
	) -> Self {
		let mut panels_manager = Self::default();
		let mut tools = Vec::new();
		// Collect panels from enabled plugins
		registry
			.plugins
			.iter()
			.filter(|(_, state)| state.is_enabled)
			.for_each(|(_, plugin)| {
				plugin.panels.iter().for_each(|panel| {
					panels_manager
						.panels
						.insert(ResourceId::panel(&plugin.name, &panel.name), panel.clone());
				});
				tools.extend(plugin.tools.clone());
			});
		info!("Processing panels");

		// Place tools that want to be placed in panels from other plugins
		tools.iter().for_each(|tool| match &tool.placement {
			ToolPlacement::ByResourceId(ref resource_id) => {
				if let Some((_, mut panel)) = panels_manager.clone().panels.into_iter().find(|(id, _)| id == resource_id) {
					info!("Adding tool '{}' to panel '{}'", tool.name, panel.name);
					panel.tools.push(tool.clone());
				} else {
					warn!(
						"Panel with resource id '{:?}' not found for tool '{}'",
						resource_id, tool.name
					);
				}
			}
			ToolPlacement::PanelByAlignment(_alignment) => todo!(),
			ToolPlacement::OwnPanel(_panel_config) => todo!(),
		});

		panels_manager
	}
}
