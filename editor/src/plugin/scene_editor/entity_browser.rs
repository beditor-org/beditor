use bridge::protocol::bep::{BepProtocol, EntityInfo, EntityKind};
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

#[derive(Clone, PartialEq)]
enum Filter {
	Entities,
	Resources,
}

#[component]
pub fn EntityBrowser() -> Element {
	let entities = use_context::<Signal<Vec<EntityInfo>>>();
	let mut selected_entity = use_context::<Signal<Option<u32>>>();
	let mut filter = use_signal(|| Filter::Entities);

	let filtered: Vec<EntityInfo> = entities
		.read()
		.iter()
		.filter(|e| match *filter.read() {
			Filter::Entities => e.kind == EntityKind::Entity,
			Filter::Resources => e.kind == EntityKind::Resource,
		})
		.cloned()
		.collect();

	let tree = match *filter.read() {
		Filter::Entities => build_tree(&filtered),
		Filter::Resources => filtered
			.iter()
			.map(|e| TreeItem {
				id: e.id,
				label: e.name.clone(),
				children: vec![],
			})
			.collect(),
	};

	rsx! {
		div { class: "flex flex-col flex-1 min-h-0",
			div { class: "flex border-b border-border shrink-0",
				button {
					class: if *filter.read() == Filter::Entities {
						"px-3 py-1 text-xs font-medium border-b-2 border-primary text-primary"
					} else {
						"px-3 py-1 text-xs text-muted-foreground hover:text-foreground"
					},
					onclick: move |_| filter.set(Filter::Entities),
					"Entities"
				}
				button {
					class: if *filter.read() == Filter::Resources {
						"px-3 py-1 text-xs font-medium border-b-2 border-primary text-primary"
					} else {
						"px-3 py-1 text-xs text-muted-foreground hover:text-foreground"
					},
					onclick: move |_| filter.set(Filter::Resources),
					"Resources"
				}
			}
			div { class: "flex flex-col flex-1 min-h-0 overflow-y-auto p-2",
				if tree.is_empty() {
					div { class: "text-muted-foreground text-sm p-2", "No entities" }
				} else {
					TreeView {
						items: tree,
						on_select: move |id| {
							let current = *selected_entity.read();
							if current == Some(id) {
								selected_entity.set(None);
							} else {
								selected_entity.set(Some(id));
							}
						},
						on_focus: move |id| {
							if let Some(protocol) = try_use_context::<BepProtocol>() {
								protocol.focus_entity(id);
							}
						},
					}
				}
			}
		}
	}
}
