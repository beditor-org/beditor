use std::sync::{Arc, Mutex};

use crate::resource::ResourceRegistry;
use bridge::protocol::frame_stream::FrameStreamProtocol;
use dioxus::prelude::*;
use tokio::sync::watch;

/// Viewport component: renders frames from game to canvas
#[component]
pub fn Viewport() -> Element {
	let mut frame = use_signal(|| None::<String>);
	let resources = use_context::<Arc<ResourceRegistry>>();

	// Create thread-safe channel for frame updates
	use_hook(|| {
		let (tx, mut rx) = watch::channel(None::<String>);
		let protocol = resources.get::<Arc<Mutex<FrameStreamProtocol>>>().unwrap();

		// Spawn thread for blocking recv
		std::thread::spawn(move || loop {
			let result = protocol.lock().unwrap().connection.reader.recv();
			match result {
				Ok(data) => {
					if let Ok(base64_string) = String::from_utf8(data) {
						let _ = tx.send(Some(base64_string));
					}
				}
				Err(_) => break,
			}
		});

		// Spawn async task to update Signal
		spawn(async move {
			while rx.changed().await.is_ok() {
				frame.set(rx.borrow().clone());
			}
		});
	});

	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900",
			if let Some(data) = frame() {
				img { src: "data:image/png;base64,{data}" }
			} else {
				div { "No frame" }
			}
		}
	}
}
