use dioxus::prelude::*;

use crate::{
	components::{panel::TabbedPanel, LayoutArea},
	panel::PanelsManager,
	plugin::PluginRegistry,
	workspace::WorkspaceRegistry,
	PanelSocket,
};
#[component]
pub fn EditorLayout() -> Element {
	info!("rendering EditorLayout component");
	let plugins = use_context::<Signal<PluginRegistry>>();

	// Provide panels signal to children
	let workspaces_registry = use_context::<Signal<WorkspaceRegistry>>();
	let mut panels_manager = use_context_provider(|| Signal::new(PanelsManager::default()));

	// Rebuild panels when workspace changes
	use_effect(move || {
		let registry = workspaces_registry.read();
		let current_workspace = registry.get_current().expect("No current workspace found").clone();

		let mut panels = PanelsManager::from_plugins(&plugins.read());
		panels.make_active_for_workspace(&current_workspace);
		info!("✓ PanelsManager rebuilt with {:?} panels", panels.panels.len());
		panels_manager.set(panels);
	});

	let mut top_panels = vec![];
	let mut bottom_panels = vec![];
	let mut left_panels = vec![];
	let mut right_panels = vec![];
	let mut center_top_panels = vec![];
	let mut center_bottom_panels = vec![];
	let mut center_panel = None;
	panels_manager
		.read()
		.panels
		.clone()
		.into_iter()
		.filter(|(_, pannel)| pannel.is_visible && pannel.is_active)
		.for_each(|(_, pannel)| match pannel.socket {
			PanelSocket::Left => left_panels.push(pannel),
			PanelSocket::Right => right_panels.push(pannel),
			PanelSocket::Top => top_panels.push(pannel),
			PanelSocket::Bottom => bottom_panels.push(pannel),
			PanelSocket::CenterTop => center_top_panels.push(pannel),
			PanelSocket::CenterBottom => center_bottom_panels.push(pannel),
			PanelSocket::Center => center_panel = Some(pannel),
		});
	rsx! {
		div {
			class: "flex flex-col h-screen overflow-hidden gap-1 p-1 bg-primary",
			LayoutArea { panels: top_panels }
			div{
				class: "flex flex-row flex-1 gap-1 overflow-hidden",
				LayoutArea { panels: left_panels }
				div {
					class: "flex flex-col flex-1 gap-1 overflow-hidden",
					LayoutArea { panels: center_top_panels }
					if let Some(panel) = center_panel {
						TabbedPanel { key: "{panel.name}", panel }
					}
					LayoutArea { panels: center_bottom_panels }
				}
				LayoutArea { panels: right_panels }
			}
			LayoutArea { panels: bottom_panels }
		}
	}
}
