use dioxus::{
	desktop::{tao::window::Window, window as desktop_window},
	prelude::*,
};
use std::sync::Arc;

/// Returns a closure that starts dragging the OS window when called.
/// Attach it to `onmousedown` of any element you want to use as a drag handle.
pub fn use_drag_window() -> impl FnMut(Event<MouseData>) {
	move |_| {
		desktop_window().window.drag_window().ok();
	}
}

/// Centers the window on the primary monitor given its logical size.
pub fn align_center(window: &Arc<Window>, width: f64, height: f64) {
	debug!("aligning window to center of primary monitor with size {}x{}", width, height);
	use dioxus::desktop::tao::dpi::LogicalPosition;
	if let Some(monitor) = window.current_monitor().or_else(|| window.primary_monitor()) {
		let scale = window.scale_factor();
		let m = monitor.size().to_logical::<f64>(scale);
		let m_pos = monitor.position().to_logical::<f64>(scale);
		window.set_outer_position(LogicalPosition::new(
			m_pos.x + (m.width - width) / 2.0,
			m_pos.y + (m.height - height) / 2.0,
		));
		window.set_inner_size(dioxus::desktop::LogicalSize::new(width, height));
	}
}

/// Sets `_NET_WM_WINDOW_TYPE_SPLASH` on the GTK window before it is shown.
/// i3wm (and most tiling WMs) automatically float splash-type windows,
/// so no IPC commands or delays are needed.
/// Must be called before `set_visible(true)`.
#[cfg(target_os = "linux")]
pub fn set_floatable(window: &Arc<Window>) {
	use dioxus::desktop::tao::platform::unix::WindowExtUnix;
	use gtk::prelude::*;
	let gtk_win = window.gtk_window();
	gtk_win.set_type_hint(gtk::gdk::WindowTypeHint::Splashscreen);
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
