use dioxus::{core::Element, prelude::*};

#[component]
fn PropertyGroup(title: String) -> Element {
	rsx! {
		div {
			class: "property-group",
			style: "margin: 10px 0; padding: 5px 0; border-bottom: 1px solid #3c3c3c;",
			span {
				style: "color: #aaa; font-weight: bold; font-size: 12px;",
				"{title}"
			}
		}
	}
}
