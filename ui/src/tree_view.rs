use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};

#[derive(Clone, Debug, PartialEq)]
pub struct TreeItem {
	pub id: u32,
	pub label: String,
	pub children: Vec<TreeItem>,
}

#[component]
pub fn TreeView(items: Vec<TreeItem>) -> Element {
	rsx! {
		div { class: "tree-view",
			for item in items {
				TreeNode { item: item.clone() }
			}
		}
	}
}

#[component]
fn TreeNode(item: TreeItem) -> Element {
	let has_children = !item.children.is_empty();

	if has_children {
		rsx! {
			Collapsible {
				CollapsibleTrigger {
					div { class: "tree-node-trigger",
						span { class: "tree-node-label", "{item.label}" }
						span { class: "tree-node-id", "({item.id})" }
					}
				}
				CollapsibleContent {
					div { class: "pl-4",
						for child in item.children {
							TreeNode { item: child.clone() }
						}
					}
				}
			}
		}
	} else {
		rsx! {
			div { class: "tree-node-leaf",
				span { class: "tree-node-icon", "◦" }
				span { class: "tree-node-label", "{item.label}" }
				span { class: "tree-node-id", "({item.id})" }
			}
		}
	}
}
