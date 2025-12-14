use dioxus::prelude::*;
use ui::{TreeItem, TreeView};

// Mock data for development
fn get_mock_entities() -> Vec<TreeItem> {
	vec![TreeItem {
		id: 1,
		label: "Scene Root".to_string(),
		children: vec![
			TreeItem {
				id: 2,
				label: "Player".to_string(),
				children: vec![
					TreeItem {
						id: 3,
						label: "Camera".to_string(),
						children: vec![],
					},
					TreeItem {
						id: 4,
						label: "PlayerMesh".to_string(),
						children: vec![],
					},
				],
			},
			TreeItem {
				id: 5,
				label: "Environment".to_string(),
				children: vec![
					TreeItem {
						id: 6,
						label: "Terrain".to_string(),
						children: vec![],
					},
					TreeItem {
						id: 7,
						label: "Props".to_string(),
						children: vec![
							TreeItem {
								id: 8,
								label: "Crate_01".to_string(),
								children: vec![],
							},
							TreeItem {
								id: 9,
								label: "Crate_02".to_string(),
								children: vec![],
							},
							TreeItem {
								id: 10,
								label: "Barrel".to_string(),
								children: vec![],
							},
						],
					},
					TreeItem {
						id: 11,
						label: "DirectionalLight".to_string(),
						children: vec![],
					},
				],
			},
			TreeItem {
				id: 12,
				label: "UI Canvas".to_string(),
				children: vec![
					TreeItem {
						id: 13,
						label: "HealthBar".to_string(),
						children: vec![],
					},
					TreeItem {
						id: 14,
						label: "Minimap".to_string(),
						children: vec![],
					},
				],
			},
		],
	}]
}

pub fn EntitiesHierarhy() -> Element {
	let entities = get_mock_entities();

	rsx! {
		TreeView { items: entities }
	}
}
