use dioxus::prelude::*;
pub fn welcome() -> Element {
	rsx! {
		 h1 { "Welcome to Beditor!" }
		 p {
			 a { "create new project"}

		  }
			p {

				a { "load project"}
			}
	}
}
