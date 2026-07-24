use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use std::path::Path;

use crate::event::Events;

pub struct ProjectOpenedEvent {
	pub project: Project,
}

#[derive(Default)]
pub struct CurrentProject {
	pub project: Option<Project>,
}

#[derive(Clone)]
pub struct Project {
	pub name: String,
	pub path: String,
}

impl From<&Path> for Project {
	fn from(path: &Path) -> Self {
		let name = path
			.file_name()
			.and_then(|name| name.to_str())
			.unwrap_or("Untitled Project")
			.to_string();
		Self {
			name,
			path: path.to_string_lossy().to_string(),
		}
	}
}

pub fn open_project_dialog(events: Events) {
	spawn(async move {
		let result = AsyncFileDialog::new().set_title("Select Game project").pick_folder().await;

		if let Some(file_handle) = result {
			info!("Opened project at path: {}", file_handle.path().display());
			events.publish(ProjectOpenedEvent {
				project: Project::from(file_handle.path()),
			});
		}
	});
}
