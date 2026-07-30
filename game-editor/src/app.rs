use dioxus::{desktop::use_window, prelude::*};
use vitronix::{components::app::CustomStartupFinished, window::use_drag_window};

#[component]
pub fn App() -> Element {
	let done = use_context::<CustomStartupFinished>();
	let window = use_window();
	use_effect(move || {
		// Tell the WM this is a splash window — tiling WMs (i3, etc.) auto-float splash windows.
		// Must be set before set_visible so the hint is read on MapNotify.
		#[cfg(target_os = "linux")]
		vitronix::window::set_floatable(&window.window);
		vitronix::window::align_center(&window.window, 200., 100.);
	});

	let mut done_done = done.clone();
	rsx! {
		style { {include_str!("../public/main.css")} }
		div {
			onmousedown: use_drag_window(),
			class: "w-screen h-screen",
			button {
				onmousedown: |e| e.stop_propagation(),
				class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
				onclick: move |_| {
					done_done.0.set(true);
				},
				"Finish Startup"
			 }
		}
	}
}
