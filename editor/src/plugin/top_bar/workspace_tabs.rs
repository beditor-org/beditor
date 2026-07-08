use dioxus::prelude::*;

use crate::{
	event::{Events, SwitchWorkspaceEvent},
	workspace::WorkspaceRegistry,
};

#[component]
pub fn WorkspaceTabsTool() -> Element {
	let workspace_registry = use_context::<Signal<WorkspaceRegistry>>();
	let events = use_context::<Events>();

	let tabs = workspace_registry.read().tabs.clone();
	let current_id = workspace_registry.read().get_current_id().cloned();

	rsx! {
		div {
			class: "flex items-center gap-1",
			for tab_id in tabs.iter() {
				{
					let tab_id_clone = tab_id.clone();
					let workspace = workspace_registry.read().get(tab_id.clone()).cloned();
					let is_current = current_id.as_ref() == Some(tab_id);

					if let Some(ws) = workspace {
						rsx! {
							div {
								key: "{tab_id.as_str()}",
								class: if is_current {
									"flex items-center gap-2 px-3 py-1 bg-primary rounded cursor-pointer hover:bg-gray-600"
								} else {
									"flex items-center gap-2 px-3 py-1 bg-secondary rounded cursor-pointer hover:bg-gray-700"
								},
								onclick: {
									let tab_id = tab_id.clone();
									let events = events.clone();
									move |_| {
										events.publish(SwitchWorkspaceEvent(tab_id.clone()));
									}
								},
								span {
									class: "text-xs text-gray-200",
									"{ws.name}"
								}
								button {
									class: "text-gray-400 hover:text-white text-sm",
									onclick: {
										let tab_id = tab_id_clone.clone();
										let mut workspace_registry = workspace_registry.clone();
										move |e| {
											e.stop_propagation();
											workspace_registry.write().close_tab(&tab_id);
										}
									},
									"×"
								}
							}
						}
					} else {
						rsx! { div { key: "{tab_id.as_str()}" } }
					}
				}
			}
		}
	}
}
