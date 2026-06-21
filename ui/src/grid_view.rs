use dioxus::prelude::*;

#[component]
pub fn GridView(children: Element) -> Element {
	rsx! {
		 div { class: "grid grid-cols-4 gap-4", {children} }
	}
}
