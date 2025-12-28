use dioxus::{core::Element, prelude::*};

use crate::{
	event::{Events, OpenGameEvent},
	panel::PanelsManager,
	plugin::PluginRegistry,
};

#[component]
pub fn MenuBar() -> Element {
	let plugins_registry = use_context::<Signal<PluginRegistry>>();
	let mut panels_manager = use_context::<Signal<PanelsManager>>();
	let events = use_context::<Events>();
	let plugins = plugins_registry.read().plugins.clone();
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
					onclick: move |_| {
						events.publish(OpenGameEvent("./target/release/examples/demo".to_string()));
					},
					"Open"
				}
			}

			// Panels menu
			MenuDropdown { label: "Plugins",
				{

					plugins.into_iter().map(|(plugin_name, plugin)| {
						rsx! {
							div {
								key: "{plugin_name:?}",
								class: "px-3 py-1 bg-red-100 cursor-pointer flex items-center justify-between",
								onclick: move |_| {
									// plugins_registry.write().toggle(plugin_name);
									info!("Toggling plugin: {}", plugin_name);
								},
								span { "{plugin_name}" }
								if plugin.is_enabled {
									span { class: "text-green-500 ml-2", "✓" }
								}
							}
						}
					})
				}
			}

			MenuDropdown { label: "Panels",
				{
					panels_manager.read().panels.iter().enumerate().map(|(idx, panel)| {
						let name = panel.name.clone();
						let is_visible = panel.is_visible;
						rsx! {
							div {
								key: "{idx}",
								class: "px-3 py-1 bg-red-100 cursor-pointer flex items-center justify-between",
								onclick: move |_| {
									info!("Toggling panel: {}", name);
									let current = panels_manager.read().panels[idx].is_visible;
									panels_manager.write().panels[idx].is_visible = !current;
								},
								span { "{name}" }
								if is_visible {
									span { class: "text-green-500 ml-2", "✓" }
								}
							}
						}
					})
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
