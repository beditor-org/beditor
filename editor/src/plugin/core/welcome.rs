use dioxus::prelude::*;

use crate::{event::Events, plugin::core::recent_projects::recent_projects, project::open_project_dialog, EditorConfig};
#[component]
pub fn welcome() -> Element {
	let events = use_context::<Events>();
	let config = use_context::<EditorConfig>();

	rsx! {
		div {
			class: "flex flex-row h-full gap-4",
			div {
				class: "flex flex-1 flex-col",
				div {
					class: "flex-1",
					h2 {
						class: "text-2xl font-semibold mb-2",
						"Start"
					}
					ul {
						li {
							a {
								class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
								"New project"
							}
						}
						li {
							a {
								class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
								onclick: move |_| open_project_dialog(events.clone()),
								"Open project"
							}
						}
					}
				}
				div {
					class: "flex-1",
					recent_projects {}
				}
			}
			div {
				class: "flex flex-1 flex-col",
				h1 {
					class: "text-4xl font-bold mb-4",
					"Welcome to {config.window.title}"
				}
				h2 {
					class: "text-2xl font-semibold mb-2",
					"What's new"
				}
				ul {
					class: "list-disc list-inside",
					li { "Basic workspaces" }
					li { "Plugin system" }
					li { "Stdio transport" }
					li { "Transport multiplexing" }
					li { "Streaming viewport" }
				}
			}
		}
	}
}
