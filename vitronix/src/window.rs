use dioxus::{desktop::window as desktop_window, prelude::*};

/// Maximizes the window when the component mounts.
///
/// On i3wm, floating windows ignore `_NET_WM_STATE_MAXIMIZED`.
/// This hook detects i3 via the `I3SOCK` env var and sends `floating disable`
/// via i3 IPC before maximizing, which moves the window into tiling mode first.
pub fn use_maximize() {
	let window = desktop_window();
	use_effect(move || {
		if std::env::var("I3SOCK").is_ok() {
			std::process::Command::new("i3-msg").arg("floating disable").spawn().ok();
		}
		window.window.set_maximized(true);
	});
}

/// Returns a closure that starts dragging the OS window when called.
/// Attach it to `onmousedown` of any element you want to use as a drag handle.
pub fn use_drag_window() -> impl FnMut(Event<MouseData>) {
	move |_| {
		desktop_window().window.drag_window().ok();
	}
}
