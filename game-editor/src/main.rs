mod app;
mod config;

use app::App;
use config::Config;

fn main() {
	let config = Config::default();
	vitronix::runner::run(vitronix::config::Config {
		window: vitronix::config::WindowConfig {
			title: config.title.to_string(),
			window_type: vitronix::config::WindowType::Sized {
				width: 800.0,
				height: 600.0,
				position: None,
				resizable: false,
			},
			..Default::default()
		},
		startup: Some(App),
		initial_theme: vitronix::theme::Theme::Dark,
	});
}
