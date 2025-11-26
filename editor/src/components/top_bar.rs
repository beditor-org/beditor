use std::sync::{Arc, RwLock};

use bevy::app::Plugin;
use dioxus::{core::Element, prelude::*};

use crate::{panel::PanelsManager, PanelState, PluginRegistry, PluginsManager};

#[component]
fn MenuBar() -> Element {
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	let mut panels_manager = use_context::<PanelsManager>();
	let mut plugins_manager = use_context::<PluginsManager>();
	rsx! {
		div { class: "flex flex-row bg-gray-800 h-8",
			// File menu
			MenuDropdown { label: "File",
				div {
					class: "px-3 py-1 hover:bg-gray-700 cursor-pointer",
					onclick: move |_| println!("New clicked"),
					"New"
				}
				div {
					class: "px-3 py-1 hover:bg-gray-700 cursor-pointer",
					onclick: move |_| println!("Open clicked"),
					"Open"
				}
			}

			// Panels menu
			MenuDropdown { label: "Plugins",
				for (idx, (typeid, plugin)) in plugins_manager.plugins.iter().enumerate() {
					{
						let name = plugins_registry.plugins.get(typeid).unwrap().get_name();
						rsx! {
							div {
								key: "{idx}",
								class: "px-3 py-1 hover:bg-gray-700 cursor-pointer flex items-center justify-between",
								// onclick: move |_| {
								// 	let mut panels_write = ;
								// 	plugins.write()[idx].is_visible = !panels_write[idx].is_visible;
								// 	println!("Panel {} visibility: {}", name, panels_write[idx].is_visible);
								// },
								span { "{name}" }
								// if is_visible {
								// 	span { class: "text-green-500 ml-2", "✓" }
								// }
							}
						}
					}
				}
			}

			// MenuDropdown { label: "Panels",
			// 	for (idx, (typeid, panel)) in panels_manager.panels.iter().enumerate() {
			// 		{
			// 			let name = panel.name.clone();
			// 			let is_visible = panel.is_visible;
			// 			rsx! {
			// 				div {
			// 					key: "{idx}",
			// 					class: "px-3 py-1 hover:bg-gray-700 cursor-pointer flex items-center justify-between",
			// 					onclick: move |_| {
			// 						// let mut panels_write = panels_manager.write();
			// 						// panels_write[idx].is_visible = !panels_write[idx].is_visible;
			// 						// println!("Panel {} visibility: {}", name, panels_write[idx].is_visible);
			// 					},
			// 					span { "{name}" }
			// 					if is_visible {
			// 						span { class: "text-green-500 ml-2", "✓" }
			// 					}
			// 				}
			// 			}
			// 		}
			// 	}
			// }
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
	rsx! {
		div {
			class: "top-bar",
			MenuBar {}
		}
		button {
			"close"
		}
	}
}
