use bridge::multiplexer::Multiplexer;
use bytesize::ByteSize;
use dioxus::prelude::*;
use std::sync::atomic::Ordering;

use crate::{
	event::Events,
	plugin::{core::plugin::CORE_STATUS_BAR_PANEL, game_process::GameProcess, Plugin, PluginRegistry},
	tool::ToolPlacement,
	Tool, ToolAlignment,
};
use tokio::process::{ChildStdin, ChildStdout};

pub struct StdioTransportReadyEvent;
pub struct StdioTransportPlugin;

const PLUGIN_NAME: &str = "Stdio Transport";

pub fn stdio_transport_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		entry: Some(entry),
		setup_context: Some(setup_context),
		description: "Implements Stdio Transport".to_string(),
		tools: vec![Tool {
			placement: ToolPlacement::ByResourceId(CORE_STATUS_BAR_PANEL.clone()),
			name: "Dumy tool".to_string(),
			component: counter,
			alignment: ToolAlignment::End,
			workspaces: vec![],
		}],
		..Default::default()
	}
}

fn setup_context() -> Element {
	use_context_provider(|| Signal::new(None::<Multiplexer<ChildStdout, ChildStdin>>));
	rsx!()
}

fn entry() -> Element {
	let mut multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let game_process = use_context::<Signal<Option<GameProcess>>>();
	let events = use_context::<Events>();

	use_effect(move || {
		let gp_data = game_process.read().clone();

		match gp_data.as_ref() {
			Some(gp) => match (gp.stdin.lock().unwrap().take(), gp.stdout.lock().unwrap().take()) {
				(Some(stdin), Some(stdout)) => {
					let mut mux = Multiplexer::new(stdout, stdin);
					mux.start();
					multiplexer.set(Some(mux));
					events.publish(StdioTransportReadyEvent {});
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
			None => {
				warn!("Game process not started yet, cannot create stdio multiplexer");
			}
		}
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});

	rsx!()
}

fn counter() -> Element {
	let multiplexer = use_context::<Signal<Option<Multiplexer<ChildStdout, ChildStdin>>>>();
	let stats = multiplexer.read().as_ref().map(|mux| {
		(
			mux.bytes_sent.load(Ordering::Relaxed),
			mux.bytes_received.load(Ordering::Relaxed),
		)
	});

	rsx! {
		div {
			if let Some((sent, received)) = stats {
				"stdio: ↑ {ByteSize(sent)} | ↓ {ByteSize(received)}"
			} else {
				"stdio: Not connected"
			}
		}

	}
}
