use std::process::{ChildStdin, ChildStdout};

use bridge::{codec::json::JsonCodec, connection::Connection, multiplexer::Multiplexer, protocol::brp::BrpProtocol};
use dioxus::prelude::*;

use crate::{
	event::Events,
	plugin::{
		game_process::{GameProcessAttachedEvent, GameProcessDetachedEvent},
		Plugin, PluginRegistry,
	},
};

const PLUGIN_NAME: &str = "BRP";

pub fn brp_plugin() -> Plugin {
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
	use_effect(move || {
		if let Some(multiplexer) = multiplexer.read().as_ref() {
			if game_process_attached() {
				info!("Game process is attached, setting up BRP Protocol");
				let connection = Connection::new(
					JsonCodec,
					multiplexer.register_for_type::<BrpProtocol>(),
					multiplexer.get_writer_for_type::<BrpProtocol>(),
				);
				let mut protocol = BrpProtocol::new(connection);
				protocol.list_entities();

				// connection.send(&"Hello from BRP Plugin".to_string()).unwrap();
				info!("✓ BRP Protocol added to multiplexer");
			}
		} else {
			info!("BRP Plugin: Multiplexer is not available in entry");
		}
	});

	use_hook(|| {
		events.subscribe(move |event: &GameProcessAttachedEvent| {
			game_process_attached.set(true);
		});
		events.subscribe(move |event: &GameProcessDetachedEvent| {
			game_process_attached.set(false);
		});
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
	});

	rsx!()
}
