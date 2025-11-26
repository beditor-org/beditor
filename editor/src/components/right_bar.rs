use dioxus::{core::Element, prelude::*};

#[component]
pub fn RightPanel() -> Element {
	let mut foo = use_signal(|| 1);
	let bar = foo * 10;

	// let state = use_context::<Arc<RwLock<S>>>();

	// let selected_name = use_memo(move || {
	// 	state
	// 		.read()
	// 		.ok()
	// 		.and_then(|s| s.selected_entity.clone())
	// 		.unwrap_or_else(|| "Nothing selected".to_string())
	// });

	rsx! {
		div {
			class: "panel right-panel",
			h3 { class: "panel-title", "Inspector" }
			span {
				"foo: {foo}"
				"bar: {bar}"
			}
			button {
				onclick: move |_| {
					foo.set(foo+1);
				},
				"Increase"
			  }
			// if selected_name() != "Nothing selected" {
			// 	div { class: "properties",
			// 		h4 { style: "color: #ccc; margin: 10px 0;", "{selected_name}" }

			// 		PropertyGroup { title: "Transform" }
			// 		Property { label: "Position X", value: "0.0" }
			// 		Property { label: "Position Y", value: "0.0" }
			// 		Property { label: "Position Z", value: "0.0" }

			// 		Property { label: "Rotation X", value: "0.0" }
			// 		Property { label: "Rotation Y", value: "0.0" }
			// 		Property { label: "Rotation Z", value: "0.0" }
			// 	}
			// } else {
			// 	div {
			// 		style: "padding: 20px; color: #888;",
			// 		"Select an entity to inspect"
			// 	}
			// }
		}
	}
}
