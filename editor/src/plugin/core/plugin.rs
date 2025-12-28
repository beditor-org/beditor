use crate::{
	plugin::{
		core::{
			top_bar::{main_menu::MenuBar, Logo, WindowControls},
			StatusBar,
		},
		Plugin,
	},
	PanelConfig, PanelDisplayMode, PanelSocket, ToolAlignment,
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
pub fn core_plugin() -> Plugin {
	Plugin {
		name: "Core Plugin".to_string(),
		description: "Core plugin providing essential tools.".to_string(),
		panels: vec![
			PanelConfig {
				socket: PanelSocket::Bottom,
				name: CorePluginPanel::StatusBar.to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				tools: vec![],
			}
			.with_tools(vec![("Status bar", StatusBar, ToolAlignment::default())]),
			PanelConfig {
				socket: PanelSocket::Top,
				name: CorePluginPanel::TopBar.to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				tools: vec![],
			}
			.with_tools(vec![
				("Logo", Logo, ToolAlignment::default()),
				("Window controls", WindowControls, ToolAlignment::End),
				("Main menu", MenuBar, ToolAlignment::default()),
			]),
			PanelConfig {
				socket: PanelSocket::Left,
				name: "Hierarchy".to_string(),
				display_mode: PanelDisplayMode::Tabbed,
				is_visible: true,
				tools: vec![],
			},
		],
		..Default::default()
	}
}
