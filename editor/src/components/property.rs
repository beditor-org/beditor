use dioxus::{core::Element, prelude::*};

#[component]
fn Property(label: String, value: String) -> Element {
	rsx! {
		div {
			class: "property-row",
			style: "display: flex; justify-content: space-between; padding: 5px 0;",
			label {
				style: "color: #aaa; font-size: 12px;",
				"{label}:"
			}
			input {
				r#type: "number",
				value: "{value}",
				style: "width: 100px; background: #1e1e1e; border: 1px solid #3c3c3c; color: #ccc; padding: 2px 5px; border-radius: 3px;",
			}
		}
	}
}
