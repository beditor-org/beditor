use std::sync::{Arc, Mutex};

use bridge::protocol::frame_stream::FrameStreamProtocol;
use dioxus::{document::eval, prelude::*};
use tracing::info;

use crate::plugin::viewport::plugin::ViewportState;

pub fn Viewport() -> Element {
	// ALL HOOKS AT THE TOP - ALWAYS CALLED
	let mut viewport_state = use_context::<Signal<ViewportState>>();
	let frame = use_signal(|| None::<String>);
	let protocol_signal = use_context::<Signal<Option<Arc<Mutex<FrameStreamProtocol>>>>>();
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

	// Hook 3: Canvas renderer - simple Image decode (hardware accelerated)
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

	// Render - no hooks here
	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900 overflow-hidden flex items-center justify-center",
			canvas {
				id: "{canvas_id}",
				class: "max-w-full max-h-full object-contain",
				style: "image-rendering: auto;"
			}
		}
	}
}
