use bevy::{ecs::resource::IsResource, prelude::*};
use bridge::protocol::bep::{BepProtocol, ComponentData, EntityInfo, EntityKind, FieldData, FieldValue};

use crate::app::ResMultiplexer;
use crate::gizmo::GizmoTarget;

#[derive(Resource)]
pub struct BepResource {
	pub protocol: BepProtocol,
}

#[derive(Resource, Default)]
struct InspectorState {
	selected: Option<u32>,
	last_components: Vec<ComponentData>,
}

/// Set by poll_bep_messages when the editor requests a camera focus.
/// Consumed by controll_editor_camera to move the orbit pivot.
#[derive(Resource, Default)]
pub struct CameraFocusRequest {
	pub position: Option<Vec3>,
}

pub struct BepPlugin;

impl Plugin for BepPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<InspectorState>()
			.init_resource::<CameraFocusRequest>()
			.add_systems(PostStartup, send_game_ready_and_entities)
			.add_systems(Update, poll_bep_messages)
			.add_systems(Update, watch_selected_entity)
			.add_systems(PostUpdate, watch_entity_list);
	}
}

fn collect_entity_list(world: &World) -> Vec<EntityInfo> {
	let type_registry = world.resource::<AppTypeRegistry>().clone();
	let registry = type_registry.read();

	// Collect entity data while world is immutably borrowed
	let entity_data: Vec<(u32, Option<String>, Option<u32>, Vec<bevy::ecs::component::ComponentId>, bool)> = world
		.iter_entities()
		.map(|e| {
			(
				e.id().index_u32(),
				e.get::<Name>().map(|n| n.as_str().to_string()),
				e.get::<ChildOf>().map(|c| c.parent().index_u32()),
				e.archetype().components().iter().copied().collect(),
				e.get::<IsResource>().is_some(),
			)
		})
		.collect();

	let components_info = world.components();
	let is_resource_type_id = std::any::TypeId::of::<IsResource>();

	// Filter entities:
	// - observer/internal entities: have raw components but none are reflect-able and not a resource
	// - empty entities (zero components) — allow through (user-created)
	// - resource entities — show in a separate category with the type name
	let entity_list: Vec<EntityInfo> = entity_data
		.into_iter()
		.filter_map(|(id, name, parent, comp_ids, is_resource)| {
			if is_resource {
				// Try Reflect short_path first, fall back to ComponentInfo::name()
				let resource_name = comp_ids.iter().find_map(|comp_id| {
					let info = components_info.get_info(*comp_id)?;
					let type_id = info.type_id()?;
					if type_id == is_resource_type_id {
						return None;
					}
					// Reflect path (not always available)
					if let Some(reg) = registry.get(type_id) {
						return Some(reg.type_info().type_path_table().short_path().to_string());
					}
					// Fallback: ComponentInfo::name() is always available — take the part after the last ::
					let full_name = info.name().to_string();
					let short = full_name.rsplit("::").next().unwrap_or(&full_name).to_string();
					Some(short)
				});
				return Some(EntityInfo {
					id,
					name: resource_name.unwrap_or_else(|| format!("Resource {}", id)),
					parent: None,
					kind: EntityKind::Resource,
				});
			}

			// Regular entities: filter out internal ones (have raw components but none are reflect-able)
			let has_any_reflected = comp_ids.is_empty()
				|| comp_ids.iter().any(|comp_id| {
					components_info
						.get_info(*comp_id)
						.and_then(|info| info.type_id())
						.and_then(|type_id| registry.get(type_id))
						.is_some()
				});

			if !has_any_reflected {
				return None;
			}

			Some(EntityInfo {
				id,
				name: name.unwrap_or_else(|| format!("Entity {}", id)),
				parent,
				kind: EntityKind::Entity,
			})
		})
		.collect();

	drop(registry);
	entity_list
}

fn send_game_ready_and_entities(world: &mut World) {
	let protocol = {
		let multiplexer = world.resource::<ResMultiplexer>();
		multiplexer.multiplexer.register_protocol::<BepProtocol>()
	};

	protocol.game_ready();

	let entity_list = collect_entity_list(world);
	protocol.update_entities_list(entity_list);

	world.insert_resource(BepResource { protocol });
}

fn watch_entity_list(world: &mut World, mut last: Local<(usize, u64)>) {
	let mut count = 0usize;
	let mut id_xor = 0u64;
	for e in world.iter_entities() {
		count += 1;
		id_xor ^= e.id().to_bits();
	}

	if *last == (count, id_xor) {
		return;
	}
	*last = (count, id_xor);

	let Some(protocol) = world.get_resource::<BepResource>().map(|b| b.protocol.clone()) else {
		return;
	};
	let entity_list = collect_entity_list(world);
	protocol.update_entities_list(entity_list);
}

