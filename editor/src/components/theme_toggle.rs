use dioxus::prelude::*;

use crate::{use_theme, Theme};

#[component]
pub fn ThemeToggle() -> Element {
	let mut theme = use_theme();

	rsx! {
		button {
			class: "px-3 py-1 hover:bg-editor-bg-secondary rounded border border-editor-border",
			onclick: move |_| {
				let new_theme = match theme() {
					Theme::Dark => Theme::Light,
					Theme::Light => Theme::Dark,
				};
				eprintln!("Switching theme to: {:?}", new_theme);
				theme.set(new_theme);

				// Apply theme to :root (html element)
				let theme_str = new_theme.as_str();
				let js = format!(r#"
					console.log('Applying theme:', '{}');
					document.documentElement.setAttribute('data-theme', '{}');
					document.body.setAttribute('data-theme', '{}');
					console.log('Current data-theme:', document.documentElement.getAttribute('data-theme'));
				"#, theme_str, theme_str, theme_str);

				// Execute JavaScript
				let _ = dioxus::document::eval(&js);
			},
			match theme() {
				Theme::Dark => "Dark",
				Theme::Light => "Light",
			}
		}
	}
}
