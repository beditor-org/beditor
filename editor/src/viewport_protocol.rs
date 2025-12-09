/// Shared state for custom protocol viewport
/// This holds the latest frame data
#[derive(Clone)]
pub struct ViewportProtocolState {
	pub frame_counter: u64,
	pub latest_frame: Option<Vec<u8>>,
}

impl Default for ViewportProtocolState {
	fn default() -> Self {
		Self::new()
	}
}

impl ViewportProtocolState {
	pub fn new() -> Self {
		Self {
			frame_counter: 0,
			latest_frame: None,
		}
	}

	pub fn update_frame(&mut self, frame_data: Vec<u8>) {
		self.latest_frame = Some(frame_data);
		self.frame_counter += 1;
	}

	pub fn get_frame(&self) -> Option<Vec<u8>> {
		self.latest_frame.clone()
	}

	pub fn frame_counter(&self) -> u64 {
		self.frame_counter
	}
}
