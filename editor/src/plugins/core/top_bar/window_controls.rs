use dioxus::prelude::*;
use lucide_dioxus::{Minus, Square, X};

#[component]
pub fn WindowControls() -> dioxus::prelude::Element {
	rsx! {
			div {
				class: "flex flex-row gap-2 ml-auto",
				Minus{
					class:"translate-y-[8px]"
				}
				Square{}
				div {
					onclick: move |_| {
						dioxus::desktop::window().close();
					},
					X{}
				}

			}
	}
}
