use std::sync::{Arc, Mutex};

use bridge::protocol::frame_stream::FrameStreamProtocol;
use dioxus::prelude::*;
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

pub fn Viewport() -> Element {
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let frame = use_signal(|| None::<String>);
	let protocol_signal = use_context::<Signal<Option<Arc<Mutex<FrameStreamProtocol>>>>>();

	use_hook(|| {
		viewport_state.write().is_opened = true;
		info!("Viewport component mounted, viewport opened");
	});

	let protocol = protocol_signal;
	use_effect(move || {
		if let Some(protocol_arc) = protocol.read().clone() {
			info!("Starting frame receiver task");
			let (tx, mut rx) = tokio::sync::watch::channel(None::<String>);

			std::thread::spawn(move || loop {
				let result = protocol_arc.lock().unwrap().connection.reader.recv();
				match result {
					Ok(data) => {
						if let Ok(base64_string) = String::from_utf8(data) {
							let _ = tx.send(Some(base64_string));
						}
					}
					Err(_) => break,
				}
			});

			let mut frame_clone = frame;
			spawn(async move {
				while rx.changed().await.is_ok() {
					frame_clone.set(rx.borrow().clone());
					viewport_state.write().frame_count += 1;
				}
			});
		}
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
