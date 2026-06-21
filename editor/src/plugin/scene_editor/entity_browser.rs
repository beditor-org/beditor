use bridge::protocol::bep::EntityInfo;
use dioxus::prelude::*;
use ui::{TreeItem, TreeView};

pub fn build_tree(entities: &[EntityInfo]) -> Vec<TreeItem> {
	build_children(entities, None)
}

fn build_children(entities: &[EntityInfo], parent: Option<u32>) -> Vec<TreeItem> {
	entities
		.iter()
		.filter(|e| e.parent == parent)
		.map(|e| TreeItem {
			id: e.id,
			label: e.name.clone(),
			children: build_children(entities, Some(e.id)),
		})
		.collect()
}

#[component]
pub fn EntityBrowser() -> Element {
	let entities = use_context::<Signal<Vec<EntityInfo>>>();
	let tree = build_tree(&entities.read());

	rsx! {
		div { class: "flex flex-col flex-1 min-h-0 overflow-y-auto p-2",
			if tree.is_empty() {
				div { class: "text-muted-foreground text-sm p-2", "No entities" }
			} else {
				TreeView { items: tree }
			}
		}
	}
}
