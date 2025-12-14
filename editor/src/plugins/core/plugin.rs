use crate::{
	plugins::core::{
		top_bar::{Logo, MenuBar, WindowControls},
		StatusBar,
	},
	tool::ToolPlacement,
	PanelConfig, PanelDisplayMode, Plugin, Tool, ToolAlignment,
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
				display_mode: PanelDisplayMode::Stacked,
			},
			PanelConfig {
				alignment: crate::panel::PanelAligment::Top,
				name: CorePluginPanel::TopBar.to_string(),
				display_mode: PanelDisplayMode::Stacked,
			},
			PanelConfig {
				alignment: crate::panel::PanelAligment::Left,
				name: "Hierarchy".to_string(),
				display_mode: PanelDisplayMode::Tabbed,
			},
		]
	}

	fn get_tools(&self) -> Vec<Tool> {
		vec![
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::StatusBar.to_string()),
				name: "Status bar".to_string(),
				component: StatusBar,
				alignment: ToolAlignment::default(),
			},
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::TopBar.to_string()),
				name: "Logo".to_string(),
				component: Logo,
				alignment: ToolAlignment::default(),
			},
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::TopBar.to_string()),
				name: "Main menu".to_string(),
				component: MenuBar,
				alignment: ToolAlignment::default(),
			},
			Tool {
				placement: ToolPlacement::PanelByName(CorePluginPanel::TopBar.to_string()),
				name: "Window controls".to_string(),
				component: WindowControls,
				alignment: ToolAlignment::End,
			},
			Tool {
				placement: ToolPlacement::PanelByName("Hierarchy".to_string()),
				name: "Entities Hierarhy".to_string(),
				component: crate::plugins::core::hierarchy::EntitiesHierarhy,
				alignment: ToolAlignment::default(),
			},
		]
	}
}
