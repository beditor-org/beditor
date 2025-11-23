use crate::config::{EditorConfig, WindowConfig};
use std::{
	process::{Child, ChildStdin, Command, Stdio},
	sync::{Arc, RwLock},
};

// pub struct Editor {
// 	config: EditorConfig,
// }

// impl Editor {
// 	pub fn new(config: EditorConfig) -> Self {
// 		Self { config }
// 	}

// 	pub fn create_window(&self, window_config: WindowConfig) -> dioxus::desktop::WindowBuilder {
// 		dioxus::desktop::WindowBuilder::new()
// 			.with_title(window_config.title.clone())
// 			.with_inner_size(dioxus::desktop::LogicalSize::new(window_config.width, window_config.height))
// 			.with_position(dioxus::desktop::LogicalPosition::new(window_config.x, window_config.y))
// 			.with_decorations(true)
// 			.with_resizable(false)
// 	}
// }

pub fn spawn_game_process(path: &str, x: i32, y: i32, width: u32, height: u32, state: Arc<RwLock<EditorState>>) {
	println!("🚀 Spawning borderless Bevy game window...");

	let game_path = "../bevy_demo_game/target/debug/bevy_demo_game";

	std::thread::spawn(move || {
		match Command::new(game_path)
			.arg("--editor-mode")
			.arg("--no-decorations")
			.arg("--window-x")
			.arg(x.to_string())
			.arg("--window-y")
			.arg(y.to_string())
			.arg("--window-width")
			.arg(width.to_string())
			.arg("--window-height")
			.arg(height.to_string())
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.spawn()
		{
			Ok(mut child) => {
				println!("✓ Borderless game window started with PID: {:?}", child.id());
				println!("  Viewport: {}x{} at ({}, {})", width, height, x, y);

				let stdin = child.stdin.take().expect("Failed to get stdin");
				let stdout = child.stdout.take().expect("Failed to get stdout");

				// Store game process in state
				let game_process = Arc::new(Mutex::new(GameProcess { stdin, _child: child }));
				if let Ok(mut s) = state.write() {
					s.game_process = Some(game_process.clone());
					s.game_connected = true;
				}

				// Start stdout reader thread
				let state_clone = state.clone();
				std::thread::spawn(move || {
					let reader = BufReader::new(stdout);
					for line in reader.lines() {
						if let Ok(text) = line {
							if text.is_empty() {
								continue;
							}

							// Parse as owned BRP response
							if let Ok(response) = serde_json::from_str::<OwnedBrpResponse>(&text) {
								handle_brp_response(response, &state_clone);
							}
						}
					}
					eprintln!("📡 stdout reader thread exiting");
					if let Ok(mut s) = state_clone.write() {
						s.game_connected = false;
					}
				});

				// Send initial BRP query to list all entities with Name component
				std::thread::sleep(std::time::Duration::from_millis(100));
				send_brp_query_entities(&game_process);
			}
			Err(e) => eprintln!("❌ Failed to spawn game: {}", e),
		}
	});
}
