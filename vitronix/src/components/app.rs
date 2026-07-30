use std::ops::Deref;

use crate::components::layout::Layout;
use dioxus::desktop::{use_window, window as desktop_window, LogicalPosition};
use dioxus::{core::Element, prelude::*};

use crate::config::Config;

#[derive(Clone)]
pub struct CustomStartupFinished(pub Signal<bool>);

impl Deref for CustomStartupFinished {
	type Target = Signal<bool>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[component]
pub fn App() -> Element {
	info!("rendering App component");
	let config: Config = use_context::<Config>();
	let custom_startup_finished = use_context_provider(|| CustomStartupFinished(Signal::new(config.startup.is_none())));
	use_effect(move || {
		let ctx = use_window();
		let win = &ctx.window;

		// 	let should_center = config.window.size.is_some()
		// 		&& config.window.position.is_none()
		// 		&& !config.window.resizable
		// 		&& !config.window.fullscreen;

		// 	if should_center {
		// 		let (win_w, win_h) = config.window.size.unwrap();
		// 		if let Some(monitor) = win.primary_monitor() {
		// 			let scale = win.scale_factor();
		// 			let m = monitor.size().to_logical::<f64>(scale);
		// 			let m_pos = monitor.position().to_logical::<f64>(scale);

		// 			let x = m_pos.x + (m.width - win_w as f64) / 2.0;
		// 			let y = m_pos.y + (m.height - win_h as f64) / 2.0;

		// 			win.set_outer_position(LogicalPosition::new(x, y));
		// 		}
		// 	}

		#[cfg(target_os = "linux")]
		{
			use crate::window::set_gtk_background_color;
			let (r, g, b) = config.initial_theme.background_rgb();
			set_gtk_background_color(r, g, b, win.clone());
		}

		win.set_visible(true);
	});
	// let plugins = use_context::<Vec<fn() -> Plugin>>();
	// let config = use_context::<EditorConfig>();
	// let events = use_context_provider(Events::new);
	// use_context_provider(|| Signal::new(config));

	// let registry = use_context_provider(|| Signal::new(Into::<PluginRegistry>::into(plugins)));

	// // Initialize WorkspaceRegistry from plugins BEFORE calling entry() functions
	// let workspaces = WorkspaceRegistry::from_plugins(&registry.read());
	// let workspace_registry = use_context_provider(|| Signal::new(workspaces));

	// // Subscribe to workspace switch events
	// use_effect(move || {
	// 	let events = events.clone();
	// 	let mut workspace_registry = workspace_registry.clone();
	// 	events.subscribe::<SwitchWorkspaceEvent>(move |event| {
	// 		workspace_registry.write().set_current(event.0.clone());
	// 	});
	// });

	// use_init_theme();

	let all_initialised = use_hook(|| false);
	// let all_initialised = all_initialised.get();
	// let all_initialised = use_memo(move || {
	// 	registry
	// 		.read()
	// 		.plugins
	// 		.values()
	// 		.all(|plugin| plugin.entry.is_none() || plugin.is_initialized)
	// });
	// info!("Plugins all_initialised: {all_initialised}");
	// let plugins = registry.read().plugins.clone();
	rsx! {
		// Phase 1: Init contexts for all plugins
		// for (_, plugin) in &plugins {
		// 	if let Some(setup_context) = &plugin.setup_context {
		// 		{setup_context()}
		// 	}
		// }

		// // Phase 2: All initialize plugins
		// for (_, plugin) in &plugins {
		// 	if let Some(entry) = &plugin.entry {
		// 		{entry()}
		// 	}
		// }

		style { {include_str!("../../public/main.css")} }
		if let Some(Startup) = config.startup {
			if !*custom_startup_finished.read() {
				Startup {}
			} else {
				Layout {}
			}
		} else {
			Layout {}
		}
	}
}
