use crate::{Plugin, Tool};

pub struct ViewportCustomProtocolPlugin;

impl Plugin for ViewportCustomProtocolPlugin {
	fn get_name(&self) -> String {
		"Viewport: Custom Protocol".to_string()
	}

	fn get_description(&self) -> String {
		"Provides viewport rendering via custom protocol handler".to_string()
	}

	fn on_load(&mut self) {
		println!("✓ ViewportCustomProtocolPlugin loaded");
	}

	fn on_unload(&mut self) {
		println!("✓ ViewportCustomProtocolPlugin unloaded");
	}
}
