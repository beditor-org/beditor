use bridge::protocol::bep::{ComponentData, FieldData, FieldValue};
use dioxus::prelude::*;

#[component]
pub fn Inspector() -> Element {
	let selected_entity = use_context::<Signal<Option<u32>>>();
	let components = use_context::<Signal<Vec<ComponentData>>>();
	let components = components.read();

	rsx! {
		div { class: "flex flex-col flex-1 min-h-0 overflow-y-auto p-2",
			if selected_entity.read().is_none() {
				div { class: "text-text-muted text-sm p-2", "Select an entity to see its components" }
			} else {
				for component in components.iter() {
					ComponentBlock { component: component.clone() }
				}
			}
		}
	}
}

#[component]
fn ComponentBlock(component: ComponentData) -> Element {
	let mut collapsed = use_signal(|| false);

	rsx! {
		div { class: "mb-1 border border-border rounded",
			// Header
			div {
				class: "flex items-center px-2 py-1 bg-secondary cursor-pointer select-none",
				onclick: move |_| collapsed.set(!collapsed()),
				span { class: "text-xs text-text-muted mr-1", if collapsed() { "▶" } else { "▼" } }
				span { class: "text-sm font-semibold text-text", "{component.short_name}" }
			}
			// Fields
			if !collapsed() {
				div { class: "px-2 py-1",
					if !component.fields.is_empty() {
						for field in component.fields.iter() {
							FieldRow { field: field.clone() }
						}
					}
				}
			}
		}
	}
}

#[component]
fn FieldRow(field: FieldData) -> Element {
	rsx! {
		div { class: "flex items-center gap-2 py-0.5",
			span { class: "text-xs text-text-muted w-32 shrink-0 truncate", title: "{field.name}", "{field.name}" }
			span { class: "text-xs text-text-muted italic shrink-0", title: "{field.field_type}",
				{field.field_type.split("::").last().unwrap_or(&field.field_type).to_string()}
			}
			FieldControl { value: field.value.clone() }
		}
	}
}

#[component]
fn FieldControl(value: FieldValue) -> Element {
	match value {
		FieldValue::Bool(b) => rsx! {
			input {
				r#type: "checkbox",
				checked: b,
				class: "accent-accent cursor-pointer",
			}
		},
		FieldValue::F32(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::F64(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::I32(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::U32(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::I64(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::U64(v) => rsx! { span { class: "text-xs text-text font-mono", "{v}" } },
		FieldValue::String(s) => rsx! { span { class: "text-xs text-text font-mono truncate", "{s}" } },
		FieldValue::Vec2 { x, y } => rsx! { span { class: "text-xs text-text font-mono", "({x}, {y})" } },
		FieldValue::Vec3 { x, y, z } => rsx! { span { class: "text-xs text-text font-mono", "({x}, {y}, {z})" } },
		FieldValue::Vec4 { x, y, z, w } => rsx! { span { class: "text-xs text-text font-mono", "({x}, {y}, {z}, {w})" } },
		FieldValue::Quat { x, y, z, w } => {
			rsx! { span { class: "text-xs text-text font-mono", "({x:.3}, {y:.3}, {z:.3}, {w:.3})" } }
		}
		FieldValue::Color { r, g, b, a } => rsx! {
			div { class: "flex items-center gap-1",
				div {
					class: "w-4 h-4 rounded-sm border border-border shrink-0",
					style: "background: rgba({(r*255.0) as u8},{(g*255.0) as u8},{(b*255.0) as u8},{a})",
				}
				span { class: "text-xs text-text font-mono", "({r:.2}, {g:.2}, {b:.2}, {a:.2})" }
			}
		},
		FieldValue::Enum { variant, .. } => rsx! { span { class: "text-xs text-text font-mono", "{variant}" } },
		FieldValue::Struct(_) | FieldValue::List(_) | FieldValue::Unknown(_) => rsx! {
			span { class: "text-xs text-text-muted italic", "…" }
		},
	}
}
