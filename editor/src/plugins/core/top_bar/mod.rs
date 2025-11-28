use dioxus::prelude::*;

pub mod logo;
pub mod main_menu;
pub mod window_controls;

#[component]
pub fn TopBar() -> dioxus::prelude::Element {
	dioxus::prelude::rsx! {
		div {
			class: "flex flex-row overflow-hidden items-center h-8",
			logo::Logo {}
			main_menu::MenuBar {}
			window_controls::WindowControls {}
		}
	}
}
