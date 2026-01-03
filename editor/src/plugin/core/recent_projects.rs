use dioxus::prelude::*;

use crate::{
	event::{Events, OpenGameEvent},
	EditorConfig,
};

pub fn recent_projects() -> Element {
	let config = use_context::<Signal<EditorConfig>>();
	let events = use_context::<Events>();
	let recent_projects = config.read().recent_projects.clone();

	rsx! {
		h2{
			class: "text-2xl font-semibold mb-2",
			"Recent Projects"
		}
		ul {
			for project in recent_projects.iter() {
				li {
					key: "{project.path}",
					{
						let project = project.clone();
						let events = events.clone();
						rsx! {
							a {
								class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
								onclick: move |_| {
									events.publish(OpenGameEvent(project.path.clone()));
								},
								"{project.name}"
							}
							" - {project.path}"
						}
					}
				}
			}
		}
	}
}
