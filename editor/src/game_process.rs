use anyhow::Result;
use std::{
	io::{BufRead, BufReader, Write},
	process::{Child, ChildStdin, Command, Stdio},
	sync::{Arc, Mutex},
};

/// Manages the Bevy game process and IPC communication
pub struct GameProcessManager {
	child: Option<Child>,
	stdin: Option<Arc<Mutex<ChildStdin>>>,
}

impl GameProcessManager {
	pub fn new() -> Self {
		Self {
			child: None,
			stdin: None,
		}
	}

	/// Start the game process with IPC
	/// Returns a channel receiver for frame data
	pub fn start(&mut self, game_path: &str) -> Result<std::sync::mpsc::Receiver<Vec<u8>>> {
		eprintln!("🚀 Starting Bevy game process: {}", game_path);

		let mut child = Command::new(game_path)
			.arg("--editor-mode")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.spawn()?;

		let stdin = child.stdin.take().expect("Failed to get stdin");
		let stdout = child.stdout.take().expect("Failed to get stdout");

		self.stdin = Some(Arc::new(Mutex::new(stdin)));
		self.child = Some(child);

		// Bounded channel with capacity=1: only keep latest frame
		let (tx, rx) = std::sync::mpsc::sync_channel::<Vec<u8>>(1);

		// Read stdout in separate OS thread (not async!)
		std::thread::spawn(move || {
			let reader = BufReader::new(stdout);
			eprintln!("✓ Game process started, reading stdout...");

			for line in reader.lines() {
				let text = match line {
					Ok(t) => t,
					Err(_) => break,
				};

				if text.is_empty() {
					continue;
				}

				// Check if this is a frame data message
				if text.starts_with("FRAME: ") {
					use base64::{engine::general_purpose, Engine as _};
					if let Some(base64_data) = text.strip_prefix("FRAME: ").map(|s| s.trim()) {
						if let Ok(frame_bytes) = general_purpose::STANDARD.decode(base64_data) {
							// eprintln!("📸 Frame received: {} bytes", frame_bytes.len());
							// try_send drops old frame if channel full - never blocks!
							let _ = tx.try_send(frame_bytes);
						} else {
							eprintln!("❌ Failed to decode base64 frame data");
						}
					}
				} else if text.starts_with("BRP:") {
					eprintln!("📡 BRP response: {}", text);
				} else {
					eprintln!("📡 Game: {}", text);
				}
			}
			eprintln!("📡 Game stdout reader exiting");
		});

		Ok(rx)
	}

	/// Send a command to the game via stdin
	pub fn send_command(&self, command: &str) -> Result<()> {
		if let Some(stdin_arc) = &self.stdin {
			let mut stdin = stdin_arc.lock().unwrap();
			stdin.write_all(command.as_bytes())?;
			stdin.write_all(b"\n")?;
			stdin.flush()?;
		}
		Ok(())
	}

	/// Request a frame capture from the game
	pub fn request_frame(&self) -> Result<()> {
		self.send_command("CAPTURE_FRAME")
	}

	/// Stop the game process
	pub fn stop(&mut self) -> Result<()> {
		if let Some(mut child) = self.child.take() {
			eprintln!("🛑 Stopping game process");
			child.kill()?;
			child.wait()?;
		}
		self.stdin = None;
		Ok(())
	}

	pub fn is_running(&self) -> bool {
		self.child.is_some()
	}
}

impl Drop for GameProcessManager {
	fn drop(&mut self) {
		// Note: can't await in Drop, game will be killed when Child drops
		if let Some(child) = self.child.take() {
			drop(child); // explicit drop
		}
	}
}
