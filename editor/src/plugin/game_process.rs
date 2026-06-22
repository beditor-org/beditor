use anyhow::{anyhow, bail};
use dioxus::{core::use_drop, prelude::*};
use rfd::AsyncFileDialog;
use std::{
	cell::Cell,
	path::Path,
	process::Stdio,
	sync::{Arc, Mutex},
};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tracing::info;

use crate::{
	config::EditorConfig,
	event::{Events, OpenGameEvent, SwitchWorkspaceEvent},
	plugin::{core::plugin::CORE_SCENE_EDITOR_WORKSPACE, Plugin, PluginRegistry},
};

pub struct RenderViewportEvent {}
pub struct GameProcessAttachedEvent {}
pub struct GameProcessDetachedEvent {}
pub struct GameProcessBrpReadyEvent {}
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
	use_context_provider(|| Signal::new(None::<GameProcessManager>));
	rsx!()
}
fn entry() -> Element {
	let events = use_context::<Events>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let game_process = use_context::<Signal<Option<GameProcess>>>();
	let mut manager = use_context::<Signal<Option<GameProcessManager>>>();
	let config = use_context::<Signal<EditorConfig>>();

	use_drop(move || {
		if let Some(process) = game_process.read().as_ref() {
			if let Some(pid) = process.child.id() {
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
		}
	});

	// use_effect(move || {
	// 	let event_clone = events.clone();
	// 	events.subscribe::<OpenGameEvent>(move |event| {
	// 		let game_path = event.0.clone();
	// 		let mut game_process = game_process.clone();
	// 		let mut config = config.clone();
	// 		let event_clone = event_clone.clone();

	// 		// Spawn async task to avoid blocking UI thread
	// 		spawn(async move {
	// 			info!("Starting Bevy game process: ${game_path}");

	// 			// Add to recent projects
	// 			let project_name = std::path::Path::new(&game_path)
	// 				.file_stem()
	// 				.and_then(|s| s.to_str())
	// 				.expect("Failed to get project name")
	// 				.to_string();

	// 			config.write().add_recent_project(RecentProject {
	// 				name: project_name,
	// 				path: game_path.clone(),
	// 			});
	// 			// Switch to editor workspace
	// 			// event_clone.publish(SwitchWorkspaceEvent(CORE_SCENE_EDITOR_WORKSPACE.clone()));
	// 			// event_clone.publish(GameProcessAttachedEvent {});
	// 		});
	// 	});
	// });

	use_hook(|| {
		*manager.write() = Some(GameProcessManager::new(game_process, events.clone()));
		let events_clone = events.clone();
		events.subscribe::<OpenGameEvent>(move |event| {
			let path = std::path::PathBuf::from(&event.0);
			if let Some(mgr) = manager.write().as_mut() {
				if let Err(e) = mgr.spawn(&path) {
					tracing::error!("Failed to spawn game process: {e}");
				} else {
					events_clone.publish(SwitchWorkspaceEvent(CORE_SCENE_EDITOR_WORKSPACE.clone()));
				}
			}
		});
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("{PLUGIN_NAME} plugin initialized!");
	});
	rsx!()
}

#[derive(Clone)]
pub struct GameProcessManager {
	process: Signal<Option<GameProcess>>,
	events: Events,
}

impl GameProcessManager {
	pub fn new(process: Signal<Option<GameProcess>>, events: Events) -> Self {
		Self { process, events }
	}

	pub fn select(&self) {
		let mut manager = self.clone();
		spawn(async move {
			let result = AsyncFileDialog::new().set_title("Select Game project").pick_folder().await;

			if let Some(file_handle) = result {
				manager.spawn(file_handle.path());
			}
		});
	}

	pub fn spawn(&mut self, project_path: &Path) -> anyhow::Result<()> {
		if self.process.read().is_some() {
			bail!("Process already running");
		}

		let asset_path = std::env::current_dir()
			.map(|cwd| cwd.join("client/examples").to_string_lossy().to_string())
			.unwrap_or_else(|_| "assets".to_string());
		let mut child = Command::new(&project_path)
			.arg("--editor-mode")
			.env("BEDITOR_ASSET_PATH", &asset_path)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.spawn()
			.map_err(|_| anyhow!("Failed to start game process"))?;

		let stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to get stdin"))?;
		let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to get stdout"))?;
		*self.process.write() = Some(GameProcess {
			child: Arc::new(child),
			stdin: Arc::new(Mutex::new(Some(stdin))),
			stdout: Arc::new(Mutex::new(Some(stdout))),
		});

		self.events.publish(GameProcessAttachedEvent {});
		info!("✓ Game process started");
		Ok(())
	}
}
