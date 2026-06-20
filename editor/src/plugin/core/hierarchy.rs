use dioxus::prelude::*;

pub fn hierarchy() -> Element {
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
	}
}
