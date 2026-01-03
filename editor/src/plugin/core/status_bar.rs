use dioxus::prelude::*;

use crate::config::{APP_NAME, APP_VERSION};
#[component]
pub fn StatusBar() -> Element {
	rsx! {
		div {
			//	simillar but not necessarily identical to the window title
			{format!("{} v{}", APP_NAME, APP_VERSION)}
		}
	}
}
