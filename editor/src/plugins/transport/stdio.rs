use std::process::{ChildStdin, ChildStdout};

use bridge::multiplexer::Multiplexer;
use tracing::{info, warn};

use crate::{
	event::Events,
	plugins::game_process::{GameProcessEndedEvent, GameProcessStartedEvent},
	resource::ResourceRegistry,
	Plugin,
};

pub struct StdioTransportReadyEvent;
pub struct StdioTransportPlugin;

impl Plugin for StdioTransportPlugin {
	fn on_load(&mut self, resources: ResourceRegistry) {
		let events = resources.get::<Events>().unwrap();
		let e_clone = events.clone();
		let res_clone = resources.clone();
		events.subscribe::<GameProcessStartedEvent>(move |event| match res_clone.get::<Multiplexer<ChildStdout, ChildStdin>>() {
			Some(_) => warn!("Multiplexer already registered"),
			None => match (event.stdin.take(), event.stdout.take()) {
				(Some(stdin), Some(stdout)) => {
					let mut mux = Multiplexer::new(stdout, stdin);
					mux.start();
					res_clone.register(mux);
					e_clone.publish(StdioTransportReadyEvent {});
					info!("registered stdio multiplexer");
				}
				(None, Some(_)) => {
					warn!("Stdin already taken");
				}
				(Some(_), None) => {
					warn!("Stdout already taken");
				}
				_ => {
					warn!("Stdin and Stdout already taken");
				}
			},
		});

		events.subscribe::<GameProcessEndedEvent>(move |_| {
			resources.unregister::<Multiplexer<ChildStdout, ChildStdin>>();
		});
	}

	fn get_name(&self) -> String {
		"Stdio Transport Plugin".to_string()
	}

	fn get_description(&self) -> String {
		"Implements Stdio Transport".to_string()
	}
}
