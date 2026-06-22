use bridge::protocol::bep::ComponentData;
use dioxus::prelude::*;

#[component]
pub fn Inspector() -> Element {
	let selected_entity = use_context::<Signal<Option<u32>>>();
	let components = use_context::<Signal<Vec<ComponentData>>>();
	let components = components.read();

	rsx! {
		div { class: "flex flex-col flex-1 min-h-0 overflow-y-auto p-2",
			if selected_entity.read().is_none() {
				div { class: "text-muted-foreground text-sm p-2", "Select an entity to see its components" }
			} else {
				div { class: "text-muted-foreground text-sm p-2", "Components for entity {selected_entity.read().unwrap()}" }
				for component in components.iter() {
					div { class: "text-sm p-2 border-b border-border", "{component.short_name}" }
				}
			}
		}
	}
}
