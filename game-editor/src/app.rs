use dioxus::{desktop::use_window, prelude::*};

fn Loader() -> Element {
	rsx! {
		div{"loading..."}
	}
}

fn Layout() -> Element {
	let window = use_window();
	use_effect(move || {
		let window = window.clone();
		spawn(async move {
			// i3wm treats windows created with a fixed size as floating.
			// Floating windows ignore _NET_WM_STATE_MAXIMIZED, so set_maximized() has no effect.
			// Sending "floating disable" via i3 IPC moves the window into tiling mode first,
			// after which set_maximized() works as expected.
			if std::env::var("I3SOCK").is_ok() {
				std::process::Command::new("i3-msg").arg("floating disable").spawn().ok();
			}
			window.set_maximized(true);
		});
	});

	rsx! {
		div{"app"}
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
