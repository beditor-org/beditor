use strum::Display;

use crate::{
	plugins::{core::CorePluginPanel, dumy::dumy::Dumy, CorePlugin},
	tool::ToolPlacement,
	PanelConfig, PanelDisplayMode, Plugin, Tool, ToolAlignment,
};

#[derive(Display)]
pub enum DumyPluginPanel {
	#[strum(to_string = "Status dumy bar")]
	StatusBar,
	#[strum(to_string = "Left dumy bar")]
	LeftBar,
}
pub struct DumyPlugin;

impl Plugin for DumyPlugin {
	fn get_name(&self) -> String {
		"Dumy Plugin".to_string()
	}

	fn get_description(&self) -> String {
		"Dumy plugin for testing purposes".to_string()
	}

	fn get_panels(&self) -> Vec<PanelConfig> {
		vec![PanelConfig {
			alignment: crate::panel::PanelAligment::Left,
			name: DumyPluginPanel::LeftBar.to_string(),
			display_mode: PanelDisplayMode::Tabbed,
		}]
	}

	fn get_tools(&self) -> Vec<Tool> {
		vec![
			Tool {
				placement: ToolPlacement::PanelByName(DumyPluginPanel::LeftBar.to_string()),
				name: "Dumy tool".to_string(),
				component: Dumy,
				alignment: ToolAlignment::default(),
			},
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::StatusBar.to_string()),
				name: "Dumy tool".to_string(),
				component: Dumy,
				alignment: ToolAlignment::End,
			},
		]
	}
}
