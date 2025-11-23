use dioxus::prelude::*;

use crate::layout::{LayoutConfig, PanelAlignment, PanelConfig};

#[component]
pub fn EditorLayout(layout: LayoutConfig) -> Element {
	let top_panels = layout.panels_by_alignment(PanelAlignment::Top);
	let bottom_panels = layout.panels_by_alignment(PanelAlignment::Bottom);
	let left_panels = layout.panels_by_alignment(PanelAlignment::Left);
	let right_panels = layout.panels_by_alignment(PanelAlignment::Right);
	let center_panels = layout.panels_by_alignment(PanelAlignment::Center);

	let left_width = left_panels
		.first()
		.map(|p| p.size_style())
		.unwrap_or_else(|| "300px".to_string());
	let right_width = right_panels
		.first()
		.map(|p| p.size_style())
		.unwrap_or_else(|| "350px".to_string());

	rsx! {
		div {
			class: "flex flex-col h-screen overflow-hidden bg-gray-800",

			// Top panels (стек зверху вниз)
			for panel in &top_panels {
				PanelContainer { config: (*panel).clone() }
			}

			// Middle row: left + center + right
			div {
				class: "flex flex-row flex-1 min-h-0",

				// Left panels container
				if !left_panels.is_empty() {
					div {
						class: "flex flex-col border-r border-gray-600",
						style: "width: {left_width}",

						for panel in &left_panels {
							PanelContainer { config: (*panel).clone() }
						}
					}
				}

				// Center panels (займає всю доступну площу)
				div {
					class: "flex flex-col flex-1 min-h-0",

					for panel in &center_panels {
						PanelContainer { config: (*panel).clone() }
					}
				}

				// Right panels container
				if !right_panels.is_empty() {
					div {
						class: "flex flex-col border-l border-gray-600",
						style: "width: {right_width}",

						for panel in &right_panels {
							PanelContainer { config: (*panel).clone() }
						}
					}
				}
			}

			// Bottom panels (стек знизу вгору - reverse order)
			for panel in bottom_panels.iter().rev() {
				PanelContainer { config: (*panel).clone() }
			}
		}
	}
}

#[component]
fn PanelContainer(config: PanelConfig) -> Element {
	let height_style = match config.alignment {
		PanelAlignment::Top | PanelAlignment::Bottom => {
			format!("height: {}", config.size_style())
		}
		PanelAlignment::Left | PanelAlignment::Right => {
			// Height auto для вертикальних панелей
			"flex: 1 1 0%".to_string()
		}
		PanelAlignment::Center => "flex: 1 1 0%".to_string(),
	};

	rsx! {
		div {
			class: "flex flex-col bg-gray-900 border-b border-gray-600",
			style: "{height_style}",
			id: "{config.id}",

			// Header
			div {
				class: "flex items-center justify-between px-4 py-2 bg-gray-700 border-b border-gray-600",

				span { class: "text-gray-300", "{config.id}" }

				// Close button (optional)
				button {
					class: "text-gray-300 hover:text-white",
					"×"
				}
			}

			// Content area
			div {
				class: "flex-1 overflow-auto p-4",

				"Panel content: {config.id}"
			}
		}
	}
}
