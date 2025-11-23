use dioxus::prelude::*;

use crate::tool::Tool;

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct PanelState {
	pub alignment: PanelAligment,
	pub tools: Vec<Tool>,
	// pub is_open: bool,
	// pub title: String,
	// Additional fields can be added as needed
}
#[component]
pub fn Panel() -> Element {
	rsx! {
		div {
			// Panel content goes here
		}
	}
}
