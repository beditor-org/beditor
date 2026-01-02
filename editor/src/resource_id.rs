use serde::{Deserialize, Serialize};

/// Unique identifier for plugin resources (panels, workspaces, tools).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceType {
	Panel,
	Workspace,
	Tool,
}

impl ResourceId {
	/// Create an ID for a panel
	pub fn panel(plugin: &str, name: &str) -> Self {
		Self(format!("{}::panel::{}", plugin, name))
	}

	/// Create an ID for a workspace
	pub fn workspace(plugin: &str, name: &str) -> Self {
		Self(format!("{}::workspace::{}", plugin, name))
	}

	/// Create an ID for a tool
	pub fn tool(plugin: &str, name: &str) -> Self {
		Self(format!("{}::tool::{}", plugin, name))
	}

	/// Get the string representation of the ID
	pub fn as_str(&self) -> &str {
		&self.0
	}

	/// Parse the ID into components (resource type, plugin, name)
	pub fn parse(&self) -> Option<(ResourceType, &str, &str)> {
		let parts: Vec<&str> = self.0.split("::").collect();
		if parts.len() != 3 {
			return None;
		}

		let resource_type = match parts[1] {
			"panel" => ResourceType::Panel,
			"workspace" => ResourceType::Workspace,
			"tool" => ResourceType::Tool,
			_ => return None,
		};

		Some((resource_type, parts[0], parts[2]))
	}

	/// Get the plugin name
	pub fn plugin(&self) -> Option<&str> {
		self.0.split("::").next()
	}

	/// Get the resource type
	pub fn resource_type(&self) -> Option<ResourceType> {
		self.parse().map(|(t, _, _)| t)
	}

	/// Get the resource name
	pub fn name(&self) -> Option<&str> {
		self.parse().map(|(_, _, n)| n)
	}
}

impl From<&str> for ResourceId {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}

impl From<String> for ResourceId {
	fn from(s: String) -> Self {
		Self(s)
	}
}

impl std::fmt::Display for ResourceId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_panel_id() {
		let id = ResourceId::panel("core", "top_bar");
		assert_eq!(id.as_str(), "core::panel::top_bar");
		assert_eq!(id.plugin(), Some("core"));
		assert_eq!(id.name(), Some("top_bar"));
		assert_eq!(id.resource_type(), Some(ResourceType::Panel));
	}

	#[test]
	fn test_workspace_id() {
		let id = ResourceId::workspace("viewport", "editor");
		assert_eq!(id.as_str(), "viewport::workspace::editor");
		assert_eq!(id.plugin(), Some("viewport"));
		assert_eq!(id.name(), Some("editor"));
		assert_eq!(id.resource_type(), Some(ResourceType::Workspace));
	}

	#[test]
	fn test_tool_id() {
		let id = ResourceId::tool("core", "logo");
		assert_eq!(id.as_str(), "core::tool::logo");
		assert_eq!(id.resource_type(), Some(ResourceType::Tool));
	}

	#[test]
	fn test_parse() {
		let id = ResourceId::panel("core", "status_bar");
		let (typ, plugin, name) = id.parse().unwrap();
		assert_eq!(typ, ResourceType::Panel);
		assert_eq!(plugin, "core");
		assert_eq!(name, "status_bar");
	}

	#[test]
	fn test_from_string() {
		let id: ResourceId = "custom::panel::test".into();
		assert_eq!(id.plugin(), Some("custom"));
	}

	#[test]
	fn test_display() {
		let id = ResourceId::panel("core", "top_bar");
		assert_eq!(format!("{}", id), "core::panel::top_bar");
	}
}
