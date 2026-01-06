use dioxus::core::Element;

use crate::{PanelConfig, PanelSocket, ResourceId};

#[derive(Clone, Debug, PartialEq)]
pub enum ToolPlacement {
	ByResourceId(ResourceId),      // Place in specific panel by resource id
	PanelByAlignment(PanelSocket), // Place in any panel with this alignment
	OwnPanel(PanelConfig),         // Create dedicated panel for this tool
	                               // NoUI,                            // No UI component (background service)
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ToolAlignment {
	#[default]
	Start,
	Center,
	End,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
	pub placement: ToolPlacement,
	pub name: String,
	pub component: fn() -> Element,
	/// Only work in Stacked panels
	pub alignment: ToolAlignment,
	/// Workspaces where this tool should be visible
	pub workspaces: Vec<ResourceId>,
}
