use anyhow::Result;
use dioxus::prelude::*;
use std::sync::{Arc, RwLock};

/// Trait for viewport rendering providers
/// Each implementation represents a different method of streaming game frames
pub trait ViewportProvider: Send + Sync {
	/// Unique identifier for this provider
	fn id(&self) -> &'static str;

	/// Human-readable name
	fn name(&self) -> &str;

	/// Description of how this provider works
	fn description(&self) -> &str;

	/// Platform compatibility check
	fn is_supported(&self) -> bool;

	/// Performance characteristics (estimated fps capability)
	fn expected_performance(&self) -> PerformanceInfo;

	/// Initialize the provider (setup shared memory, sockets, etc)
	fn initialize(&mut self, config: ViewportConfig) -> Result<()>;

	/// Shutdown and cleanup resources
	fn shutdown(&mut self) -> Result<()>;

	/// Get the Dioxus component that renders the viewport
	/// This will be different per provider (canvas, img, iframe, etc)
	fn get_component(&self) -> fn() -> Element;

	/// Optional: called every frame to update state
	fn update(&mut self) -> Result<()> {
		Ok(())
	}

	/// Get current connection status
	fn is_connected(&self) -> bool {
		false
	}
}

#[derive(Clone, Debug)]
pub struct ViewportConfig {
	pub width: u32,
	pub height: u32,
	pub game_executable_path: String,
	pub target_fps: u32,
}

impl Default for ViewportConfig {
	fn default() -> Self {
		Self {
			width: 1920,
			height: 1080,
			game_executable_path: "../bevy_demo_game/target/release/bevy_demo_game".to_string(),
			target_fps: 60,
		}
	}
}

#[derive(Clone, Debug)]
pub struct PerformanceInfo {
	pub estimated_fps: u32,
	pub latency_ms: f32,
	pub cpu_overhead: CpuOverhead,
	pub memory_overhead_mb: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CpuOverhead {
	Low,    // < 5% CPU
	Medium, // 5-15% CPU
	High,   // > 15% CPU
}

/// Manager for viewport providers
pub struct ViewportManager {
	providers: Vec<Box<dyn ViewportProvider>>,
	active_provider: Option<usize>,
	config: ViewportConfig,
}

impl ViewportManager {
	pub fn new(config: ViewportConfig) -> Self {
		Self {
			providers: Vec::new(),
			active_provider: None,
			config,
		}
	}

	pub fn register_provider(&mut self, provider: Box<dyn ViewportProvider>) {
		self.providers.push(provider);
	}

	pub fn get_providers(&self) -> &[Box<dyn ViewportProvider>] {
		&self.providers
	}

	pub fn get_supported_providers(&self) -> Vec<&Box<dyn ViewportProvider>> {
		self.providers.iter().filter(|p| p.is_supported()).collect()
	}

	pub fn set_active_provider(&mut self, id: &str) -> Result<()> {
		// Shutdown current provider
		if let Some(idx) = self.active_provider {
			self.providers[idx].shutdown()?;
		}

		// Find and activate new provider
		let new_idx = self
			.providers
			.iter()
			.position(|p| p.id() == id)
			.ok_or_else(|| anyhow::anyhow!("Provider not found: {}", id))?;

		self.providers[new_idx].initialize(self.config.clone())?;
		self.active_provider = Some(new_idx);

		Ok(())
	}

	pub fn get_active_provider(&self) -> Option<&Box<dyn ViewportProvider>> {
		self.active_provider.map(|idx| &self.providers[idx])
	}

	pub fn get_active_provider_mut(&mut self) -> Option<&mut Box<dyn ViewportProvider>> {
		self.active_provider.map(|idx| &mut self.providers[idx])
	}

