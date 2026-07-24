use dioxus::prelude::*;

use crate::{
	event::Events,
	plugin::{core::recent_projects::recent_projects, i18n_core::plugin::I18n},
	project::open_project_dialog,
	EditorConfig,
};
#[component]
pub fn welcome() -> Element {
	let events = use_context::<Events>();
	let config = use_context::<EditorConfig>();
	let i18n = use_context::<Signal<I18n>>();
	rsx! {
		div {
			class: "flex flex-row h-full gap-4",
			div {
				class: "flex flex-1 flex-col",
				div {
					class: "flex-1",
					h2 {
						class: "text-2xl font-semibold mb-2",
						{i18n.read().get("core:welcome:start")}
					}
					ul {
						li {
							a {
								class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
								{i18n.read().get("core:welcome:new_project")}
							}
						}
						li {
							a {
								class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
								onclick: move |_| open_project_dialog(events.clone()),
								{i18n.read().get("core:welcome:open_project")}
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
					{i18n.read().get("core:welcome:welcome_to")}" "{config.window.title}
				}
				h2 {
					class: "text-2xl font-semibold mb-2",
					{i18n.read().get("core:welcome:what_new")}
				}
				ul {
					class: "list-disc list-inside",
					li { "Basic workspaces" }
					li { "Plugin system" }
					li { "Stdio transport" }
					li { "Transport multiplexing" }
					li { "Streaming viewport" }
					li { "I18n support" }
				}
			}
		}
	}
}
