use tokio::process::{ChildStdin, ChildStdout};

use bridge::{
	multiplexer::Multiplexer,
	protocol::bep::{BepMessage, BepProtocol, EntityInfo},
};
use dioxus::prelude::*;

use crate::{
	event::Events,
	plugin::{
		game_process::{GameProcessAttachedEvent, GameProcessDetachedEvent},
		Plugin, PluginRegistry,
	},
};

const PLUGIN_NAME: &str = "BEP";

pub fn bep_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		entry: Some(entry),
		setup_context: Some(setup_context),
		description: "Plugin responsible for reading frames from the game process".to_string(),
		..Default::default()
	}
}

fn setup_context() -> Element {
	rsx!()
}

fn entry() -> Element {
	let events = use_context::<Events>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let mut game_process_attached = use_signal(|| false);
	let mut world_initialized = use_signal(|| false);

	let mut entities: Signal<Vec<EntityInfo>> = use_context_provider(|| Signal::new(vec![]));

	use_effect(move || {
		if let Some(multiplexer) = multiplexer.read().as_ref() {
			if game_process_attached() && !world_initialized() {
				info!("Game process is attached, setting up World Protocol");
				let protocol = multiplexer.register_protocol::<BepProtocol>();
				info!("✓ World Protocol initialized");

				spawn(async move {
					loop {
						match protocol.connection.recv_async().await {
							Ok(message) => match message {
								BepMessage::EntitiesListUpdate { entities: new_entities } => {
									entities.set(new_entities);
									println!("Received entities list update {:#?}", entities);
								}
								_ => {}
							},
							Err(_) => break,
						}
					}
				});

				world_initialized.set(true);
			}
		} else {
			info!("World Plugin: Multiplexer is not available in entry");
		}
	});

	use_hook(move || {
		events.subscribe(move |_event: &GameProcessAttachedEvent| {
			game_process_attached.set(true);
		});
		events.subscribe(move |_event: &GameProcessDetachedEvent| {
			game_process_attached.set(false);
			world_initialized.set(false);
		});
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
	});

	rsx!()
}
