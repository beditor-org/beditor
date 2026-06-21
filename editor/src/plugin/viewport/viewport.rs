use std::sync::{Arc, Mutex};

use bridge::protocol::camera::CameraInputProtocol;
use bridge::protocol::camera::MouseEvent;
use dioxus::{document::eval, prelude::*};
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

pub fn Viewport() -> Element {
	// ALL HOOKS AT THE TOP - ALWAYS CALLED
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let camera_input = use_context::<Signal<Option<Arc<Mutex<CameraInputProtocol>>>>>();
	let canvas_id = "viewport-canvas";

	// Hook 1: Mount effect — force redraw with last known frame on remount
	use_effect(move || {
		viewport_state.write().is_opened = true;
		info!("Viewport component mounted, viewport opened");
	});

	use_effect(move || {
		let data_opt = viewport_state.read().frame.clone();
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

					const container = canvas.parentElement;
					const w = container.clientWidth;
					const h = container.clientHeight;
					if (w > 0 && h > 0 && (canvas.width !== w || canvas.height !== h)) {{
						canvas.width  = w;
						canvas.height = h;
					}}

					const ctx = canvas.getContext('2d', {{ alpha: false }});
					const img = new Image();
					img.onload = function() {{
						ctx.clearRect(0, 0, canvas.width, canvas.height);
						const scale = Math.min(canvas.width / this.width, canvas.height / this.height);
						const x = (canvas.width  - this.width  * scale) / 2;
						const y = (canvas.height - this.height * scale) / 2;
						ctx.drawImage(this, x, y, this.width * scale, this.height * scale);
					}};
					img.src = 'data:image/jpeg;base64,{}';
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
						trace!("🖱️ Right mouse button pressed - camera control started");
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
							let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
									x: dx as f32,
									y: dy as f32,
								});
							// camera_input.lock().unwrap().connection.send();
							trace!("🎥 Camera drag: dx={:.1}, dy={:.1}", dx, dy);
						}
					}
				},
				onmouseup: move |evt| {
					if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_dragging.set(false);
						trace!("🖱️ Right mouse button released - camera control stopped");
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
