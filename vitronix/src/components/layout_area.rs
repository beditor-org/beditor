use dioxus::prelude::*;

use crate::{components::Panel, PanelConfig};

#[component]
pub fn LayoutArea(panels: Vec<PanelConfig>, #[props(default)] class: String) -> Element {
	rsx! {
		if !panels.is_empty() {
			div {
				class: "flex flex-col gap-1 {class}",
				for panel in panels {
					Panel { panel: panel.clone() }
				}
			}
		}
	}
}
