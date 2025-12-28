use dioxus::prelude::*;
use std::{
	cell::Cell,
	process::{Child, ChildStdin, ChildStdout, Command, Stdio},
	sync::{Arc, Mutex},
};
use tracing::info;

use crate::{
	event::{Events, OpenGameEvent},
	plugin::{Plugin, PluginRegistry},
};

pub struct RenderViewportEvent {}
pub struct GameProcessStartedEvent {
	pub child: Arc<Child>,
	pub stdin: Cell<Option<ChildStdin>>,
	pub stdout: Cell<Option<ChildStdout>>,
}

#[derive(Clone)]
pub struct GameProcess {
	pub child: Arc<Child>,
	pub stdin: Arc<Mutex<Option<ChildStdin>>>,
	pub stdout: Arc<Mutex<Option<ChildStdout>>>,
}

const PLUGIN_NAME: &str = "Game Process";
pub fn game_process_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		entry: Some(entry),
		setup_context: Some(setup_context),
		description: "Plugin responsible for starting/stopping the game process".to_string(),
		..Default::default()
	}
}

fn setup_context() -> Element {
	use_context_provider(|| Signal::new(None::<GameProcess>));
	rsx!()
}
fn entry() -> Element {
	let events = use_context::<Events>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let mut game_process = use_context::<Signal<Option<GameProcess>>>();

	use_effect(move || {
		events.subscribe::<OpenGameEvent>(move |event| {
			let game_path = &event.0;
			info!("Starting Bevy game process: ${game_path}");

			let mut child = Command::new(game_path)
				.arg("--editor-mode")
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to start game process");

			let stdin = child.stdin.take().expect("Failed to get stdin");
			let stdout = child.stdout.take().expect("Failed to get stdout");
			*game_process.write() = Some(GameProcess {
				child: Arc::new(child),
				stdin: Arc::new(Mutex::new(Some(stdin))),
				stdout: Arc::new(Mutex::new(Some(stdout))),
			});
			info!("✓ Game process started");
		});
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});
	rsx!()
}
