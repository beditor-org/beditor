pub mod asset_browser;
pub mod core;
pub mod dumy;
pub mod game_process;
// pub mod scene_editor;
pub mod brp;
pub mod transport;
pub mod viewport;

use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{workspace::Workspace, PanelConfig, Tool};

pub type PluginComponent = fn() -> Element;

#[derive(Clone, Default)]
pub struct Plugin {
	pub is_enabled: bool,
	pub is_initialized: bool,
	pub name: String,
	pub entry: Option<PluginComponent>,
	pub setup_context: Option<PluginComponent>,
	pub description: String,
	pub tools: Vec<Tool>,
	pub workspaces: Vec<Workspace>,
	pub panels: Vec<PanelConfig>,
}
impl Plugin {
	pub fn with_panels(&mut self, panels: Vec<PanelConfig>) -> &mut Self {
		self
	}

	pub fn with_tools(&mut self, tools: Vec<Tool>) -> &mut Self {
		self.tools.extend(tools);
		self
	}
}

pub type PluginBuilder = fn() -> Plugin;

#[derive(Clone)]
pub struct PluginRegistry {
	pub plugins: HashMap<String, Plugin>,
}

impl Default for PluginRegistry {
	fn default() -> Self {
		Self::new()
	}
}

impl PluginRegistry {
	pub fn new() -> Self {
		Self { plugins: HashMap::new() }
	}

	pub fn register(&mut self, plugin: Plugin) {
		let plugin_name = plugin.name.clone();
		match self.plugins.get(&plugin_name) {
			Some(_) => warn!("PluginRegistry: Plugin with name '{plugin_name}' is already registered."),
			None => {
				self.plugins.insert(plugin_name.clone(), plugin);
			}
		};
	}
}

impl From<Vec<PluginBuilder>> for PluginRegistry {
	fn from(value: Vec<PluginBuilder>) -> Self {
		let mut registry = Self::new();
		for plugin_builder in value {
			let mut plugin = plugin_builder();
			plugin.is_enabled = true;
			registry.register(plugin);
		}
		registry
	}
}
