use dioxus::prelude::*;

use crate::components::{LayoutArea, PanelState};

#[component]
pub fn EditorLayout(panels: Vec<PanelState>) -> Element {
	let mut top_panels = vec![];
	let mut bottom_panels = vec![];
	let mut left_panels = vec![];
	let mut right_panels = vec![];
	let mut center_top_panels = vec![];
	let mut center_bottom_panels = vec![];
	let mut center_panel = None;
	panels
		.into_iter()
		.filter(|pannel| pannel.is_visible)
		.for_each(|pannel| match pannel.alignment {
			crate::components::PanelAligment::Top => top_panels.push(pannel),
			crate::components::PanelAligment::Bottom => bottom_panels.push(pannel),
			crate::components::PanelAligment::Left => left_panels.push(pannel),
			crate::components::PanelAligment::Right => right_panels.push(pannel),
			crate::components::PanelAligment::CenterTop => center_top_panels.push(pannel),
			crate::components::PanelAligment::CenterBottom => center_bottom_panels.push(pannel),
			crate::components::PanelAligment::Center => center_panel = Some(pannel),
		});
	rsx! {
		div {
			class: "flex flex-col h-screen overflow-hidden gap-3",
			LayoutArea {
				panels: top_panels.clone(),
			}
			div{
				class: "flex flex-row grow-1 gap-3",
				LayoutArea {
					panels: left_panels.clone(),
				}
				div {
					class: "flex flex-col grow-1 gap-3",
					LayoutArea {
						panels: center_top_panels.clone(),
					}
					div{
						class: "grow-1 bg-red-100",
						"center pannel"
					}
					LayoutArea {
						panels: center_bottom_panels.clone(),
					}
				}
				LayoutArea {
					panels: right_panels.clone(),
				}
			}
			LayoutArea {
				panels: bottom_panels.clone(),
			}
		}
	}
}
