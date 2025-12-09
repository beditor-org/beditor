use dioxus::prelude::spawn;
use std::sync::Arc;

use dioxus::{core::Element, prelude::*};

use crate::{panel::PanelsManager, GameProcessManager, PluginRegistry, PluginsManager, ViewportProtocolState};

#[component]
pub fn MenuBar() -> Element {
	eprintln!("📋 MenuBar component rendering");
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	let mut panels_manager = use_context::<Signal<PanelsManager>>();
	let mut plugins_manager = use_context::<Signal<PluginsManager>>();
	let mut state = use_context::<Signal<ViewportProtocolState>>();
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
					onclick: move |_| {
						eprintln!("🔴 'Open' button clicked!");

					let mut manager = GameProcessManager::new();
					match manager.start("./target/release/examples/demo") {
						Ok(rx) => {
							eprintln!("✅ Game started, spawning frame listener");

							// Bridge sync channel to async: blocking recv in thread -> async channel
							let (tx, mut async_rx) = tokio::sync::mpsc::unbounded_channel();
							std::thread::spawn(move || {
								while let Ok(frame_bytes) = rx.recv() {
									if tx.send(frame_bytes).is_err() {
										break; // UI dropped receiver
									}
								}
							});

							// Non-blocking async loop
							spawn(async move {
								while let Some(frame_bytes) = async_rx.recv().await {
									state.write().update_frame(frame_bytes);
								}
							});
						}
							Err(e) => eprintln!("❌ Failed to start game: {}", e),
						}
					},
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
