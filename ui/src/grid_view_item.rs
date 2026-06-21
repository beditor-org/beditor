use dioxus::prelude::*;

#[component]
pub fn GridViewItem(name: String, item_type: ItemType) -> Element {
	rsx! {
		div{
			"Grid View Component"
		}
	}
}
