use crate::EditorConfig;

#[derive(Clone, Debug, Default)]
pub struct EditorContext {
	pub selected_entity: Option<String>,
	// pub entities: Vec<EntityInfo>,
	pub game_connected: bool,
	pub config: EditorConfig,
	// pub game_process: Option<Arc<Mutex<GameProcess>>>,
}
