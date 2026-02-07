use crate::{components::Panel, PanelConfig};

pub struct SceneEditorWorkspace {
	panels: Vec<ResourceId>,
}

pub struct SceneTreePanel;
impl Panel for SceneTreePanel {
	fn tools(&self) -> Vec<Tool> {
		vec![scene_tree()]
	}
}

pub fn ComponentInspectorPanel() -> PanelConfig {
	PanelConfig {
		name: "Component Inspector".to_string(),
		tools: vec![scene_tree()],
	}
}

pub fn scene_tree() -> Tool {
	Tool {
		placement: ToolPlacement::PanelByAlignment(PanelSocket::Start),
		name: "Scene Tree".to_string(),
		component: scene_tree_panel,
		alignment: ToolAlignment::Start,
		workspaces: vec![],
	}
}

pub fn component_inspector() -> Element {}
