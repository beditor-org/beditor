use dioxus::prelude::*;

use crate::{PanelConfig, PanelDisplayMode};

#[component]
pub fn Panel(panel: PanelConfig) -> Element {
	match panel.display_mode {
		PanelDisplayMode::Tabbed => rsx!(TabbedPanel { panel: panel.clone() }),
		PanelDisplayMode::Stacked => rsx!(StackedPanel { panel: panel.clone() }),
	}
}

#[component]
pub fn StackedPanel(panel: PanelConfig) -> Element {
	use crate::ToolAlignment;
	let workspace_registry = use_context::<Signal<crate::workspace::WorkspaceRegistry>>();
	let current_workspace_name = workspace_registry
		.read()
		.get_current()
		.expect("No current workspace")
		.name
		.clone();

	// Split tools by alignment (but render ALL of them)
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
			div {
				class: if start_tools.is_empty() { "hidden" } else { "flex flex-row gap-2" },
				for (idx, tool) in start_tools.iter().enumerate() {
					// Check visibility INSIDE render, but always render the component
					{
						let is_visible = tool.workspaces.is_empty() || tool.workspaces.iter().any(|ws| ws.name() == current_workspace_name);
						rsx! {
							div {
								key: "{idx}",
								class: if is_visible { "" } else { "hidden" },
								{(tool.component)()}
							}
						}
					}
				}
			}
			// Center-aligned tools
			div {
				class: if center_tools.is_empty() { "hidden" } else { "flex flex-row gap-2 mx-auto" },
				for (idx, tool) in center_tools.iter().enumerate() {
					{
						let is_visible = tool.workspaces.is_empty() || tool.workspaces.iter().any(|ws| ws.name() == current_workspace_name);
						rsx! {
							div {
								key: "{idx}",
								class: if is_visible { "" } else { "hidden" },
								{(tool.component)()}
							}
						}
					}
				}
			}
			// End-aligned tools
			div {
				class: if end_tools.is_empty() { "hidden" } else { "flex flex-row gap-2 ml-auto" },
				for (idx, tool) in end_tools.iter().enumerate() {
					{
						let is_visible = tool.workspaces.is_empty() || tool.workspaces.iter().any(|ws| ws.name() == current_workspace_name);
						rsx! {
							div {
								key: "{idx}",
								class: if is_visible { "" } else { "hidden" },
								{(tool.component)()}
							}
						}
					}
				}
			}
		}
	}
}

#[component]
pub fn TabbedPanel(panel: PanelConfig) -> Element {
	let tools = panel.tools.clone();
	rsx!(for (idx, tool) in tools.iter().enumerate() {
		div {
			key: "{idx}",
			class: "flex flex-col flex-1 min-h-0 panel overflow-hidden",
			{(tool.component)()}
		}
	})
}
