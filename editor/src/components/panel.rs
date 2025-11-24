use dioxus::prelude::*;

use crate::tool::Tool;

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
pub struct PanelState {
	pub alignment: PanelAligment,
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
			tools: Vec::new(),
			name: String::new(),
			is_visible: true,
		}
	}
}
#[component]
pub fn Panel() -> Element {
	rsx! {
		div {
			// Panel content goes here
		}
	}
}
