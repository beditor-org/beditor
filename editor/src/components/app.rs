use std::sync::Arc;

use dioxus::{core::Element, prelude::*};

use crate::{components::EditorLayout, init_theme, PluginRegistry, PluginsManager, ViewportProtocolState};

#[component]
pub fn App() -> Element {
	let plugins_registry = use_context::<Arc<PluginRegistry>>();
	use_context_provider(|| Signal::new(Into::<PluginsManager>::into(plugins_registry.as_ref())));

	let protocol_state = ViewportProtocolState::new();
	use_context_provider(|| Signal::new(Into::<ViewportProtocolState>::into(protocol_state)));
	// Initialize theme
	init_theme();

	// Create panels signal and provide it to children
	// let initial_panels = state.read().map(|s| s.panels.clone()).unwrap_or_default();
	// let panels_signal = use_context_provider(|| Signal::new(initial_panels));
	// use_context_provider(|| Signal::new(manager.read().unwrap().plugin_states.clone()));

	// LEGACY: Window Overlay spawn code - disabled when using Custom Protocol or Shared Memory
	// This should only be called by WindowOverlayProvider when it's active
	// let mut game_spawned = use_signal(|| false);
	// let state_for_effect = state.clone();
	// use_effect(move || {
	// 	if game_spawned() {
	// 		return;
	// 	}
	// 	spawn(async move {
	// 		tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
	// 		let window = &dioxus::desktop::window().window;
	// 		let window_pos_x11 = get_window_position(window.as_ref()).unwrap_or((0, 0));
	// 		let window_pos_js = get_window_position_js().await.unwrap_or((0, 0));
	// 		let wm_panel_offset_y = window_pos_js.1 - window_pos_x11.1;
	// 		if let Some((vp_x, vp_y, vp_width, vp_height)) = get_viewport_screen_bounds().await {
	// 			let screen_x = vp_x + window_pos_x11.0;
	// 			let screen_y = vp_y + window_pos_x11.1 - wm_panel_offset_y;
	// 			spawn_game_process(screen_x, screen_y, vp_width, vp_height);
	// 			game_spawned.set(true);
	// 		}
	// 	});
	// });
	// let _s = asset!("/assets/main.css");
	rsx! {
		style { {include_str!("../../public/main.css")} }
		EditorLayout {}
	}
}
