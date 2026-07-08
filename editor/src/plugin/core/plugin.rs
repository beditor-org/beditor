use dioxus::prelude::*;
use lazy_static::lazy_static;

use crate::{
	config::RecentProject,
	event::Events,
	plugin::{
		asset_browser::ASSET_BROWSER_WORKSPACE,
		core::{welcome, StatusBar},
		Plugin, PluginRegistry,
	},
	project::{CurrentProject, ProjectOpenedEvent},
	workspace::{Workspace, WorkspaceRegistry},
	EditorConfig, PanelConfig, PanelDisplayMode, PanelSocket, ResourceId, ToolAlignment,
};

const PLUGIN_NAME: &str = "Core";
lazy_static! {
	//	workspaces
	pub static ref CORE_WELCOME_WORKSPACE: ResourceId = ResourceId::workspace(PLUGIN_NAME, "welcome");
	pub static ref CORE_SCENE_EDITOR_WORKSPACE: ResourceId = ResourceId::workspace(PLUGIN_NAME, "scene_editor");
	pub static ref CORE_PLAYTEST_WORKSPACE: ResourceId = ResourceId::workspace(PLUGIN_NAME, "playtest");
	//	panels
	pub static ref CORE_STATUS_BAR_PANEL: ResourceId = ResourceId::panel(PLUGIN_NAME, "status_bar");
	pub static ref CORE_TOP_BAR_PANEL: ResourceId = ResourceId::panel(PLUGIN_NAME, "top_bar");
	pub static ref CORE_WELCOME_PANEL: ResourceId = ResourceId::panel(PLUGIN_NAME, "welcome");
}

pub struct CorePlugin;

pub fn core_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: "Core plugin providing essential tools.".to_string(),
		setup_context: Some(setup_context),
		entry: Some(entry),
		panels: vec![
			PanelConfig {
				socket: PanelSocket::Bottom,
				name: CORE_STATUS_BAR_PANEL.name().to_string(),
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
			.with_tools(vec![("Status bar", StatusBar, ToolAlignment::default())]),
			PanelConfig {
				socket: PanelSocket::Center,
				name: CORE_WELCOME_PANEL.name().to_string(),
				display_mode: PanelDisplayMode::Stacked,
				is_visible: true,
				is_active: true,
				tools: vec![],
				workspaces: vec![CORE_WELCOME_WORKSPACE.clone()],
			}
			.with_tools(vec![("Welcome", welcome::welcome, ToolAlignment::default())]),
		],
		workspaces: vec![
			Workspace {
				name: CORE_WELCOME_WORKSPACE.name().to_string(),
				panels: vec![],
			},
			Workspace {
				name: CORE_SCENE_EDITOR_WORKSPACE.name().to_string(),
				panels: vec![],
			},
			Workspace {
				name: CORE_PLAYTEST_WORKSPACE.name().to_string(),
				panels: vec![],
			},
		],
		..Default::default()
	}
}

fn entry() -> Element {
	let mut workspace = use_context::<Signal<WorkspaceRegistry>>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	let mut current_project = use_context::<Signal<CurrentProject>>();
	let mut config = use_context::<Signal<EditorConfig>>();
	let events = use_context::<Events>();

	use_hook(|| {
		workspace.write().set_current(CORE_WELCOME_WORKSPACE.clone());
		registry
			.write()
			.plugins
			.get_mut(PLUGIN_NAME)
			.expect("Core plugin not found")
			.is_initialized = true;
	});

	events.subscribe::<ProjectOpenedEvent>(move |ev| {
		current_project.set(CurrentProject {
			project: Some(ev.project.clone()),
		});
		config.write().add_recent_project(RecentProject {
			name: ev.project.name.clone(),
			path: ev.project.path.clone(),
		});
		workspace.write().set_current(ASSET_BROWSER_WORKSPACE.clone());
	});
	rsx!()
}

fn setup_context() -> Element {
	use_context_provider(|| Signal::new(CurrentProject::default()));
	rsx!()
}
