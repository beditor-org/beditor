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
	use crate::ToolAlignment;

	let (start_tools, center_tools, end_tools): (Vec<_>, Vec<_>, Vec<_>) =
		panel
			.tools
			.iter()
			.fold((vec![], vec![], vec![]), |(mut start, mut center, mut end), tool| {
				match tool.alignment {
					ToolAlignment::Start => start.push(tool),
					ToolAlignment::Center => center.push(tool),
					ToolAlignment::End => end.push(tool),
				}
				(start, center, end)
			});

	rsx! {
		div {
			class: "grow panel flex flex-row",
			// Start-aligned tools
			if !start_tools.is_empty() {
				div {
					class: "flex flex-row gap-2",
					for tool in start_tools {
						{(tool.component)()}
					}
				}
			}
			// Center-aligned tools
			if !center_tools.is_empty() {
				div {
					class: "flex flex-row gap-2 mx-auto",
					for tool in center_tools {
						{(tool.component)()}
					}
				}
			}
			// End-aligned tools
			if !end_tools.is_empty() {
				div {
					class: "flex flex-row gap-2 ml-auto",
					for tool in end_tools {
						{(tool.component)()}
					}
				}
			}
		}
	}
}

#[component]
pub fn TabbedPanel(panel: PanelState) -> Element {
	rsx!(for tool in panel.tools.iter() {
		div{
			class: "grow panel",
			{(tool.component)()}
		}
	})
}
