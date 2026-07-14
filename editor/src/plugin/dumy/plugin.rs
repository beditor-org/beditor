use std::collections::HashMap;

use dioxus::prelude::*;
use icu_locale_core::langid;
use strum::Display;
use tracing::info;

use crate::{
	event::Events,
	main_menu::{MenuBarGroupConfig, MenuBarItemConfig},
	plugin::{core::plugin::CORE_STATUS_BAR_PANEL, dumy::dumy::Dumy, i18n_core::plugin::Translation, Plugin, PluginRegistry},
	tool::ToolPlacement,
	PanelConfig, PanelDisplayMode, PanelSocket, Tool, ToolAlignment,
};

const PLUGIN_NAME: &str = "Dumy";
pub struct DumyMenuClick;

#[derive(Display)]
pub enum DumyPluginPanel {
	#[strum(to_string = "Status dumy bar")]
	StatusBar,
	#[strum(to_string = "Left dumy bar")]
	LeftBar,
}
pub struct DumyPlugin;
pub fn dumy_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: format!("{PLUGIN_NAME} plugin for testing purposes"),
		menu_groups: vec![MenuBarGroupConfig {
			label: "main_menu:dumy",
			items: vec![
				MenuBarItemConfig {
					label: "main_menu:dumy:do_something",
					action: Some(|_| {
						info!("Dumy plugin menu item clicked!");
					}),
					..Default::default()
				},
				MenuBarItemConfig {
					label: "main_menu:dumy:do_more",
					action: Some(|events| {
						events.publish(DumyMenuClick);
					}),
					..Default::default()
				},
			],
		}],
		i18n: Some(HashMap::from([
			(
				langid!("en"),
				HashMap::from([
					("main_menu:dumy".to_string(), Translation::Single("Dumy".to_string())),
					(
						"main_menu:dumy:do_something".to_string(),
						Translation::Single("Do something".to_string()),
					),
					(
						"main_menu:dumy:do_more".to_string(),
						Translation::Single("Do more".to_string()),
					),
				]),
			),
			(
				langid!("ua"),
				HashMap::from([
					("main_menu:dumy".to_string(), Translation::Single("Bidon".to_string())),
					(
						"main_menu:dumy:do_something".to_string(),
						Translation::Single("Зробити щось".to_string()),
					),
					(
						"main_menu:dumy:do_more".to_string(),
						Translation::Single("Зробити більше".to_string()),
					),
				]),
			),
		])),
		panels: vec![PanelConfig {
			socket: PanelSocket::Left,
			name: DumyPluginPanel::LeftBar.to_string(),
			display_mode: PanelDisplayMode::Tabbed,
			is_visible: true,
			is_active: false,
			tools: vec![],
			workspaces: vec![],
		}
		.with_tools(vec![("Dumy tool", Dumy, ToolAlignment::default())])],

		// Tool placed in another plugin's panel (Core's status bar)
		tools: vec![Tool {
			placement: ToolPlacement::ByResourceId(CORE_STATUS_BAR_PANEL.clone()),
			name: "Dumy tool".to_string(),
			component: Dumy,
			alignment: ToolAlignment::End,
			workspaces: vec![],
		}],
		entry: Some(dumy_entry),
		..Default::default()
	}
}

fn dumy_entry() -> Element {
	info!("{PLUGIN_NAME} plugin rendering");
	let events = use_context::<Events>();
	let mut registry = use_context::<Signal<PluginRegistry>>();
	use_hook(|| {
		registry.write().plugins.get_mut(PLUGIN_NAME).unwrap().is_initialized = true;
		info!("hello from {PLUGIN_NAME} plugin!");
	});
	use_effect(move || {
		events.subscribe::<DumyMenuClick>(move |_| {
			info!("Dummy subscribed menu click event");
		});
	});
	rsx!()
}
