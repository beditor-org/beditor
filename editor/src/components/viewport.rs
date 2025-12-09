use crate::ViewportProtocolState;
use base64::{engine::general_purpose, Engine};
use dioxus::prelude::*;

/// Viewport component: renders frames from game to canvas
#[component]
pub fn Viewport() -> Element {
	let protocol_state = use_context::<Signal<ViewportProtocolState>>();
	let frame_count = protocol_state.read().frame_counter;

	use_effect(move || {
		let state = protocol_state.read();

		if let Some(frame_bytes) = state.get_frame() {
			let base64_data = general_purpose::STANDARD.encode(&frame_bytes);
			let js = format!(
				r#"
				const canvas = document.getElementById('viewport-canvas');
				if (!canvas) return;
				
				// Reuse image or create if needed
				if (!window.__viewportImg) {{
					window.__viewportImg = new Image();
					window.__viewportImg.onload = function() {{
						const ctx = canvas.getContext('2d');
						canvas.width = this.width;
						canvas.height = this.height;
						ctx.drawImage(this, 0, 0);
					}};
				}}
				
				window.__viewportImg.src = 'data:image/png;base64,{}';
			"#,
				base64_data
			);
			dioxus::document::eval(&js);
		}
	});

	rsx! {
		div {
			class: "relative w-full h-full bg-gray-900",

			canvas {
				id: "viewport-canvas",
				class: "absolute inset-0 w-full h-full object-contain",
			}

			div {
				class: "absolute top-2 left-2 px-2 py-1 bg-black bg-opacity-70 text-white text-xs rounded",
				"Frame: {frame_count}"
			}
		}
	}
}
