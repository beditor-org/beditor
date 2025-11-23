#[component]
pub fn TreeItem(name: String, selected: bool, onclick: EventHandler<MouseEvent>) -> Element {
	let class_name = if selected {
		"tree-item tree-item-selected"
	} else {
		"tree-item"
	};

	rsx! {
		div {
			class: class_name,
			onclick: move |evt| onclick.call(evt),
			"▸ {name}"
		}
	}
}
