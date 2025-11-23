use dioxus::core::Element;

use crate::components::PanelState;

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
	pub require_stand_alone_panel: Option<PanelState>, // otherwise should be added manually to existing panel
	pub name: String,
	pub component: fn() -> Element,
	pub panel_group: Option<String>,
}
