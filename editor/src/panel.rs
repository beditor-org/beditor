use std::{any::TypeId, collections::HashMap};

use dioxus::prelude::*;

use crate::{
	plugin::PluginState,
	tool::{Tool, ToolPlacement},
	PluginRegistry, PluginsManager,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PanelAligment {
	#[default]
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
	pub alignment: PanelAligment,
	pub display_mode: PanelDisplayMode,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PanelState {
	pub alignment: PanelAligment,
	pub display_mode: PanelDisplayMode,
	pub tools: Vec<Tool>,
	pub name: String,
	pub is_visible: bool,
	// pub is_open: bool,
	// pub title: String,
	// Additional fields can be added as needed
}

impl Default for PanelState {
	fn default() -> Self {
		Self {
			alignment: PanelAligment::Left,
			display_mode: PanelDisplayMode::Tabbed,
			tools: Vec::new(),
			name: String::new(),
			is_visible: true,
		}
	}
}

impl PanelState {
	pub fn toggle(&mut self) {
		self.is_visible = !self.is_visible;
	}
}

#[derive(Clone, Default)]
pub struct PanelsManager {
	pub panels: Vec<PanelState>,
}

// impl From<HashMap<TypeId, PluginState>> for PanelsManager {
// 	fn from(value: HashMap<TypeId, PluginState>) -> Self {
// 		Self { panels: Vec::new() }
// 	}
// }

impl PanelsManager {
	pub fn add_panel(&mut self, panel: PanelState) {
		info!("Adding panel: {:?}", panel.name);
		self.panels.push(panel);
	}

	pub fn get_panel_by_name(&mut self, name: &str) -> Option<&mut PanelState> {
		self.panels.iter_mut().find(|p| p.name == name)
	}

	pub fn from_plugins(registry: &PluginRegistry, manager: &PluginsManager) -> Self {
		let enabled_plugins = manager.plugins.iter().filter(|(_, state)| state.enabled);
		info!("processing pannels");
		let mut panels = Self::default();

		enabled_plugins.clone().for_each(|(typeid, plugin_state)| {
			if let Some(plugin) = registry.plugins.get(typeid) {
				for panel_cfg in plugin.get_panels() {
					let panel_state = PanelState {
						name: panel_cfg.name,
						alignment: panel_cfg.alignment,
						display_mode: panel_cfg.display_mode,
						..Default::default()
					};
					panels.add_panel(panel_state);
				}
			}
		});

		info!("processing tooks");
		enabled_plugins.for_each(|(typeid, plugin_state)| {
			if let Some(plugin) = registry.plugins.get(typeid) {
				for tool in plugin.get_tools() {
					match tool.placement {
						ToolPlacement::PanelByName(ref panel_name) => {
							if let Some(panel) = panels.get_panel_by_name(&panel_name) {
								info!("Adding tool '{}' to panel '{}'", tool.name, panel_name);
								panel.tools.push(tool);
							} else {
								warn!("Panel with name '{}' not found for tool '{}'", panel_name, tool.name);
							}
						}
						ToolPlacement::PanelByAlignment(_alignment) => {
							todo!()
						}
						ToolPlacement::OwnPanel(_panel_config) => todo!(),
					}
				}
			}
		});
		panels
	}
}
