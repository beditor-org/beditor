use std::{
	process::{ChildStdin, ChildStdout},
	sync::{Arc, Mutex},
};

use bridge::{
	codec::json::JsonCodec,
	connection::Connection,
	multiplexer::Multiplexer,
	protocol::{camera::CameraInputProtocol, frame_stream::FrameStreamProtocol},
};
use dioxus::{document::eval, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

#[derive(Serialize, Deserialize, Debug)]
struct MouseEvent {
	x: f64,
	y: f64,
}
pub fn Viewport() -> Element {
	// ALL HOOKS AT THE TOP - ALWAYS CALLED
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let frame = use_signal(|| None::<String>);
	let protocol_signal = use_context::<Signal<Option<Arc<Mutex<FrameStreamProtocol>>>>>();
	let camera_input = use_context::<Signal<Option<Arc<Mutex<CameraInputProtocol>>>>>();
	let canvas_id = "viewport-canvas";

	// Hook 1: Mount effect
	use_hook(|| {
		viewport_state.write().is_opened = true;

		info!("Viewport component mounted, viewport opened");
	});

	// Hook 2: Frame receiver
	use_effect(move || {
		let protocol_opt = protocol_signal.read().clone();
		if let Some(protocol_arc) = protocol_opt {
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

	use_effect(move || {
		let data_opt = frame();
		let data = match data_opt {
			Some(d) => d,
			None => return,
		};

		let canvas_id = canvas_id.to_string();
		spawn(async move {
			let eval_js = format!(
				r#"
				(function() {{
					const canvas = document.getElementById('{}');
					if (!canvas) return;
					
					const ctx = canvas.getContext('2d', {{ alpha: false }});
					
					if (!window.__viewportImage) {{
						window.__viewportImage = new Image();
						window.__viewportImage.onload = function() {{
							if (canvas.width !== this.width || canvas.height !== this.height) {{
								canvas.width = this.width;
								canvas.height = this.height;
							}}
							ctx.drawImage(this, 0, 0);
						}};
					}}
					
					window.__viewportImage.src = 'data:image/jpeg;base64,{}';
				}})();
				"#,
				canvas_id, data
			);
			eval(&eval_js).await.ok();
		});
	});

	// Mouse state for camera control
	let mut is_dragging = use_signal(|| false);
	let mut last_mouse_pos = use_signal(|| (0.0, 0.0));

	// Render - no hooks here
	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900 overflow-hidden",
			canvas {
				id: "{canvas_id}",
				class: "w-full h-full",
				style: "image-rendering: auto; object-fit: contain;",
				onmousedown: move |evt| {
					if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_dragging.set(true);
						let coords = evt.page_coordinates();
						last_mouse_pos.set((coords.x, coords.y));
						info!("🖱️ Right mouse button pressed - camera control started");
					}
				},
				onmousemove: move |evt| {
					if is_dragging() {
						if let Some(camera_input) = camera_input.read().as_ref() {

							let coords = evt.page_coordinates();
							let (last_x, last_y) = last_mouse_pos();
							let dx = coords.x - last_x;
							let dy = coords.y - last_y;
							last_mouse_pos.set((coords.x, coords.y));
							camera_input.lock().unwrap().connection.send(to_value(&MouseEvent {
									x: dx as f64,
									y: dy as f64,
								}).unwrap());
							// camera_input.lock().unwrap().connection.send();
							info!("🎥 Camera drag: dx={:.1}, dy={:.1}", dx, dy);
						}
					}
				},
				onmouseup: move |evt| {
					if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_dragging.set(false);
						info!("🖱️ Right mouse button released - camera control stopped");
					}
				},
				// Prevent context menu on right click
				oncontextmenu: move |evt| {
					evt.prevent_default();
				}
			}
		}
	}
}
