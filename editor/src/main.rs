mod components;
mod config;
mod editor;
mod layout;
mod tool;
mod windows_manager;

use anyhow::Result;
use std::sync::{Arc, RwLock};

use components::{App, PanelAligment, PanelState};
use config::EditorConfig;
use tool::Tool;

#[derive(Clone, Debug)]
#[derive(Default)]
struct EditorState {
	pub selected_entity: Option<String>,
	// pub entities: Vec<EntityInfo>,
	pub game_connected: bool,
	pub config: EditorConfig,
	pub layout: layout::LayoutConfig,
	pub panels: Vec<components::PanelState>,
	// pub game_process: Option<Arc<Mutex<GameProcess>>>,
}


#[tokio::main]
async fn main() -> Result<()> {
	let config = EditorConfig::default();

	let editor_state = EditorState {
		panels: vec![
			// PanelState {
			// 	alignment: PanelAligment::Top,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Top,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Bottom,
			// 	..Default::default()
			// },
			PanelState {
				alignment: PanelAligment::Top,
				name: "Top Bar".to_string(),
				tools: vec![Tool {
					require_stand_alone_panel: None,
					name: "Menu".to_string(),
					component: components::TopBar,
					panel_group: None,
				}],
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Bottom,
				name: "Status Bar".to_string(),
				tools: vec![Tool {
					require_stand_alone_panel: None,
					name: "StatusBar".to_string(),
					component: components::StatusBar,
					panel_group: None,
				}],
				..Default::default()
			},
			PanelState {
				alignment: PanelAligment::Left,
				name: "Basic tools".to_string(),
				tools: vec![Tool {
					require_stand_alone_panel: None,
					name: "Dumy".to_string(),
					component: components::Dumy,
					panel_group: None,
				}],
				..Default::default()
			},
			// PanelState {
			// 	alignment: PanelAligment::Right,
			// 	tools: vec![Tool {
			// 		require_stand_alone_panel: None,
			// 		name: "Dumy".to_string(),
			// 		component: components::Dumy,
			// 		panel_group: None,
			// 	}],
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Right,
			// 	tools: vec![Tool {
			// 		require_stand_alone_panel: None,
			// 		name: "Dumy".to_string(),
			// 		component: components::Dumy,
			// 		panel_group: None,
			// 	}],
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Right,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Right,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Right,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::CenterBottom,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::CenterTop,
			// 	..Default::default()
			// },
			// PanelState {
			// 	alignment: PanelAligment::Left,
			// 	..Default::default()
			// },
		],
		..Default::default()
	};

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.top_bar.title.to_string())
		.with_decorations(true)
		.with_resizable(true);
	let window_cfg = dioxus::desktop::Config::new().with_window(window);

	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.launch(App);

	Ok(())
}
