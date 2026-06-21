use dioxus::prelude::*;

use crate::{components::Panel, PanelConfig};

#[component]
pub fn LayoutArea(panels: Vec<PanelConfig>) -> Element {
	rsx! {
		if !panels.is_empty() {
			div {
				class: "flex flex-col gap-1",
				for panel in panels {
					Panel { panel: panel.clone() }
				}
			}
		}
	}
}
