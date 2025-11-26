use crate::{
	plugins::core::{MenuBar, StatusBar},
	tool::ToolPlacement,
	PanelConfig, Plugin, Tool,
};
use strum::Display;

pub struct CorePlugin;

#[derive(Display)]
pub enum CorePluginPanel {
	#[strum(to_string = "Top bar")]
	TopBar,
	#[strum(to_string = "Status bar")]
	StatusBar,
}
impl Plugin for CorePlugin {
	fn get_name(&self) -> String {
		"Basic Plugin".to_string()
	}

	fn get_description(&self) -> String {
		"Basic plugin providing essential tools.".to_string()
	}

	fn get_panels(&self) -> Vec<PanelConfig> {
		vec![
			PanelConfig {
				alignment: crate::panel::PanelAligment::Bottom,
				name: CorePluginPanel::StatusBar.to_string(),
			},
			PanelConfig {
				alignment: crate::panel::PanelAligment::Top,
				name: CorePluginPanel::TopBar.to_string(),
			},
		]
	}

	fn get_tools(&self) -> Vec<Tool> {
		vec![
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::StatusBar.to_string()),
				name: "Status bar".to_string(),
				component: StatusBar,
			},
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::TopBar.to_string()),
				name: "Main menu".to_string(),
				component: MenuBar,
			},
		]
	}
}
