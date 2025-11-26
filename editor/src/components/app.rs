use std::sync::{Arc, RwLock};

use dioxus::{core::Element, desktop::wry::dpi::PhysicalPosition, prelude::*};

use crate::{
	components::EditorLayout, editor::get_window_position, panel::PanelsManager, EditorContext, PluginRegistry, PluginsManager,
};

#[component]
pub fn App() -> Element {
	let state = use_context::<Arc<RwLock<EditorContext>>>();
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	use_context_provider(|| Signal::new(Into::<PluginsManager>::into(plugins_registry.as_ref())));

	// Create panels signal and provide it to children
	// let initial_panels = state.read().map(|s| s.panels.clone()).unwrap_or_default();
	// let panels_signal = use_context_provider(|| Signal::new(initial_panels));
	// use_context_provider(|| Signal::new(manager.read().unwrap().plugin_states.clone()));

	let game_spawned = use_signal(|| false);

	let state_for_effect = state.clone();
	use_effect(move || {
		if game_spawned() {
			return;
		}

		let state_for_spawn = state_for_effect.clone();
		spawn(async move {
			// Minimum delay so that UI has time to render
			tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

			let window = &dioxus::desktop::window().window;
			let size = window.inner_size();

			let window_position = get_window_position(window.as_ref())
				.map(|(x, y)| PhysicalPosition::new(x, y))
				.unwrap_or_else(|| PhysicalPosition::new(0, 0));

			println!("✓ Window position: ({}, {})", window_position.x, window_position.y);
			println!("✓ Window size: {}x{}", size.width, size.height);

			let top_bar_height = 100;
			let left_panel_width = 300;
			let right_panel_width = 350;

			let viewport_x = window_position.x + left_panel_width as i32;
			let viewport_y = window_position.y + top_bar_height as i32 + 2;
			let viewport_width = size.width.saturating_sub(left_panel_width + right_panel_width);
			let viewport_height = size.height.saturating_sub(top_bar_height + 35);
			println!("  Viewport screen position: {viewport_width}x{viewport_height} at ({viewport_x}, {viewport_y})");

			// spawn_bevy_game_borderless(
			// 	viewport_x as i32,
			// 	viewport_y as i32,
			// 	viewport_width,
			// 	viewport_height,
			// 	state_for_spawn,
			// );
			// game_spawned.set(true);
		});
	});
	// let _s = asset!("/assets/main.css");
	rsx! {
		style { {include_str!("../../public/main.css")} }
		EditorLayout {}
	}
}
