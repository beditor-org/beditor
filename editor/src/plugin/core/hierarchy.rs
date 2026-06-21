use bridge::protocol::bep::EntityInfo;
use dioxus::prelude::*;

pub fn hierarchy() -> Element {
	let entities = use_context::<Signal<Vec<EntityInfo>>>();
	// let protocol = use_context::<Option<Arc<Mutex<BrpProtocol<std::io::Stdout>>>>>();
	// use_effect(move || {
	// 	if let Some(protocol) = protocol.as_ref() {
	// 		let protocol = protocol.clone();
	// 		tokio::spawn(async move {
	// 			let entities = protocol.lock().await.list_entities().await;
	// 			info!("Received entities: {:?}", entities);
	// 		});
	// 	}
	// });

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
