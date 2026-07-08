use crate::event::Events;

#[derive(Clone, PartialEq, Eq)]
pub struct MenuBarGroupConfig {
	pub label: &'static str,
	pub items: Vec<MenuBarItemConfig>,
}

#[derive(Clone, Default)]
pub struct MenuBarItemConfig {
	pub action: Option<fn(&Events)>,
	pub label: &'static str,
	pub disabled: bool,
}

impl PartialEq for MenuBarItemConfig {
	fn eq(&self, other: &Self) -> bool {
		self.label == other.label && self.disabled == other.disabled
	}
}

impl Eq for MenuBarItemConfig {}
