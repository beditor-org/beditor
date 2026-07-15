use crate::event::Events;

#[derive(Clone)]
pub struct MenuBarGroupConfig {
	pub label: &'static str,
	pub items: Vec<MenuBarItemConfig>,
}

impl PartialEq for MenuBarGroupConfig {
	fn eq(&self, other: &Self) -> bool {
		self.label == other.label && self.items == other.items
	}
}

impl Eq for MenuBarGroupConfig {}

#[derive(Clone, Default)]
pub struct MenuBarItemConfig {
	pub action: Option<fn(&Events, &str)>,
	pub label: String,
	pub value: String,
	pub disabled: bool,
}

impl PartialEq for MenuBarItemConfig {
	fn eq(&self, other: &Self) -> bool {
		self.label == other.label && self.value == other.value && self.disabled == other.disabled
	}
}

impl Eq for MenuBarItemConfig {}
