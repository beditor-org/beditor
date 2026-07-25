use dioxus::{
	desktop::{tao::window::Window, window as desktop_window},
	prelude::*,
};
use std::sync::Arc;

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

// On Linux, set the GTK window background via CSS provider so that
// the X11 Expose event paints the correct color instead of white.
#[cfg(target_os = "linux")]
pub fn set_gtk_background_color(r: u8, g: u8, b: u8, window: Arc<Window>) {
	use dioxus::desktop::tao::platform::unix::WindowExtUnix;
	use gtk::prelude::*;
	let css = gtk::CssProvider::new();
	let _ = css.load_from_data(format!("window, widget {{ background: rgb({r},{g},{b}); }}").as_bytes());
	let gtk_win = window.gtk_window();
	gtk_win
		.style_context()
		.add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}
