use dioxus::prelude::*;

#[component]
pub fn Logo() -> dioxus::prelude::Element {
	rsx! {
			img {
				class: "h-8 w-auto object-contain",
				src: asset!("/assets/bevy-logo.png"),
			}
	}
}
