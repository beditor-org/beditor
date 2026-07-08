use dioxus::prelude::*;

use crate::{
	components::{
		menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
		ThemeToggle,
	},
	event::{EditorEvent, Events},
	main_menu::MenuBarGroupConfig,
	panel::PanelsManager,
	plugin::{
		top_bar::{logo::Logo, window_controls::WindowControls},
		PluginRegistry,
	},
	project::open_project_dialog,
};

#[component]
pub fn MenuBar() -> Element {
	let menu_bar_groups = use_context::<Memo<Vec<MenuBarGroupConfig>>>();

	let plugins_registry = use_context::<Signal<PluginRegistry>>();
	let mut panels_manager = use_context::<Signal<PanelsManager>>();
	let events = use_context::<Events>();
	let plugins = plugins_registry.read().plugins.clone();
	rsx! {
		div { class: "flex flex-row h-8",
			ThemeToggle {}
			Menubar {
				{
					menu_bar_groups.iter().enumerate().map(|(group_index, group)| {
						rsx! {
							MenubarMenu {
								index: {group_index as usize},
								MenubarTrigger { "{group.label}" }
								MenubarContent {
									{
										group.items.iter().enumerate().map(|(item_index, item)| {
											rsx! {
												MenubarItem {
													index: {item_index as usize},
													value: "{item.label}".to_string(),
													disabled: {item.disabled},
													on_select: move |value| {
														tracing::info!("Selected value: {}", value);
													},
													"{item.label}"
												}
											}
										})
									}
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
