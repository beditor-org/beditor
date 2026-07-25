use dioxus::prelude::*;
use vitronix::window::use_maximize;

fn Loader() -> Element {
	rsx! {
		div { "loading..." }
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
			tokio::time::sleep(std::time::Duration::from_secs(2)).await;
			loading.set(false);
		});
	});

	rsx! {
		if loading() {
			Loader{}
		} else {
			Layout{}
		}
	}
}
