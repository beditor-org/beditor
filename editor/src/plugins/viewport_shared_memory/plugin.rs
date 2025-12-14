use crate::Plugin;

pub struct ViewportSharedMemoryPlugin;

impl Plugin for ViewportSharedMemoryPlugin {
	fn get_name(&self) -> String {
		"Viewport: Shared Memory".to_string()
	}

	fn get_description(&self) -> String {
		"Provides high-performance viewport rendering via shared memory".to_string()
	}

	fn on_load(&mut self) {
		println!("✓ ViewportSharedMemoryPlugin loaded");
	}

	fn on_unload(&mut self) {
		println!("✓ ViewportSharedMemoryPlugin unloaded");
	}
}
