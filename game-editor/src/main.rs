mod app;
mod config;

use app::App;
use config::Config;

fn main() {
	let config = Config::default();
	vitronix::runner::run(vitronix::config::Config {
		window: vitronix::config::WindowConfig {
			title: config.title_with_version(),
			resizable: false,
			size: Some((800.0, 600.0)),
			..Default::default()
		},
		app: App,
	});
}