fn poll_bep_messages(world: &mut World) {
	use bridge::protocol::bep::BepMessage;

	let Some(protocol) = world.get_resource::<BepResource>().map(|r| r.protocol.clone()) else {
		return;
	};

	while let Ok(Some(msg)) = protocol.connection.try_recv() {
		match msg {
			BepMessage::SelectEntity { entity: id } => {
				if let Some(mut gizmo) = world.get_resource_mut::<GizmoTarget>() {
					gizmo.entity = id;
				}
				if let Some(mut state) = world.get_resource_mut::<InspectorState>() {
					if state.selected != id {
						state.selected = id;
						state.last_components.clear();
					}
				}
			}
			BepMessage::FocusEntity { entity: id } => {
				// Extract position first — drops the immutable world borrow before the mutable one
				let pos = world
					.iter_entities()
					.find(|e| e.id().index_u32() == id)
					.and_then(|e| e.get::<GlobalTransform>())
					.map(|gt| gt.translation());
				if let Some(pos) = pos {
					if let Some(mut req) = world.get_resource_mut::<CameraFocusRequest>() {
						req.position = Some(pos);
					}
				}
				if let Some(mut gizmo) = world.get_resource_mut::<GizmoTarget>() {
					gizmo.entity = Some(id);
				}
			}
			_ => {}
		}
	}
}

fn watch_selected_entity(world: &mut World) {
	let (selected, protocol) = {
		let state = world.get_resource::<InspectorState>();
		let bep = world.get_resource::<BepResource>();
		match (state, bep) {
			(Some(s), Some(b)) => (s.selected, b.protocol.clone()),
			_ => return,
		}
	};

	let Some(entity_id) = selected else {
		return;
	};

	let components = collect_entity_components(world, entity_id);

	let state = world.get_resource_mut::<InspectorState>();
	if let Some(mut state) = state {
		if components != state.last_components {
			protocol.entity_components_update(entity_id, components.clone());
			state.last_components = components;
		}
	}
}

fn reflect_to_fields(reflected: &dyn Reflect) -> Vec<FieldData> {
	use bevy::reflect::ReflectRef;

	// Special case: bevy::core::Name — show the string directly
	if let Some(name) = reflected.as_any().downcast_ref::<bevy::prelude::Name>() {
		return vec![FieldData {
			name: "name".to_string(),
			field_type: "String".to_string(),
			value: FieldValue::String(name.as_str().to_string()),
		}];
	}

	match reflected.reflect_ref() {
		ReflectRef::Struct(s) => (0..s.field_len())
			.filter_map(|i| {
				let field = s.field_at(i)?;
				Some(FieldData {
					name: s.name_at(i).unwrap_or("?").to_string(),
					field_type: field.reflect_type_path().to_string(),
					value: partial_reflect_to_value(field),
				})
			})
			.collect(),
		ReflectRef::TupleStruct(ts) => (0..ts.field_len())
			.filter_map(|i| {
				let field = ts.field(i)?;
				Some(FieldData {
					name: i.to_string(),
					field_type: field.reflect_type_path().to_string(),
					value: partial_reflect_to_value(field),
				})
			})
			.collect(),
		ReflectRef::Enum(e) => {
			// Represent the current variant as a single synthetic field
			let inner_fields: Vec<FieldData> = (0..e.field_len())
				.filter_map(|i| {
					let field = e.field_at(i)?;
					let name = e.name_at(i).map(|s| s.to_string()).unwrap_or_else(|| i.to_string());
					Some(FieldData {
						name,
						field_type: field.reflect_type_path().to_string(),
						value: partial_reflect_to_value(field),
					})
				})
				.collect();

			let variant_value = if inner_fields.is_empty() {
				FieldValue::Enum {
					variant: e.variant_name().to_string(),
					value: None,
				}
			} else {
				FieldValue::Enum {
					variant: e.variant_name().to_string(),
					value: Some(Box::new(FieldValue::Struct(inner_fields))),
				}
			};
			vec![FieldData {
				name: "variant".to_string(),
				field_type: reflected.reflect_type_path().to_string(),
				value: variant_value,
			}]
		}
		_ => vec![],
	}
}

