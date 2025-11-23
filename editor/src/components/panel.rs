use dioxus::prelude::*;

#[derive(Clone, Debug)]
pub enum PanelAligment {
	Left,
	Right,
	Top,
	Bottom,
	Center,
	CenterTop,
	CenterBottom,
}
#[derive(Clone, Debug)]
pub struct PanelState {
	pub alignment: PanelAligment,
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
