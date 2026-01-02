use dioxus::prelude::*;

use crate::EditorConfig;

pub fn recent_projects() -> Element {
	let config = use_context::<EditorConfig>();
	rsx! {
		h2{
			class: "text-2xl font-semibold mb-2",
			"Recent Projects"
		}
		ul {
			for project in config.recent_projects.iter() {
				li {
					a {
						class: "text-blue-500 hover:text-blue-600 hover:underline cursor-pointer",
						"{project.name}"
					}
					" - {project.path}"
				}
			}
		}
	}
}
