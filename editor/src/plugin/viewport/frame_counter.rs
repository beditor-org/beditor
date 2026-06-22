use dioxus::prelude::*;

use crate::plugin::viewport::plugin::ViewportState;
#[component]
pub fn FrameCounter() -> Element {
	let viewport_state = use_context::<Signal<ViewportState>>();
	rsx! {
		div {
			"{viewport_state.read().fps:.0} fps"
		}
	}
}
