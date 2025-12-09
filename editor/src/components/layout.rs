use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
	components::{LayoutArea, Viewport},
	panel::PanelsManager,
	PanelAligment, PluginRegistry, PluginsManager,
};

#[component]
pub fn EditorLayout() -> Element {
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	let pm = use_context::<Signal<PluginsManager>>();

	// Provide panels signal to children
	let mut panels_manager = use_context_provider(|| Signal::new(PanelsManager::default()));

	// Auto-rebuild panels when plugin states change
	use_effect(move || {
		let panels = PanelsManager::from_plugins(&plugins_registry, &pm.read());
		println!("✓ PanelsManager rebuilt with {:?} panels", panels.panels.len());
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
		.filter(|pannel| pannel.is_visible)
		.for_each(|pannel| match pannel.alignment {
			PanelAligment::Top => top_panels.push(pannel),
			PanelAligment::Bottom => bottom_panels.push(pannel),
			PanelAligment::Left => left_panels.push(pannel),
			PanelAligment::Right => right_panels.push(pannel),
			PanelAligment::CenterTop => center_top_panels.push(pannel),
			PanelAligment::CenterBottom => center_bottom_panels.push(pannel),
			PanelAligment::Center => center_panel = Some(pannel),
		});
	rsx! {
		div {
			class: "flex flex-col h-screen overflow-hidden gap-1 p-1 bg-primary",
			LayoutArea { panels: top_panels }
			div{
				class: "flex flex-row grow-1 gap-1",
				LayoutArea { panels: left_panels }
				div {
					class: "flex flex-col grow-1 gap-1",
					LayoutArea { panels: center_top_panels }
					Viewport {}
					LayoutArea { panels: center_bottom_panels }
				}
				LayoutArea { panels: right_panels }
			}
			LayoutArea { panels: bottom_panels }
		}
	}
}
