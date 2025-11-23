use std::sync::{Arc, RwLock};

use dioxus::{core::Element, desktop::wry::dpi::PhysicalPosition, prelude::*};

use crate::{
	components::{top_bar::TopBar, LeftPanel, RightPanel},
	editor::get_window_position,
};

#[component]
pub fn App<S: 'static>() -> Element {
	let state = use_context::<Arc<RwLock<S>>>();

	let mut game_spawned = use_signal(|| false);

	use_effect(move || {
		if game_spawned() {
			return;
		}

		let state_for_spawn = state.clone();
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
			println!(
				"  Viewport screen position: {}x{} at ({}, {})",
				viewport_width, viewport_height, viewport_x, viewport_y
			);

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

	rsx! {
		style { {include_str!("../../assets/editor.css")} }

		div {
			class: "editor-container",
			style: "display: flex; flex-direction: column; height: 100vh; width: 100vw;",

			// Top Bar
			div {
				class: "top-bar",
				style: "height: 100px; background: #2d2d2d; border-bottom: 1px solid #1e1e1e;",
				TopBar {}
			}

			// Main content area
			div {
				class: "main-content",
				style: "display: flex; flex: 1; overflow: hidden;",

				// Left Panel (Hierarchy)
				div {
					class: "left-panel",
					style: "width: 300px; background: #252526; border-right: 1px solid #1e1e1e; overflow-y: auto;",
					LeftPanel<S> {}
				}

				// Center Viewport (placeholder for Bevy game)
				div {
					class: "viewport",
					style: "flex: 1; background: #1e1e1e; display: flex; align-items: center; justify-content: center; color: #888;",
					"🎮 Game Viewport (Bevy window will overlay here)"
				}

				// Right Panel (Inspector)
				div {
					class: "right-panel",
					style: "width: 350px; background: #252526; border-left: 1px solid #1e1e1e; overflow-y: auto;",
					RightPanel {}
				}
			}
		}
	}
}
