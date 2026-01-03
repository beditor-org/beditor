use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::event::{Events, OpenGameEvent};

/// Opens a file dialog to select a game executable and publishes events
pub fn open_project_dialog(events: Events) {
	spawn(async move {
		let result = AsyncFileDialog::new()
			.add_filter("All files", &[""])
			.set_title("Select Game Executable")
			.pick_file()
			.await;

		if let Some(file_handle) = result {
			let file_path = file_handle.path().to_string_lossy().to_string();
			events.publish(OpenGameEvent(file_path));
		}
	});
}
