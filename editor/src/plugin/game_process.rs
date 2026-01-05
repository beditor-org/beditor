use dioxus::{core::use_drop, prelude::*};
use std::{
	cell::Cell,
	process::{Child, ChildStdin, ChildStdout, Command, Stdio},
	sync::{Arc, Mutex},
};
use tracing::info;

use crate::{
	config::{EditorConfig, RecentProject},
	event::{Events, OpenGameEvent, SwitchWorkspaceEvent},
	plugin::{core::plugin::CORE_EDITOR_WORKSPACE, Plugin, PluginRegistry},
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
	let game_process = use_context::<Signal<Option<GameProcess>>>();
	let config = use_context::<Signal<EditorConfig>>();

	use_drop(move || {
		if let Some(process) = game_process.read().as_ref() {
			let pid = process.child.id();
			#[cfg(unix)]
			{
				use std::process::Command as SysCommand;
				let _ = SysCommand::new("kill").arg(pid.to_string()).status();
				info!("✓ Sent SIGTERM to game process (PID: {})", pid);
			}
			#[cfg(windows)]
			{
				use std::process::Command as SysCommand;
				let _ = SysCommand::new("taskkill").args(["/PID", &pid.to_string(), "/F"]).status();
				info!("✓ Killed game process (PID: {})", pid);
			}
		}
	});

	use_effect(move || {
		let event_clone = events.clone();
		events.subscribe::<OpenGameEvent>(move |event| {
			let game_path = event.0.clone();
			let mut game_process = game_process.clone();
			let mut config = config.clone();
			let event_clone = event_clone.clone();

			// Spawn async task to avoid blocking UI thread
			spawn(async move {
				info!("Starting Bevy game process: ${game_path}");

				let mut child = Command::new(&game_path)
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

				// Add to recent projects
				let project_name = std::path::Path::new(&game_path)
					.file_stem()
					.and_then(|s| s.to_str())
					.expect("Failed to get project name")
					.to_string();

				config.write().add_recent_project(RecentProject {
					name: project_name,
					path: game_path.clone(),
				});
				// Switch to editor workspace
				event_clone.publish(SwitchWorkspaceEvent(CORE_EDITOR_WORKSPACE.clone()));
			});
		});
	});

	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});
	rsx!()
}
