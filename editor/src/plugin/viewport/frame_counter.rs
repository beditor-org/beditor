use dioxus::prelude::*;

use crate::plugin::viewport::plugin::ViewportState;
#[component]
pub fn FrameCounter() -> Element {
	let viewport_state = use_context::<Signal<ViewportState>>();
	rsx! {
		div {
			"Frame: {viewport_state.read().frame_count}"
		}
	}
}
