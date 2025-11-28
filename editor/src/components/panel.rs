use dioxus::prelude::*;

use crate::{PanelDisplayMode, PanelState};

#[component]
pub fn Panel(panel: PanelState) -> Element {
	match panel.display_mode {
		PanelDisplayMode::Tabbed => rsx!(TabbedPanel { panel: panel.clone() }),
		PanelDisplayMode::Stacked => rsx!(StackedPanel { panel: panel.clone() }),
	}
}

#[component]
pub fn StackedPanel(panel: PanelState) -> Element {
	rsx! {
		div{
			class: "grow panel",
			for tool in panel.tools.iter() {
				 {(tool.component)()}

			}
		}
	}
}

#[component]
pub fn TabbedPanel(panel: PanelState) -> Element {
	rsx!(for tool in panel.tools.iter() {
		div{
			class: "grow panel",
			"tabbed" {(tool.component)()}
		}
	})
}
