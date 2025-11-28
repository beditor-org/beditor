use dioxus::prelude::*;

use crate::{components::Panel, PanelState};

#[component]
pub fn LayoutArea(panels: Vec<PanelState>) -> Element {
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
