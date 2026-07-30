use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Theme {
	#[default]
	Dark,
	Light,
}

impl Theme {
	pub fn as_str(&self) -> &'static str {
		match self {
			Theme::Dark => "dark",
			Theme::Light => "light",
		}
	}

	/// Corresponds to --color-bg-primary from the theme CSS files.
	pub fn background_rgb(&self) -> (u8, u8, u8) {
		match self {
			Theme::Dark => (30, 30, 30),     // #1e1e1e
			Theme::Light => (255, 255, 255), // #ffffff
		}
	}
}

pub fn use_theme() -> Signal<Theme> {
	use_context::<Signal<Theme>>()
}

pub fn use_init_theme() -> Signal<Theme> {
	use_context_provider(|| Signal::new(Theme::Dark))
}
