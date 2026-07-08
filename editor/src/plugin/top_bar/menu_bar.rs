use dioxus::prelude::*;

use crate::{
	components::{
		menubar::{Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger},
		ThemeToggle,
	},
	event::Events,
	main_menu::MenuBarGroupConfig,
};

#[component]
pub fn MenuBar() -> Element {
	let menu_bar_groups = use_context::<Memo<Vec<MenuBarGroupConfig>>>();

	let events = use_context::<Events>();
	let groups = menu_bar_groups.read().clone();
	rsx! {
		div { class: "flex flex-row h-8",
			ThemeToggle {}
			Menubar {
				{
					groups.into_iter().enumerate().map(|(group_index, group)| {
						let group_label = group.label;
						let items = group.items;
						rsx! {
							MenubarMenu {
								index: {group_index as usize},
								MenubarTrigger { "{group_label}" }
								MenubarContent {
									{
										items.into_iter().enumerate().map(|(item_index, item)| {
											let action = item.action;
											let item_label = item.label;
											let disabled = item.disabled;
											let events = events.clone();
											rsx! {
												MenubarItem {
													index: {item_index as usize},
													value: item_label.to_string(),
													disabled: disabled,
													on_select: move |_| {
														if let Some(action) = action {
															action(&events);
														}
													},
													"{item_label}"
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
