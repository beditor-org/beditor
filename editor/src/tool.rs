use crate::components::PanelState;

#[derive(Clone, Debug, PartialEq)]
pub struct Tool {
	require_stand_alone_panel: Option<PanelState>, // otherwise should be added manually to existing panel
	name: String,
}
