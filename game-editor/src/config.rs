pub struct Config {
	pub title: &'static str,
}

impl Default for Config {
	fn default() -> Self {
		Self { title: "Game Editor" }
	}
}

impl Config {
	pub fn title_with_version(&self) -> String {
		format!("{} v{}", self.title, env!("CARGO_PKG_VERSION"))
	}
}
