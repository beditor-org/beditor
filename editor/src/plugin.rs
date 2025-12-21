use std::{any::TypeId, collections::HashMap};

use dioxus::prelude::*;

use crate::{resource::ResourceRegistry, PanelConfig, Tool};

pub trait Plugin {
	fn get_name(&self) -> String;
	fn get_description(&self) -> String;
	fn get_tools(&self) -> Vec<Tool> {
		vec![]
	}
	fn get_panels(&self) -> Vec<PanelConfig> {
		vec![]
	}
	fn on_load(&mut self, _: ResourceRegistry) {}
	fn on_unload(&mut self, _: ResourceRegistry) {}
}

pub struct PluginRegistry {
	pub plugins: HashMap<TypeId, Box<dyn Plugin + Send + Sync>>,
}

impl PluginRegistry {
	pub fn new() -> Self {
		Self { plugins: HashMap::new() }
	}

	pub fn register<T: Plugin + 'static + Send + Sync>(&mut self, plugin: T) {
		let type_id = TypeId::of::<T>();
		self.plugins.insert(type_id, Box::new(plugin));
	}
}

#[derive(Clone)]
pub struct PluginState {
	pub enabled: bool,
}

impl PluginState {
	pub fn toggle(&mut self) {
		self.enabled = !self.enabled;
	}
}

#[derive(Clone)]
pub struct PluginsManager {
	pub plugins: HashMap<TypeId, PluginState>,
}

impl From<&PluginRegistry> for PluginsManager {
	fn from(registry: &PluginRegistry) -> Self {
		Self {
			plugins: HashMap::from_iter(
				registry
					.plugins
					.iter()
					.map(|(typeid, _)| (*typeid, PluginState { enabled: true })),
			),
		}
	}
}

impl PluginsManager {
	pub fn toggle(&mut self, type_id: TypeId) {
		if let Some(state) = self.plugins.get_mut(&type_id) {
			state.toggle();
		}
	}

	pub fn enable<T: Plugin + 'static>(&mut self) {
		let type_id = TypeId::of::<T>();
		if let Some(state) = self.plugins.get_mut(&type_id) {
			state.enabled = true;
		} else {
			warn!("⚠️ PluginManager: No state found for plugin {type_id:?}.");
		}
	}
}
