use std::{
	cell::Cell,
	process::{Child, ChildStdin, ChildStdout, Command, Stdio},
	sync::Arc,
};

use tracing::info;

use crate::{
	event::{Events, OpenGameEvent},
	resource::ResourceRegistry,
	Plugin,
};

pub struct RenderViewportEvent {}
pub struct GameProcessStartedEvent {
	pub child: Arc<Child>,
	pub stdin: Cell<Option<ChildStdin>>,
	pub stdout: Cell<Option<ChildStdout>>,
}
pub struct GameProcessEndedEvent;

// plugin responsible for starting/stopping the game process
pub struct GameProcessPlugin;
impl Plugin for GameProcessPlugin {
	fn get_name(&self) -> String {
		"Game Process Plugin".to_string()
	}

	fn get_description(&self) -> String {
		todo!()
	}

	fn on_load(&mut self, resources: ResourceRegistry) {
		let events = resources.get::<Events>().unwrap();

		events.clone().subscribe::<OpenGameEvent>(move |event| {
			let game_path = &event.0;
			info!("🚀 Starting Bevy game process: ${game_path}");

			let mut child = Command::new(game_path)
				.arg("--editor-mode")
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to start game process");

			let stdin = child.stdin.take().expect("Failed to get stdin");
			let stdout = child.stdout.take().expect("Failed to get stdout");
			info!("✓ Game process started");
			events.publish(GameProcessStartedEvent {
				child: Arc::new(child),
				stdin: Cell::new(Some(stdin)),
				stdout: Cell::new(Some(stdout)),
			});
		});
	}
}
