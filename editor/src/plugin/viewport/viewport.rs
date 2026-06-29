use std::sync::{Arc, Mutex};

use bridge::protocol::camera::CameraInputProtocol;
use bridge::protocol::camera::MouseEvent;
use dioxus::{document::eval, prelude::*};
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

/// Convert page coordinates to game-image-normalized [0, 1] coords,
/// accounting for letterboxing inside the canvas element.
/// Convert element-relative coordinates (from evt.element_coordinates()) to normalised [0,1]
/// game-image coordinates accounting for letterbox / pillarbox.
/// canvas_wh = (canvas_css_width, canvas_css_height) as returned by get_client_rect().
fn to_game_coords(elem_x: f64, elem_y: f64, canvas_wh: (f64, f64)) -> (f32, f32) {
	const W: f64 = 1280.0;
	const H: f64 = 720.0;
	let (cw, ch) = canvas_wh;
	let scale = (cw / W).min(ch / H);
	if scale < 1e-6 {
		return (0.0, 0.0);
	}
	let dx = (cw - W * scale) / 2.0;
	let dy = (ch - H * scale) / 2.0;
	let abs_x = ((elem_x - dx) / (W * scale)).clamp(0.0, 1.0) as f32;
	let abs_y = ((elem_y - dy) / (H * scale)).clamp(0.0, 1.0) as f32;
	(abs_x, abs_y)
}

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
					const rs = Math.min(cw / W, ch / H);
					window.__vp_img_rect = [(cw - W*rs)/2, (ch - H*rs)/2, W*rs, H*rs];
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
						window.__vp_img_rect = [dx, dy, W * scale, H * scale];
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
	let mut is_lmb_down = use_signal(|| false);
	let mut last_mouse_pos = use_signal(|| (0.0, 0.0));
	// Canvas bounding rect (left, top, width, height) in page coords, for letterbox-correct
	// game-image coordinate mapping. Polled every 500 ms via JS eval.
	// Canvas display size in CSS pixels, updated via onmounted + get_client_rect().
	// Used to compute the letterbox offset for correct game-image coordinate mapping.
	let mut canvas_wh = use_signal(|| (1280.0_f64, 720.0_f64));

	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900 overflow-hidden",
			canvas {
				onmounted: move |evt| {
					// Spawn a loop that refreshes the canvas CSS size every 300 ms.
					// get_client_rect() uses WebView platform API — no JS eval needed.
					spawn(async move {
						loop {
							if let Ok(rect) = evt.get_client_rect().await {
								let w = rect.size.width;
								let h = rect.size.height;
								if w > 10.0 && h > 10.0 {
									canvas_wh.set((w, h));
								}
							}
							tokio::time::sleep(std::time::Duration::from_millis(300)).await;
						}
					});
				},
				id: "{canvas_id}",
				class: "w-full h-full",
				style: "image-rendering: auto; display: block;",
				onmousedown: move |evt| {
					let coords = evt.page_coordinates();
					let btn = evt.trigger_button();
					if btn == Some(dioxus::html::input_data::MouseButton::Auxiliary) {
						is_dragging.set(true);
						last_mouse_pos.set((coords.x, coords.y));
					} else if btn == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_lmb_down.set(true);
						let elem = evt.element_coordinates();
						let (abs_x, abs_y) = to_game_coords(elem.x, elem.y, canvas_wh());
						if let Some(camera_input) = camera_input.read().as_ref() {
							let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
								abs_x, abs_y, lmb_pressed: true, ..Default::default()
							});
						}
					}
				},
				onmousemove: move |evt| {
					let coords = evt.page_coordinates();
					let elem = evt.element_coordinates();
					let (abs_x, abs_y) = to_game_coords(elem.x, elem.y, canvas_wh());
					if is_dragging() {
						if let Some(camera_input) = camera_input.read().as_ref() {
							let (last_x, last_y) = last_mouse_pos();
							let dx = (coords.x - last_x) as f32;
							let dy = (coords.y - last_y) as f32;
							last_mouse_pos.set((coords.x, coords.y));
							let msg = if evt.modifiers().shift() {
								MouseEvent { pan_x: dx, pan_y: dy, abs_x, abs_y, ..Default::default() }
							} else {
								MouseEvent { x: dx, y: dy, abs_x, abs_y, ..Default::default() }
							};
							let _ = camera_input.lock().unwrap().connection.send(&msg);
						}
					} else if is_lmb_down() {
						if let Some(camera_input) = camera_input.read().as_ref() {
							let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
								abs_x, abs_y, lmb_held: true, ..Default::default()
							});
						}
					}
				},
				onmouseup: move |evt| {
					let btn = evt.trigger_button();
					if btn == Some(dioxus::html::input_data::MouseButton::Auxiliary) {
						is_dragging.set(false);
					} else if btn == Some(dioxus::html::input_data::MouseButton::Primary) {
						is_lmb_down.set(false);
						let elem = evt.element_coordinates();
						let (abs_x, abs_y) = to_game_coords(elem.x, elem.y, canvas_wh());
						if let Some(camera_input) = camera_input.read().as_ref() {
							let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
								abs_x, abs_y, lmb_released: true, ..Default::default()
							});
						}
					}
				},
				onwheel: move |evt| {
					evt.prevent_default();
					if let Some(camera_input) = camera_input.read().as_ref() {
						let scroll = match evt.delta() {
							dioxus::html::geometry::WheelDelta::Lines(l) => l.y as f32,
							dioxus::html::geometry::WheelDelta::Pages(p) => p.y as f32 * 10.0,
							dioxus::html::geometry::WheelDelta::Pixels(p) => p.y as f32 * 0.05,
						};
						let _ = camera_input.lock().unwrap().connection.send(&MouseEvent {
							scroll, ..Default::default()
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
