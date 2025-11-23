use dioxus::prelude::*;

use crate::components::PanelState;

#[component]
pub fn LayoutArea(panels: Vec<PanelState>) -> Element {
	rsx! {
		if !panels.is_empty() {
			div {
				class: "flex flex-col gap-3",
				for _panel in panels {
					div {
						class: "grow bg-red-100",
						for _tool in _panel.tools.iter() {
							{(_tool.component)()}
						}
					}
				}
			}
		}
	}
}
