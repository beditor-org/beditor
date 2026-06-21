use bridge::protocol::bep::EntityInfo;
use dioxus::prelude::*;

pub fn hierarchy() -> Element {
	let entities = use_context::<Signal<Vec<EntityInfo>>>();

	rsx! {
		h2{
			class: "text-2xl font-semibold mb-2",
			"Hierarchy"
		}
		ul {
			class: "list-disc list-inside",
			{entities.read().iter().map(|entity| rsx!(
				li {
					key: "{entity.id}",
					{format!("{} (ID: {})", entity.name, entity.id)}
				}
			))}
		}
	}
}
