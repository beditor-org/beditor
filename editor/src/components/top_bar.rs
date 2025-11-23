use dioxus::{core::Element, prelude::*};

#[component]
pub fn TopBar() -> Element {
	rsx! {
		div {
			class: "top-bar",
			button { class: "toolbar-button", "File" }
			button { class: "toolbar-button", "Edit" }
			button { class: "toolbar-button", "View" }
			button { class: "toolbar-button", "Tools" }
			button { class: "toolbar-button", "Help" }
		}
	}
}
