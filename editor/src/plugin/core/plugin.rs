use crate::{
	plugin::{
		core::{
			top_bar::{main_menu::MenuBar, Logo, WindowControls},
			welcome, StatusBar,
		},
		Plugin, PluginRegistry,
	},
	workspace::{Workspace, WorkspaceRegistry},
	PanelConfig, PanelDisplayMode, PanelSocket, ResourceId, ToolAlignment,
};
use dioxus::prelude::*;

const PLUGIN_NAME: &str = "Core";
pub struct CorePlugin;

pub enum CorePluginPanel {
	TopBar,
	StatusBar,
	Welcome,
}

impl CorePluginPanel {
	pub fn id(&self) -> ResourceId {
		ResourceId::panel(PLUGIN_NAME, self.name())
	}

	pub fn name(&self) -> &str {
		match self {
			Self::TopBar => "top_bar",
			Self::StatusBar => "status_bar",
			Self::Welcome => "welcome_panel",
		}
	}
}

pub enum CoreWorkspace {
	Welcome,
	Editor,
	Playtest,
}

impl CoreWorkspace {
	pub fn id(&self) -> ResourceId {
		ResourceId::workspace(PLUGIN_NAME, self.name())
	}

	pub fn name(&self) -> &str {
		match self {
			Self::Welcome => "Welcome",
			Self::Editor => "Editor",
			Self::Playtest => "Playtest",
		}
	}
}

pub fn core_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: "Core plugin providing essential tools.".to_string(),
		entry: Some(entry),
		panels: vec![
			PanelConfig {
				socket: PanelSocket::Bottom,
				name: CorePluginPanel::StatusBar.name().to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				is_active: false,
				tools: vec![],
				workspaces: vec![
					CoreWorkspace::Welcome.id(),
					CoreWorkspace::Editor.id(),
					CoreWorkspace::Playtest.id(),
				],
			}
			.with_tools(vec![("Status bar", StatusBar, ToolAlignment::default())]),
			PanelConfig {
				socket: PanelSocket::Top,
				name: CorePluginPanel::TopBar.name().to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				is_active: false,
				tools: vec![],
				workspaces: vec![
					CoreWorkspace::Welcome.id(),
					CoreWorkspace::Editor.id(),
					CoreWorkspace::Playtest.id(),
				],
			}
			.with_tools(vec![
				("Logo", Logo, ToolAlignment::default()),
				("Window controls", WindowControls, ToolAlignment::End),
				("Main menu", MenuBar, ToolAlignment::default()),
			]),
			PanelConfig {
				socket: PanelSocket::Center,
				name: CorePluginPanel::Welcome.name().to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				is_active: true,
				tools: vec![],
				workspaces: vec![CoreWorkspace::Welcome.id()],
			}
			.with_tools(vec![("Welcome", welcome::welcome, ToolAlignment::default())]),
		],
		workspaces: vec![
			Workspace {
				name: CoreWorkspace::Welcome.name().to_string(),
				panels: vec![],
			},
			Workspace {
				name: CoreWorkspace::Editor.name().to_string(),
				panels: vec![],
			},
			Workspace {
				name: CoreWorkspace::Playtest.name().to_string(),
				panels: vec![],
			},
		],
		..Default::default()
	}
}

fn entry() -> Element {
	let mut workspace = use_context::<Signal<WorkspaceRegistry>>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	use_hook(|| {
		workspace.write().set_current(CoreWorkspace::Welcome.id());
		registry
			.write()
			.plugins
			.get_mut(PLUGIN_NAME)
			.expect("Core plugin not found")
			.is_initialized = true;
	});
	rsx!()
}