	pub fn update(&mut self) -> Result<()> {
		if let Some(provider) = self.get_active_provider_mut() {
			provider.update()?;
		}
		Ok(())
	}
}

/// Dioxus component that renders the active viewport
#[component]
pub fn Viewport() -> Element {
	let viewport_manager = use_context::<Arc<RwLock<ViewportManager>>>();
	let mut show_settings = use_signal(|| false);

	// CRITICAL: Extract component function BEFORE rendering, then drop lock
	// Prevents holding read() lock during render which would deadlock with write() in settings
	let component_fn = {
		let manager = viewport_manager.read().unwrap();
		let provider = manager.get_active_provider();
		eprintln!("🔍 Viewport rendering, active provider: {:?}", provider.map(|p| p.id()));
		provider.map(|p| p.get_component())
	}; // Lock dropped here

	eprintln!("🔍 component_fn is Some: {}", component_fn.is_some());

	rsx! {
		div {
			class: "relative w-full h-full",

			// Viewport content
			match component_fn {
				Some(component) => rsx! { {component()} },
				None => rsx! {
					div {
						class: "flex items-center justify-center h-full bg-gray-800 text-gray-400",
						"No viewport provider active"
					}
				}
			}

			// Settings button (top-right corner)
			button {
				class: "absolute top-2 right-2 px-3 py-1 bg-gray-800 bg-opacity-75 hover:bg-opacity-100 text-white rounded shadow-lg transition-all",
				onclick: move |_| show_settings.set(true),
				"⚙️ Settings"
			}

			// Settings modal
			if show_settings() {
				ViewportSettingsModal { show: show_settings }
			}
		}
	}
}

#[component]
pub fn ViewportSettingsModal(show: Signal<bool>) -> Element {
	rsx! {
		// Modal overlay
		div {
			class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
			onclick: move |_| show.set(false),

			// Modal content
			div {
				class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-2xl w-full m-4",
				onclick: move |e| e.stop_propagation(),

				// Header
				div {
					class: "flex items-center justify-between p-4 border-b dark:border-gray-700",
					h2 { class: "text-xl font-bold dark:text-white", "Viewport Settings" }
					button {
						class: "text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 text-2xl",
						onclick: move |_| show.set(false),
						"✕"
					}
				}

				// Content
				div {
					class: "p-4 max-h-96 overflow-y-auto",
					ViewportSettings {}
				}

				// Footer
				div {
					class: "flex justify-end p-4 border-t dark:border-gray-700",
					button {
						class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
						onclick: move |_| show.set(false),
						"Close"
					}
				}
			}
		}
	}
}

/// Settings UI for selecting viewport provider
#[component]
pub fn ViewportSettings() -> Element {
	let viewport_manager = use_context::<Arc<RwLock<ViewportManager>>>();

	// CRITICAL: Extract all data we need BEFORE rendering, then drop the lock
	// If we hold read() during render and onclick tries to write(), we get deadlock!
	let (active_id, active_name, providers_data) = {
		let manager = viewport_manager.read().unwrap();
		let active_id = manager.get_active_provider().map(|p| p.id());
		let active_name = manager.get_active_provider().map(|p| p.name().to_string());

		let providers_data: Vec<_> = manager
			.get_supported_providers()
			.iter()
			.map(|provider| {
				let perf = provider.expected_performance();
				(
					provider.id(),
					provider.name().to_string(),
					provider.description().to_string(),
					perf.estimated_fps,
					perf.latency_ms,
					format!("{:?}", perf.cpu_overhead),
				)
			})
			.collect();

		(active_id, active_name, providers_data)
	}; // Lock is dropped here!

	rsx! {
		div {
			class: "space-y-3",

			// Info about current provider
			if let Some(name) = active_name {
				div {
					class: "p-3 bg-blue-50 dark:bg-blue-900 rounded-lg mb-4",
					p { class: "text-sm font-semibold text-blue-800 dark:text-blue-200",
						"🟢 Active: {name}"
					}
				}
			}

			// List all available providers
			for (provider_id, provider_name, provider_desc, fps, latency, cpu) in providers_data {
				{
					let is_active = active_id == Some(provider_id);
					let manager_clone = viewport_manager.clone();

					rsx! {
						div {
							class: if is_active {
								"p-4 border-2 border-blue-500 rounded-lg bg-blue-50 dark:bg-blue-950"
							} else {
								"p-4 border border-gray-300 dark:border-gray-600 rounded-lg hover:border-gray-400 dark:hover:border-gray-500"
							},

							div {
								class: "flex items-start justify-between gap-4",
								div {
									class: "flex-1",
									div {
										class: "flex items-center gap-2 mb-1",
										h4 {
											class: "font-semibold text-lg dark:text-white",
											"{provider_name}"
										}
										if is_active {
											span {
												class: "px-2 py-0.5 text-xs bg-blue-500 text-white rounded-full",
												"Active"
											}
										}
									}
									p {
										class: "text-sm text-gray-600 dark:text-gray-400 mb-3",
										"{provider_desc}"
									}

									// Performance metrics
									div {
										class: "flex flex-wrap gap-3 text-xs",
										div {
											class: "flex items-center gap-1",
											span { class: "font-semibold dark:text-gray-300", "FPS:" }
											span { class: "text-gray-600 dark:text-gray-400", "~{fps}" }
										}
										div {
											class: "flex items-center gap-1",
											span { class: "font-semibold dark:text-gray-300", "Latency:" }
											span { class: "text-gray-600 dark:text-gray-400", "~{latency}ms" }
										}
										div {
											class: "flex items-center gap-1",
											span { class: "font-semibold dark:text-gray-300", "CPU:" }
											span { class: "text-gray-600 dark:text-gray-400", "{cpu}" }
										}
									}
								}

								// Select button
								if !is_active {
									button {
										class: "px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors whitespace-nowrap",
										onclick: move |_| {
											if let Err(e) = manager_clone.write().unwrap().set_active_provider(provider_id) {
												eprintln!("❌ Failed to activate provider: {}", e);
											} else {
												println!("✓ Switched to provider: {}", provider_id);
											}
										},
										"Switch to this"
									}
								} else {
									div {
										class: "px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-500 dark:text-gray-400 rounded-lg",
										"✓ In use"
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
