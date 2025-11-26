use std::{any::TypeId, collections::HashMap};

use dioxus::prelude::*;

use crate::{plugin::PluginState, tool::Tool};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PanelAligment {
	#[default]
	Left,
	Right,
	Top,
	Bottom,
	Center,
	CenterTop,
	CenterBottom,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanelConfig {
	pub name: String,
	pub alignment: PanelAligment,
}
#[derive(Clone, Debug, PartialEq)]
pub struct PanelState {
	pub alignment: PanelAligment,
	pub tools: Vec<Tool>,
	pub name: String,
	pub is_visible: bool,
	// pub is_open: bool,
	// pub title: String,
	// Additional fields can be added as needed
}

impl Default for PanelState {
	fn default() -> Self {
		Self {
			alignment: PanelAligment::Left,
			tools: Vec::new(),
			name: String::new(),
			is_visible: true,
		}
	}
}

#[derive(Clone, Default)]
pub struct PanelsManager {
	pub panels: Vec<PanelState>,
}

impl From<HashMap<TypeId, PluginState>> for PanelsManager {
	fn from(value: HashMap<TypeId, PluginState>) -> Self {
		Self { panels: Vec::new() }
	}
}

impl PanelsManager {
	pub fn add_panel(&mut self, panel: PanelState) {
		self.panels.push(panel);
	}

	pub fn get_panel_by_name(&mut self, name: &str) -> Option<&mut PanelState> {
		self.panels.iter_mut().find(|p| p.name == name)
	}
}

#[component]
pub fn Panel() -> Element {
	rsx! {
		div {
			// Panel content goes here
		}
	}
}
