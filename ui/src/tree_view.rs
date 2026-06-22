use dioxus::prelude::*;
use dioxus_primitives::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use lucide_dioxus::{ChevronDown, ChevronRight};

#[derive(Clone, Debug, PartialEq)]
pub struct TreeItem {
	pub id: u32,
	pub label: String,
	pub children: Vec<TreeItem>,
}

#[component]
pub fn TreeView(items: Vec<TreeItem>, on_select: Option<EventHandler<u32>>) -> Element {
	rsx! {
		div { class: "tree-view",
			for item in items {
				TreeNode { item: item.clone(), on_select: on_select.clone() }
			}
		}
	}
}

#[component]
fn TreeNode(item: TreeItem, on_select: Option<EventHandler<u32>>) -> Element {
	let has_children = !item.children.is_empty();
	let mut is_open = use_signal(|| false);
	let id = item.id;

	if has_children {
		rsx! {
			Collapsible {
				open: is_open(),
				on_open_change: move |open| {
					is_open.set(open);
				},
				CollapsibleTrigger {
					div {
						class: "tree-node-trigger flex items-center gap-1",
						onclick: move |_| {
							if let Some(h) = &on_select {
								h.call(id);
							}
						},
						span { class: "tree-node-icon",
							if is_open() {
								ChevronDown { size: 16 }
							} else {
								ChevronRight { size: 16 }
							}
						}
						span { class: "tree-node-label", "{item.label}" }
						span { class: "tree-node-id", "({item.id})" }
					}
				}
				CollapsibleContent {
					div { class: "pl-4 ml-2 border-l border-border",
						for child in item.children {
							TreeNode { item: child.clone(), on_select: on_select.clone() }
						}
					}
				}
			}
		}
	} else {
		rsx! {
			div {
				class: "tree-node-leaf",
				onclick: move |_| {
					if let Some(h) = &on_select {
						h.call(id);
					}
				},
				span { class: "tree-node-icon", "◦" }
				span { class: "tree-node-label", "{item.label}" }
				span { class: "tree-node-id", "({item.id})" }
			}
		}
	}
}
