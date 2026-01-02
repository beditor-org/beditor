use std::collections::HashMap;

use tracing::{info, warn};

use crate::{plugin::PluginRegistry, ResourceId};

#[derive(Clone, Default)]
pub struct Workspace {
	pub name: String,
	pub panels: Vec<ResourceId>,
}

#[derive(Clone, Default)]
pub struct WorkspaceRegistry {
	pub workspaces: HashMap<ResourceId, Workspace>,
	current: Option<ResourceId>,
}

impl WorkspaceRegistry {
	pub fn get(&self, id: ResourceId) -> Option<&Workspace> {
		self.workspaces.get(&id)
	}

	pub fn set_current(&mut self, workspace_id: ResourceId) {
		if let Some(_) = self.workspaces.get(&workspace_id) {
			self.current = Some(workspace_id.clone());
			info!("Current workspace set to {}", workspace_id.as_str());
		} else {
			warn!("Workspace ID {} not found in registry", workspace_id.as_str());
		}
	}

	pub fn get_current(&self) -> Option<&Workspace> {
		self.current.as_ref().and_then(|id| self.workspaces.get(id))
	}

	pub fn from_plugins(plugins: &PluginRegistry) -> Self {
		let mut registry = Self::default();
		// Collect workspaces from enabled plugins
		plugins
			.plugins
			.iter()
			.filter(|(_, state)| state.is_enabled)
			.for_each(|(_, plugin)| {
				registry.workspaces.extend(plugin.workspaces.iter().map(|ws| {
					let id = ResourceId::workspace(&plugin.name, &ws.name);
					(id, ws.clone())
				}));
			});

		// Add panels to workspaces based on panel.workspaces field
		plugins
			.plugins
			.iter()
			.filter(|(_, state)| state.is_enabled)
			.for_each(|(_, plugin)| {
				for panel in &plugin.panels {
					let panel_id = ResourceId::panel(&plugin.name, &panel.name);

					for workspace_id in &panel.workspaces {
						if let Some(workspace) = registry.workspaces.get_mut(workspace_id) {
							if !workspace.panels.contains(&panel_id) {
								workspace.panels.push(panel_id.clone());
								info!("Added panel {} to workspace {}", panel_id.as_str(), workspace_id.as_str());
							}
						} else {
							warn!(
								"Plugin '{}' panel '{}' tried to register in non-existent workspace {}",
								plugin.name,
								panel.name,
								workspace_id.as_str()
							);
						}
					}
				}
			});

		registry
	}
}
