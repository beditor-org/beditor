use crate::viewport::{CpuOverhead, PerformanceInfo, ViewportConfig, ViewportProvider};
use crate::GameProcessManager;
use anyhow::Result;
use dioxus::prelude::*;
use std::sync::{atomic::AtomicU64, Arc, Mutex};

/// Custom protocol handler viewport provider
/// Works by serving frames through beditor:// protocol
/// Most cross-platform and reliable method
pub struct CustomProtocolProvider {
	config: Option<ViewportConfig>,
	frame_counter: Arc<AtomicU64>,
	is_running: Arc<Mutex<bool>>,
	game_process: Option<Arc<Mutex<GameProcessManager>>>,
}

impl CustomProtocolProvider {
	pub fn new() -> Self {
		Self {
			config: None,
			frame_counter: Arc::new(AtomicU64::new(0)),
			is_running: Arc::new(Mutex::new(false)),
			game_process: None,
		}
	}

	fn component_impl() -> Element {
		use crate::ViewportProtocolState;

		// Get protocol state to read frame counter
		let protocol_state = use_context::<Signal<ViewportProtocolState>>();
		// let protocol_state_for_game = protocol_state.clone();
		// let protocol_state_for_test = protocol_state.clone();
		// let protocol_state_for_canvas = protocol_state.clone();
		// let protocol_state_for_refresh = protocol_state.clone();

		// Signal to track frame counter and last rendered frame
		// let mut frame_counter = use_signal(|| 0u64);
		// let mut last_rendered_frame = use_signal(|| 0u64);

		eprintln!("🚀 CustomProtocolProvider component mounted");

		// Auto-refresh canvas when new frames arrive
		// use_effect(move || {
		// 	eprintln!("🔄 use_effect started, spawning auto-refresh loop");
		// 	let protocol_state_check = protocol_state_for_canvas.clone();
		// 	let mut last_rendered = last_rendered_frame.clone();
		// 	let mut frame_cnt = frame_counter.clone();

		// 	spawn(async move {
		// 		loop {
		// 			tokio::time::sleep(std::time::Duration::from_millis(100)).await;

		// 			let current_count = protocol_state_check.frame_counter();
		// 			eprintln!("🔍 Auto-refresh check: current={}, last={}", current_count, last_rendered());

		// 			if current_count != last_rendered() && current_count > 0 {
		// 				if let Some(frame_bytes) = protocol_state_check.get_frame() {
		// 					use base64::{engine::general_purpose, Engine as _};
		// 					let base64_data = general_purpose::STANDARD.encode(&frame_bytes);
		// 					eprintln!(
		// 						"🎨 Rendering frame {} to canvas (base64 len: {})",
		// 						current_count,
		// 						base64_data.len()
		// 					);

		// 					let js = format!(
		// 						r#"
		// 						console.log('🎨 JS: Rendering frame to canvas');
		// 						const canvas = document.getElementById('viewport-canvas');
		// 						console.log('🎨 JS: Canvas found:', canvas);
		// 						if (canvas) {{
		// 							const ctx = canvas.getContext('2d');
		// 							const img = new Image();
		// 							img.onload = () => {{
		// 								console.log('🎨 JS: Image loaded, size:', img.width, 'x', img.height);
		// 								canvas.width = img.width;
		// 								canvas.height = img.height;
		// 								ctx.drawImage(img, 0, 0);
		// 							}};
		// 							img.onerror = (e) => console.error('🎨 JS: Image load error:', e);
		// 							img.src = 'data:image/png;base64,{}';
		// 						}} else {{
		// 							console.error('🎨 JS: Canvas not found!');
		// 						}}
		// 					"#,
		// 						base64_data
		// 					);

		// 					dioxus::document::eval(&js);
		// 					last_rendered.set(current_count);
		// 					frame_cnt.set(current_count);
		// 				}
		// 			}
		// 		}
		// 	});
		// });

		// Function to refresh canvas from latest frame (for manual button)
		// let mut refresh_canvas = move || {
		// 	let current_count = protocol_state_for_refresh.frame_counter();
		// 	if current_count != last_rendered_frame() && current_count > 0 {
		// 		if let Some(frame_bytes) = protocol_state_for_refresh.get_frame() {
		// 			use base64::{engine::general_purpose, Engine as _};
		// 			let base64_data = general_purpose::STANDARD.encode(&frame_bytes);

		// 			let js = format!(
		// 				r#"
		// 				const canvas = document.getElementById('viewport-canvas');
		// 				if (canvas) {{
		// 					const ctx = canvas.getContext('2d');
		// 					const img = new Image();
		// 					img.onload = () => {{
		// 						canvas.width = img.width;
		// 						canvas.height = img.height;
		// 						ctx.drawImage(img, 0, 0);
		// 					}};
		// 					img.src = 'data:image/png;base64,{}';
		// 				}}
		// 			"#,
		// 				base64_data
		// 			);

		// 			dioxus::document::eval(&js);
		// 			last_rendered_frame.set(current_count);
		// 			frame_counter.set(current_count);
		// 		}
		// 	}
		// };

		rsx! {
			// div {
			// 	id: "viewport-container",
			// 	class: "w-full h-full bg-black relative overflow-hidden",

			// 	// Canvas element for rendering frames
			// 	canvas {
			// 		id: "viewport-canvas",
			// 		class: "absolute inset-0 w-full h-full object-contain z-0",
			// 		style: "pointer-events: none;",
			// 	}

			// 	// Control buttons - moved to top for visibility
			// 	div {
			// 		class: "absolute top-12 left-4 flex flex-col gap-2 z-50",

			// 		// Start game button
			// 		button {
			// 			class: "px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded font-bold text-lg shadow-lg",
			// 			onclick: move |_| {
			// 				use crate::GameProcessManager;
			// 				let state = protocol_state_for_game.clone();

			// 				// Start game in background thread
			// 					let mut manager = GameProcessManager::new(protocol_state);
			// 					if let Err(e) = manager.start("target/release/examples/demo") {
			// 						eprintln!("❌ Failed to start game: {}", e);
			// 					} else {
			// 						eprintln!("✅ Game started successfully");
			// 						// Keep manager alive - it will auto-stop on drop
			// 						std::thread::park();
			// 					}
			// 			},
			// 			"▶ Start Game"
			// 		}

			// 		// Refresh canvas button
			// 		button {
			// 			class: "px-6 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded font-bold text-lg shadow-lg",
			// 			onclick: move |_| {
			// 				refresh_canvas();
			// 				eprintln!("🔄 Canvas refreshed, frame: {}", frame_counter());
			// 			},
			// 			"🔄 Refresh"
			// 		}

			// 		// // Test button to update frame
			// 		// button {
			// 		// 	class: "px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded font-bold text-lg shadow-lg",
			// 		// 	onclick: move |_| {
			// 		// 		// Update frame with new placeholder
			// 		// 		protocol_state_for_test.update_frame(ViewportProtocolState::generate_placeholder_frame());
			// 		// 		eprintln!("🔵 Test frame generated");
			// 		// 	},
			// 		// 	"📸 Test Frame"
			// 		// }
			// 	}

			// 	// Frame counter display
			// 	div {
			// 		class: "absolute top-4 left-4 px-3 py-1 bg-black bg-opacity-75 text-white text-sm rounded z-10",
			// 		"Frame: {frame_counter}"
			// 	}
			// }
		}
	}
}