fn partial_reflect_to_value(value: &dyn bevy::reflect::PartialReflect) -> FieldValue {
	use bevy::reflect::ReflectRef;

	// Try downcasting to known primitive/math types via Reflect (full type info)
	if let Some(r) = value.try_as_reflect() {
		let any = r.as_any();
		if let Some(v) = any.downcast_ref::<bool>() {
			return FieldValue::Bool(*v);
		}
		if let Some(v) = any.downcast_ref::<f32>() {
			return FieldValue::F32(*v);
		}
		if let Some(v) = any.downcast_ref::<f64>() {
			return FieldValue::F64(*v);
		}
		if let Some(v) = any.downcast_ref::<i32>() {
			return FieldValue::I32(*v);
		}
		if let Some(v) = any.downcast_ref::<u32>() {
			return FieldValue::U32(*v);
		}
		if let Some(v) = any.downcast_ref::<i64>() {
			return FieldValue::I64(*v);
		}
		if let Some(v) = any.downcast_ref::<u64>() {
			return FieldValue::U64(*v);
		}
		if let Some(v) = any.downcast_ref::<u8>() {
			return FieldValue::U32(*v as u32);
		}
		if let Some(v) = any.downcast_ref::<u16>() {
			return FieldValue::U32(*v as u32);
		}
		if let Some(v) = any.downcast_ref::<i8>() {
			return FieldValue::I32(*v as i32);
		}
		if let Some(v) = any.downcast_ref::<i16>() {
			return FieldValue::I32(*v as i32);
		}
		if let Some(v) = any.downcast_ref::<usize>() {
			return FieldValue::U64(*v as u64);
		}
		if let Some(v) = any.downcast_ref::<isize>() {
			return FieldValue::I64(*v as i64);
		}
		if let Some(v) = any.downcast_ref::<String>() {
			return FieldValue::String(v.clone());
		}
		if let Some(v) = any.downcast_ref::<Vec2>() {
			return FieldValue::Vec2 { x: v.x, y: v.y };
		}
		if let Some(v) = any.downcast_ref::<Vec3>() {
			return FieldValue::Vec3 { x: v.x, y: v.y, z: v.z };
		}
		if let Some(v) = any.downcast_ref::<Vec4>() {
			return FieldValue::Vec4 {
				x: v.x,
				y: v.y,
				z: v.z,
				w: v.w,
			};
		}
		if let Some(v) = any.downcast_ref::<Quat>() {
			return FieldValue::Quat {
				x: v.x,
				y: v.y,
				z: v.z,
				w: v.w,
			};
		}
		if let Some(v) = any.downcast_ref::<bevy::color::LinearRgba>() {
			return FieldValue::Color {
				r: v.red,
				g: v.green,
				b: v.blue,
				a: v.alpha,
			};
		}
	}

	// Recurse into composite types
	match value.reflect_ref() {
		ReflectRef::Struct(s) => {
			let fields = (0..s.field_len())
				.filter_map(|i| {
					let field = s.field_at(i)?;
					Some(FieldData {
						name: s.name_at(i).unwrap_or("?").to_string(),
						field_type: field.reflect_type_path().to_string(),
						value: partial_reflect_to_value(field),
					})
				})
				.collect();
			FieldValue::Struct(fields)
		}
		ReflectRef::Enum(e) => FieldValue::Enum {
			variant: e.variant_name().to_string(),
			value: None,
		},
		ReflectRef::List(l) => {
			let values = (0..l.len()).filter_map(|i| l.get(i)).map(partial_reflect_to_value).collect();
			FieldValue::List(values)
		}
		_ => FieldValue::Unknown(serde_json::Value::Null),
	}
}

fn collect_entity_components(world: &World, entity_id: u32) -> Vec<ComponentData> {
	let type_registry = world.resource::<AppTypeRegistry>().clone();
	let registry = type_registry.read();

	let Some(entity_ref) = world.iter_entities().find(|e| e.id().index_u32() == entity_id) else {
		return vec![];
	};

	entity_ref
		.archetype()
		.components()
		.iter()
		.filter_map(|comp_id| {
			let comp_info = world.components().get_info(*comp_id)?;
			let type_id = comp_info.type_id()?;
			let registration = registry.get(type_id)?;
			let type_info = registration.type_info();

			let reflect_component = registration.data::<ReflectComponent>()?;
			let reflected = reflect_component.reflect(entity_ref)?;

			Some(ComponentData {
				type_name: type_info.type_path().to_string(),
				short_name: type_info.type_path_table().short_path().to_string(),
				fields: reflect_to_fields(reflected),
			})
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use bridge::multiplexer::Multiplexer;

	#[test]
	fn test_channel_id() {
		let channel_id = Multiplexer::<tokio::io::DuplexStream, tokio::io::DuplexStream>::channel_id_for_type::<BepProtocol>();
		eprintln!("BepProtocol Channel ID: {:#018x}", channel_id);
	}
}
