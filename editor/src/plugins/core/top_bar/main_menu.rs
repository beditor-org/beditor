use std::sync::Arc;

use dioxus::{core::Element, prelude::*};

use crate::{panel::PanelsManager, PluginRegistry, PluginsManager};

#[component]
pub fn MenuBar() -> Element {
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	let mut panels_manager = use_context::<Signal<PanelsManager>>();
	let mut plugins_manager = use_context::<Signal<PluginsManager>>();
	println!("plugins: {}", plugins_manager.read().plugins.len());
	rsx! {
		div { class: "flex flex-row h-8",
			// File menu
			MenuDropdown { label: "File",
				div {
					class: "px-3 py-1 cursor-pointer",
					onclick: move |_| println!("New clicked"),
					"New"
				}
				div {
					class: "px-3 py-1 cursor-pointer",
					onclick: move |_| println!("Open clicked"),
					"Open"
				}
			}

			// Panels menu
			MenuDropdown { label: "Plugins",
				{
					plugins_manager.read().plugins.iter().map(|(typeid, state)| {
						let typeid = *typeid;
						let enabled = state.enabled;
						let name = plugins_registry.plugins.get(&typeid).unwrap().get_name();
						rsx! {
							div {
								key: "{typeid:?}",
								class: "px-3 py-1 bg-red-100 cursor-pointer flex items-center justify-between",
								onclick: move |_| {
									plugins_manager.write().toggle(typeid);
								},
								span { "{name}" }
								if enabled {
									span { class: "text-green-500 ml-2", "✓" }
								}
							}
						}
					})
				}
			}

			MenuDropdown { label: "Panels",
				for (idx, panel) in panels_manager.read().panels.iter().enumerate() {
					 {
					let name = panel.name.clone();
					let is_visible = panel.is_visible;
					rsx! {
							div {
							key: "{idx}",
							class: "px-3 py-1 bg-red-100 cursor-pointer flex items-center justify-between",
								onclick: move |_| {
									panels_manager.write().panels[idx].toggle();
								 },
								span { "{name}" }
								if is_visible {
									span { class: "text-green-500 ml-2", "✓" }
								}
							}
					}
					}
				}
			}
		}
	}
}

#[component]
fn MenuDropdown(label: String, children: Element) -> Element {
	let mut open = use_signal(|| false);

	rsx! {
		div { class: "relative",
			button {
				class: "px-3 py-1 hover:bg-gray-700",
				onclick: move |_| open.set(!open()),
				"{label}"
			}

			if open() {
				div {
					class: "absolute top-full left-0 bg-gray-800 border border-gray-700 shadow-lg min-w-32",
					onclick: move |_| open.set(false),
					{children}
				}
			}
		}
	}
}

#[component]
pub fn TopBar() -> Element {
	use super::{Logo, WindowControls};
	use crate::components::ThemeToggle;

	rsx! {
		div {
			class: "flex flex-row items-center h-8 overflow-hidden",
			Logo {}
			MenuBar {}
			div { class: "flex-1" }
			ThemeToggle {}
			WindowControls {}
		}
	}
}
