use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
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
}

pub fn use_theme() -> Signal<Theme> {
	use_context::<Signal<Theme>>()
}

pub fn init_theme() -> Signal<Theme> {
	use_context_provider(|| Signal::new(Theme::Dark))
}
