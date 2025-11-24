use std::sync::{Arc, RwLock};

use dioxus::{core::Element, prelude::*};

#[component]
pub fn LeftPanel<S: 'static>() -> Element {
	let state = use_context::<Arc<RwLock<S>>>();
	let selected = use_signal(|| None::<String>);
	let mut update_trigger = use_signal(|| 0u32);

	// Synchronize with global state
	let state_clone = state.clone();
	use_effect(move || {
		// if let Some(sel) = selected() {
		// 	if let Ok(mut s) = state_clone.write() {
		// 		s.selected_entity = Some(sel);
		// 	}
		// }
	});

	// Poll for state changes periodically (less frequently to reduce CPU usage)
	use_effect(move || {
		spawn(async move {
			loop {
				tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
				update_trigger.set(update_trigger() + 1);
			}
		});
	});

	// Read state reactively
	let _ = update_trigger(); // Subscribe to updates
						   // let entities = state.read().map(|s| s.entities.clone()).unwrap_or_default();
						   // let game_connected = state.read().map(|s| s.game_connected).unwrap_or(false);

	rsx! {
		div {
			class: "panel left-panel",
			h3 { class: "panel-title", "Hierarchy" }

			// if !game_connected {
			// 	div {
			// 		style: "padding: 20px; color: #888;",
			// 		"⏳ Connecting to Game..."
			// 	}
			// } else if entities.is_empty() {
			// 	div {
			// 		style: "padding: 20px; color: #888;",
			// 		"📦 No entities found"
			// 	}
			// } else {
			// 	div { class: "tree-view",
			// 		for entity in entities.iter() {
			// 			TreeItem {
			// 				name: entity.name.clone(),
			// 				selected: selected() == Some(entity.name.clone()),
			// 				onclick: {
			// 					let entity_name = entity.name.clone();
			// 					move |_| selected.set(Some(entity_name.clone()))
			// 				}
			// 			}
			// 		}
			// 	}
			// }
		}
	}
}