// impl ViewportProvider for CustomProtocolProvider {
// 	fn id(&self) -> &'static str {
// 		"custom_protocol"
// 	}

// 	fn name(&self) -> &str {
// 		"Custom Protocol (Recommended)"
// 	}

// 	fn description(&self) -> &str {
// 		"Streams frames via custom beditor:// protocol. Cross-platform, reliable, good performance."
// 	}

// 	fn is_supported(&self) -> bool {
// 		// Works on all platforms with wry
// 		true
// 	}

// 	fn expected_performance(&self) -> PerformanceInfo {
// 		PerformanceInfo {
// 			estimated_fps: 60,
// 			latency_ms: 15.0,
// 			cpu_overhead: CpuOverhead::Low,
// 			memory_overhead_mb: 50,
// 		}
// 	}

// 	fn initialize(&mut self, config: ViewportConfig) -> Result<()> {
// 		println!("🚀 Initializing Custom Protocol viewport provider");
// 		println!("   Resolution: {}x{}", config.width, config.height);
// 		println!("   Target FPS: {}", config.target_fps);

// 		self.config = Some(config.clone());
// 		*self.is_running.lock().unwrap() = true;

// 		println!("⚠️  Game process will be started manually via UI");
// 		println!("   Game path: {}", config.game_executable_path);

// 		Ok(())
// 	}

// 	fn shutdown(&mut self) -> Result<()> {
// 		println!("🛑 Shutting down Custom Protocol viewport provider");
// 		*self.is_running.lock().unwrap() = false;
// 		self.config = None;
// 		Ok(())
// 	}

// 	fn get_component(&self) -> fn() -> Element {
// 		Self::component_impl
// 	}

// 	fn is_connected(&self) -> bool {
// 		*self.is_running.lock().unwrap()
// 	}
// }
