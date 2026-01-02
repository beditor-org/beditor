use dioxus::prelude::*;

use crate::{editor::open_project_dialog, event::Events};
#[component]
pub fn welcome() -> Element {
	let events = use_context::<Events>();

	rsx! {
		 h1 { "Welcome to Beditor!" }
		 p {
			 a { "create new project"}

	  }
			p {

				a {
					onclick: move |_| open_project_dialog(events.clone()),
					"load project"
			}
		}
	}
}
