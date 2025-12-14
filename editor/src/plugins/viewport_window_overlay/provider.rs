use crate::viewport::{CpuOverhead, PerformanceInfo, ViewportConfig, ViewportProvider};
use anyhow::Result;
use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

/// Legacy window overlay provider
/// Spawns separate game window positioned over viewport element
/// Has known issues with tiling WMs and window synchronization
pub struct WindowOverlayProvider {
	config: Option<ViewportConfig>,
	is_running: Arc<Mutex<bool>>,
}

impl WindowOverlayProvider {
	pub fn new() -> Self {
		Self {
			config: None,
			is_running: Arc::new(Mutex::new(false)),
		}
	}

	fn component_impl() -> Element {
		// CRITICAL: use_hook to run ONLY ONCE on mount
		use_hook(|| {
			// This provider uses external window, so component is just placeholder
			println!("📺 Window overlay viewport active");
		});

		rsx! {
			div {
				id: "viewport-placeholder",
				class: "w-full h-full bg-gray-900 flex items-center justify-center text-gray-500",

				div {
					class: "text-center",
					p { class: "text-xl mb-2", "🪟 Separate Window Mode" }
					p { class: "text-sm", "Game renders in overlay window" }
					p { class: "text-xs mt-4 text-yellow-500",
						"⚠️ May have issues with tiling window managers"
					}
				}
			}
		}
	}
}

impl ViewportProvider for WindowOverlayProvider {
	fn id(&self) -> &'static str {
		"window_overlay"
	}

	fn name(&self) -> &str {
		"Window Overlay (Legacy)"
	}

	fn description(&self) -> &str {
		"Spawns separate game window. Known issues with tiling WMs. Not recommended."
	}

	fn is_supported(&self) -> bool {
		// Works everywhere but not recommended
		true
	}

	fn expected_performance(&self) -> PerformanceInfo {
		PerformanceInfo {
			estimated_fps: 144,
			latency_ms: 2.0,
			cpu_overhead: CpuOverhead::Low,
			memory_overhead_mb: 10,
		}
	}

	fn initialize(&mut self, config: ViewportConfig) -> Result<()> {
		println!("🚀 Initializing Window Overlay viewport provider");
		println!("   ⚠️  WARNING: This method has known issues with tiling WMs");

		self.config = Some(config);
		*self.is_running.lock().unwrap() = true;

		// TODO: Spawn game process with window positioning
		// TODO: Setup window synchronization

		Ok(())
	}

	fn shutdown(&mut self) -> Result<()> {
		println!("🛑 Shutting down Window Overlay viewport provider");
		*self.is_running.lock().unwrap() = false;
		self.config = None;

		// TODO: Kill game process

		Ok(())
	}

	fn get_component(&self) -> fn() -> Element {
		Self::component_impl
	}

	fn is_connected(&self) -> bool {
		*self.is_running.lock().unwrap()
	}
}
