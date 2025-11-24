use dioxus::{core::Element, prelude::*};

use crate::components::PanelState;

#[component]
fn MenuBar() -> Element {
	let mut panels = use_context::<Signal<Vec<PanelState>>>();

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
			MenuDropdown { label: "Panels",
				for (idx, panel) in panels.read().iter().enumerate() {
					{
						let name = panel.name.clone();
						let is_visible = panel.is_visible;
						rsx! {
							div {
								key: "{idx}",
								class: "px-3 py-1 hover:bg-gray-700 cursor-pointer flex items-center justify-between",
								onclick: move |_| {
									let mut panels_write = panels.write();
									panels_write[idx].is_visible = !panels_write[idx].is_visible;
									println!("Panel {} visibility: {}", name, panels_write[idx].is_visible);
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
