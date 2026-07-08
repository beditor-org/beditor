use dioxus::{core::Element, desktop::tao::window::Theme, prelude::*};

use crate::{
	components::{
		menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
		ThemeToggle,
	},
	event::{EditorEvent, Events},
	panel::PanelsManager,
	plugin::{
		top_bar::{logo::Logo, window_controls::WindowControls},
		PluginRegistry,
	},
	project::open_project_dialog,
};

#[component]
pub fn MenuBar() -> Element {
	let plugins_registry = use_context::<Signal<PluginRegistry>>();
	let mut panels_manager = use_context::<Signal<PanelsManager>>();
	let events = use_context::<Events>();
	let plugins = plugins_registry.read().plugins.clone();
	rsx! {
		div { class: "flex flex-row h-8",
			ThemeToggle {}
			Menubar {
				MenubarMenu {
					index: 0usize,
					MenubarTrigger { "File" }
					MenubarContent {
						MenubarItem {
							index: 0usize,
							value: "new".to_string(),
							on_select: move |value| {
								tracing::info!("Selected value: {}", value);
							},
							"New"
						}
						MenubarItem {
							index: 1usize,
							value: "open".to_string(),
							disabled: true,
							on_select: move |value| {
								tracing::info!("Selected value: {}", value);
							},
							"Open"
						}
						MenubarItem {
							index: 2usize,
							value: "save".to_string(),
							on_select: move |value| {
								tracing::info!("Selected value: {}", value);
							},
							"Save"
						}
					}
				}
				MenubarMenu { index: 1usize, MenubarTrigger { "Plugins" }}
				MenubarMenu { index: 2usize, MenubarTrigger { "Panels" }}
				MenubarMenu { index: 3usize, MenubarTrigger { "Themes" }}
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
