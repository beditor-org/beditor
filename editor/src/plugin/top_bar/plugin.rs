use crate::{
	plugin::{
		core::plugin::{CORE_PLAYTEST_WORKSPACE, CORE_SCENE_EDITOR_WORKSPACE, CORE_TOP_BAR_PANEL, CORE_WELCOME_WORKSPACE},
		top_bar::{logo::Logo, menu_bar::MenuBar, window_controls::WindowControls, workspace_tabs::WorkspaceTabsTool},
		Plugin,
	},
	PanelConfig, PanelDisplayMode, PanelSocket, ToolAlignment,
};

const PLUGIN_NAME: &str = "Top bar";

pub fn top_bar_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: "Top bar plugin providing default top bar menu.".to_string(),
		panels: vec![PanelConfig {
			socket: PanelSocket::Top,
			name: CORE_TOP_BAR_PANEL.name().to_string(),
			display_mode: PanelDisplayMode::Stacked,
			is_visible: true,
			is_active: false,
			tools: vec![],
			workspaces: vec![
				CORE_WELCOME_WORKSPACE.clone(),
				CORE_SCENE_EDITOR_WORKSPACE.clone(),
				CORE_PLAYTEST_WORKSPACE.clone(),
			],
		}
		.with_tools(vec![
			("Logo", Logo, ToolAlignment::default()),
			("Main menu", MenuBar, ToolAlignment::default()),
			("Workspace tabs", WorkspaceTabsTool, ToolAlignment::default()),
			("Window controls", WindowControls, ToolAlignment::End),
		])],
		..Default::default()
	}
}
