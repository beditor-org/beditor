use crate::Plugin;

pub struct ViewportWindowOverlayPlugin;

impl Plugin for ViewportWindowOverlayPlugin {
	fn get_name(&self) -> String {
		"Viewport: Window Overlay (Legacy)".to_string()
	}

	fn get_description(&self) -> String {
		"Legacy method using separate window overlay. Has issues with tiling WMs.".to_string()
	}

	fn on_load(&mut self) {
		println!("✓ ViewportWindowOverlayPlugin loaded");
	}

	fn on_unload(&mut self) {
		println!("✓ ViewportWindowOverlayPlugin unloaded");
	}
}
