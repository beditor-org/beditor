use std::sync::{Arc, Mutex};

use bridge::protocol::camera::CameraInputProtocol;
use bridge::protocol::camera::MouseEvent;
use dioxus::{document::eval, prelude::*};
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

pub fn Viewport() -> Element {
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let camera_input = use_context::<Signal<Option<Arc<Mutex<CameraInputProtocol>>>>>();
	let canvas_id = "viewport-canvas";

	// Mount: start the rAF render loop once. The loop reads window.__vp_version
	// which Rust updates via a tiny eval on each new frame signal.
	use_effect(move || {
		viewport_state.write().is_opened = true;
		info!("Viewport component mounted, starting rAF loop");

		let canvas_id = canvas_id.to_string();
		spawn(async move {
			let init_js = format!(
				r#"(function() {{
				const canvas = document.getElementById('{}');
				if (!canvas) return;
				const W = 1280, H = 720;
				// Canvas fills container via CSS (w-full h-full).
				// We match the buffer to the container size and use drawImage
				// for scaling — this uses bilinear filtering guaranteed.
				const ctx = canvas.getContext('2d', {{ alpha: false }});
				const container = canvas.parentElement;
				// Intermediate canvas at native game resolution for putImageData
				const src = document.createElement('canvas');
				src.width = W; src.height = H;
				const srcCtx = src.getContext('2d');
				function resize() {{
					const cw = container.clientWidth || W;
					const ch = container.clientHeight || H;
					if (canvas.width !== cw || canvas.height !== ch) {{
						canvas.width = cw;
						canvas.height = ch;
					}}
				}}
				resize();
				new ResizeObserver(resize).observe(container);
				let pendingVersion = 0;
				let fetching = false;
				window.__vp_update = function(v) {{ pendingVersion = v; }};
				function draw() {{
					requestAnimationFrame(draw);
					if (pendingVersion === 0 || fetching) return;
					const v = pendingVersion;
					fetching = true;
					fetch('beditor://frame/?v=' + v)
						.then(r => r.arrayBuffer())
						.then(buf => {{
							fetching = false;
							if (buf.byteLength !== W * H * 4) return;
							const cw = canvas.width, ch = canvas.height;
							const scale = Math.min(cw / W, ch / H);
							const dx = (cw - W * scale) / 2;
							const dy = (ch - H * scale) / 2;
							srcCtx.putImageData(new ImageData(new Uint8ClampedArray(buf), W, H), 0, 0);
							ctx.clearRect(0, 0, cw, ch);
							ctx.drawImage(src, dx, dy, W * scale, H * scale);
						}})
						.catch(() => {{ fetching = false; }});
				}}
				requestAnimationFrame(draw);
			}})()"#,
				canvas_id
			);
			eval(&init_js).await.ok();
		});
	});

	// On each new frame signal: just poke the JS version counter (tiny eval)
	use_effect(move || {
		let version = viewport_state.read().frame_version;
		if version == 0 {
			return;
		}
		spawn(async move {
			eval(&format!("window.__vp_update && window.__vp_update({version})"))
				.await
				.ok();
		});
	});

	let mut is_dragging = use_signal(|| false);
	let mut last_mouse_pos = use_signal(|| (0.0, 0.0));

	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900 overflow-hidden",
			canvas {
				id: "{canvas_id}",
				class: "w-full h-full",
				style: "image-rendering: auto; display: block;",
				onmousedown: move |evt| {
					if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_dragging.set(true);
						let coords = evt.page_coordinates();
						last_mouse_pos.set((coords.x, coords.y));
						trace!("Right mouse button pressed - camera control started");
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
								scroll: 0.0,
							});
							trace!("Camera drag: dx={:.1}, dy={:.1}", dx, dy);
						}
					}
				},
				onmouseup: move |evt| {
					if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_dragging.set(false);
						trace!("Right mouse button released - camera control stopped");
					}
				},
				onwheel: move |evt| {
					evt.prevent_default();
					if let Some(camera_input) = camera_input.read().as_ref() {
						let scroll_delta = match evt.delta() {
							dioxus::html::geometry::WheelDelta::Lines(lines) => lines.y as f32,
							dioxus::html::geometry::WheelDelta::Pages(pages) => pages.y as f32 * 10.0,
							dioxus::html::geometry::WheelDelta::Pixels(pixels) => pixels.y as f32 * 0.05,
						};
						let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
							x: 0.0,
							y: 0.0,
							scroll: scroll_delta,
						});
					}
				},
				oncontextmenu: move |evt| {
					evt.prevent_default();
				}
			}
		}
	}
}
