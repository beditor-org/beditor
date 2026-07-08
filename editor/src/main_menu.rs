#[derive(Clone, PartialEq, Eq)]
pub struct MenuBarGroupConfig {
	pub label: &'static str,
	pub items: Vec<MenuBarItemConfig>,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct MenuBarItemConfig {
	// pub action: Box<dyn EditorEvent>,
	pub label: &'static str,
	pub disabled: bool,
}
