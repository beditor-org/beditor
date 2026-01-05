use std::{
	fs::read_dir,
	path::{Path, PathBuf},
};

use dioxus::prelude::*;
use lazy_static::lazy_static;
use ui::GridView;

use crate::{
	plugin::{
		core::plugin::{CORE_STATUS_BAR_PANEL, CORE_TOP_BAR_PANEL},
		Plugin,
	},
	project::CurrentProject,
	workspace::Workspace,
	PanelConfig, PanelDisplayMode, PanelSocket, ResourceId, ToolAlignment,
};

const PLUGIN_NAME: &str = "Asset Browser";

lazy_static! {
	pub static ref ASSET_BROWSER_WORKSPACE: ResourceId = ResourceId::workspace(PLUGIN_NAME, "asset browser");
	pub static ref ASSET_BROWSER_PANEL: ResourceId = ResourceId::panel(PLUGIN_NAME, "asset browser");
}

pub fn asset_browser_plugin() -> Plugin {
	Plugin {
		name: PLUGIN_NAME.to_string(),
		description: "Plugin responsible for browsing and managing game assets".to_string(),
		panels: vec![PanelConfig {
			socket: PanelSocket::Center,
			name: ASSET_BROWSER_PANEL.name().to_string(),
			display_mode: PanelDisplayMode::Stacked,
			is_visible: true,
			is_active: true,
			tools: vec![],
			workspaces: vec![ASSET_BROWSER_WORKSPACE.clone()],
		}
		.with_tools(vec![("Asset Browser", AssetBrowser, ToolAlignment::default())])],
		workspaces: vec![Workspace {
			name: ASSET_BROWSER_WORKSPACE.name().to_string(),
			panels: vec![CORE_TOP_BAR_PANEL.clone(), CORE_STATUS_BAR_PANEL.clone()],
		}],
		..Default::default()
	}
}

#[derive(Clone)]
enum AssetItem {
	Scene,
	Config,
	Tilemap,
	Image,
	Unknown,
}
#[derive(Clone)]
enum ItemType {
	Folder,
	File(AssetItem),
	MoveUp,
}
#[derive(Clone)]
struct Item {
	path: PathBuf,
	item_type: ItemType,
}

#[component]
pub fn AssetBrowser() -> Element {
	let current_project = use_context::<Signal<CurrentProject>>();
	let project_path = current_project.read().project.as_ref().map(|p| p.path.clone()).unwrap();
	let mut path = use_signal(|| current_project.read().project.as_ref().map(|p| p.path.clone()).unwrap());
	let mut items = vec![];
	if path() != project_path {
		if let Some(parent) = PathBuf::from(path()).parent() {
			items.push(Item {
				path: parent.to_path_buf(),
				item_type: ItemType::MoveUp,
			})
		}
	};
	for entry in read_dir(path())? {
		let entry = entry?;
		if entry.file_type()?.is_dir() {
			items.push(Item {
				path: entry.path(),
				item_type: ItemType::Folder,
			});
		} else {
			let file_name = entry.file_name();
			items.push(Item {
				path: entry.path(),
				item_type: ItemType::File(AssetItem::Unknown),
			});
		}
	}

	// Sort: folders first, then files
	items.sort_by(|a, b| match (&a.item_type, &b.item_type) {
		(ItemType::MoveUp, ItemType::MoveUp) => std::cmp::Ordering::Equal,
		(ItemType::MoveUp, _) => std::cmp::Ordering::Less,
		(_, ItemType::MoveUp) => std::cmp::Ordering::Greater,
		(ItemType::Folder, ItemType::File(_)) => std::cmp::Ordering::Less,
		(ItemType::File(_), ItemType::Folder) => std::cmp::Ordering::Greater,
		_ => a.path.file_name().cmp(&b.path.file_name()),
	});

	rsx! {
		div {
			class: "w-full h-[calc(100vh-8rem)] overflow-y-auto",
			"{path.read()}",
			div {
				class: "grid grid-cols-[repeat(auto-fill,minmax(80px,1fr))] gap-2 p-4",
				{items.iter().map(|item| {
					let item_path = item.path.clone();
					let item_type = item.item_type.clone();
					rsx! {
						div {
							class: "group flex flex-col items-center p-2 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 cursor-pointer transition-colors duration-150 select-none",
							ondoubleclick: move |_| {
							if matches!(item_type, ItemType::Folder | ItemType::MoveUp) {
									path.set(item_path.to_string_lossy().to_string());
								}
							},
							// Icon
							div {
								class: "text-4xl mb-1",
								{match item_type {
									ItemType::Folder => "📁",
									ItemType::File(AssetItem::Scene) => "🎬",
									ItemType::File(AssetItem::Config) => "⚙️",
									ItemType::File(AssetItem::Tilemap) => "🗺️",
									ItemType::File(AssetItem::Image) => "🖼️",
									ItemType::File(AssetItem::Unknown) => "📄",
									ItemType::MoveUp => "⬆️",
								}}
							}

							// Filename
							div {
								class: "text-xs text-center truncate w-full group-hover:text-gray-900 dark:group-hover:text-gray-100",
								title: "{item.path.display()}",
								{item.path.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string()}
							}
						}
					}
				})}
			}
		}
	}
}
