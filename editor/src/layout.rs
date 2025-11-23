use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelAlignment {
	Top,
	Bottom,
	Left,
	Right,
	Center,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PanelConfig {
	pub id: String,
	pub alignment: PanelAlignment,
	pub size: PanelSize,
	pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PanelSize {
	Fixed(u32),      // pixels
	Percentage(f32), // 0.0 - 1.0
	Auto,            // flex-based
}

impl PanelConfig {
	pub fn new(id: impl Into<String>, alignment: PanelAlignment) -> Self {
		Self {
			id: id.into(),
			alignment,
			size: PanelSize::Auto,
			visible: true,
		}
	}

	pub fn with_size(mut self, size: PanelSize) -> Self {
		self.size = size;
		self
	}

	pub fn size_style(&self) -> String {
		match self.size {
			PanelSize::Fixed(px) => format!("{}px", px),
			PanelSize::Percentage(p) => format!("{}%", p * 100.0),
			PanelSize::Auto => "auto".to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
	pub panels: Vec<PanelConfig>,
}

impl Default for LayoutConfig {
	fn default() -> Self {
		Self {
			panels: vec![
				PanelConfig::new("top-bar", PanelAlignment::Top).with_size(PanelSize::Fixed(40)),
				PanelConfig::new("left-panel", PanelAlignment::Left).with_size(PanelSize::Fixed(300)),
				PanelConfig::new("right-panel", PanelAlignment::Right).with_size(PanelSize::Fixed(350)),
				PanelConfig::new("center-panel", PanelAlignment::Center),
			],
		}
	}
}

impl LayoutConfig {
	pub fn panels_by_alignment(&self, alignment: PanelAlignment) -> Vec<&PanelConfig> {
		self.panels.iter().filter(|p| p.visible && p.alignment == alignment).collect()
	}
}
