use dioxus::prelude::*;

use crate::components::PanelState;

#[component]
pub fn LayoutArea(panels: Vec<PanelState>) -> Element {
	rsx! {
		for _panel in panels {
			div {
				class: "grow bg-red-100",
				for _tool in _panel.tools.iter() {
					{(_tool.component)()}
				}
				"panel content"
			}
		}
	}
}
