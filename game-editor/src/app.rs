use dioxus::prelude::*;

fn Loader() -> Element {
	rsx! {
		div{"loading..."}
	}
}

fn Layout() -> Element {
	rsx! {
		div{"app"}
	}
}

pub fn App() -> Element {
	rsx! {
		Loader{}

	}
}
