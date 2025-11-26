use dioxus::core::Element;

use crate::{PanelAligment, PanelConfig};

#[derive(Clone, Debug, PartialEq)]
pub enum ToolPlacement {
	PanelByName(String),             // Place in specific panel by name
	PanelByAlignment(PanelAligment), // Place in any panel with this alignment
	OwnPanel(PanelConfig),           // Create dedicated panel for this tool
	                                 // NoUI,                            // No UI component (background service)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
	pub placement: ToolPlacement,
	pub name: String,
	pub component: fn() -> Element,
}
