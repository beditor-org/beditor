use dioxus::prelude::*;
use vitronix::window::{use_drag_window, use_maximize};

fn Loader() -> Element {
	rsx! {
		div {
			onmousedown: use_drag_window(),
			"loading..."
		}
	}
}

fn Layout() -> Element {
	use_maximize();

	rsx! {
		div { "app" }
	}
}

pub fn App() -> Element {
	let mut loading = use_signal(|| true);

	use_effect(move || {
		spawn(async move {
			tokio::time::sleep(std::time::Duration::from_secs(5)).await;
			loading.set(false);
		});
	});

	rsx! {
		style { {include_str!("../public/main.css")} }
		if loading() {
			Loader{}
		} else {
			Layout{}
		}
	}
}
