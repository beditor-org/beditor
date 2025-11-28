use dioxus::prelude::*;

#[component]
pub fn Logo() -> dioxus::prelude::Element {
	rsx! {
			img {
				class: "h-full w-auto object-contain",
				src: asset!("/assets/bevy-logo.png"),
			}
	}
}
