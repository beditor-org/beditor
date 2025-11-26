use std::sync::Arc;

use dioxus::{html::g::to, prelude::*};

use crate::{components::LayoutArea, panel::PanelsManager, tool, PanelAligment, PanelState, PluginRegistry, PluginsManager};

#[component]
pub fn EditorLayout() -> Element {
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	let pm = use_context::<Signal<PluginsManager>>();

	let mut panels_manager: Signal<PanelsManager> = use_context_provider(|| Signal::new(PanelsManager::default()));

	// Rebuild panels when plugin states change
	use_effect(move || {
		pm.read(); // Subscribe to changes

		let mut new_panels = PanelsManager::default();
		pm.read()
			.plugins
			.iter()
			.filter(|(_, plugin_state)| plugin_state.enabled)
			.for_each(|(typeid, plugin_state)| {
				let plugin = plugins_registry.plugins.get(typeid).unwrap();
				// Register panels
				for panel_cfg in plugin.get_panels() {
					let panel_state = PanelState {
						name: panel_cfg.name,
						alignment: panel_cfg.alignment,
						..Default::default()
					};
					new_panels.add_panel(panel_state);
				}
				for tool in plugin.get_tools() {
					match tool.placement {
						tool::ToolPlacement::PanelByName(ref panel_name) => {
							if let Some(panel) = new_panels.get_panel_by_name(&panel_name) {
								panel.tools.push(tool);
							}
						}
						tool::ToolPlacement::PanelByAlignment(alignment) => {
							todo!()
						}
						tool::ToolPlacement::OwnPanel(panel_config) => todo!(),
					}
				}
			});
		println!("✓ PanelsManager rebuilt with {:?} panels", new_panels.panels.len());
		panels_manager.set(new_panels);
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
