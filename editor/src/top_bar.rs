use dioxus::{core::Element, prelude::*};

#[component]
pub fn TopBar() -> Element {
	rsx! {
		style { {include_str!("../assets/editor.css")} }
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
