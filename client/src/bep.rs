use bevy::{ecs::resource::IsResource, prelude::*};
use bridge::protocol::bep::{BepProtocol, ComponentData, EntityInfo, EntityKind};

use crate::app::ResMultiplexer;

#[derive(Resource)]
pub struct BepResource {
	pub protocol: BepProtocol,
}

pub struct BepPlugin;
impl Plugin for BepPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PostStartup, send_game_ready_and_entities)
			.add_systems(Update, poll_bep_messages);
	}
}

fn send_game_ready_and_entities(world: &mut World) {
	let protocol = {
		let multiplexer = world.resource::<ResMultiplexer>();
		multiplexer.multiplexer.register_protocol::<BepProtocol>()
	};

	protocol.game_ready();

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

	protocol.update_entities_list(entity_list);

	world.insert_resource(BepResource { protocol });
}

fn poll_bep_messages(world: &mut World) {
	use bridge::protocol::bep::BepMessage;

	let Some(protocol) = world.get_resource::<BepResource>().map(|r| r.protocol.clone()) else {
		return;
	};

	while let Ok(Some(msg)) = protocol.connection.try_recv() {
		match msg {
			BepMessage::SelectEntity { entity: id } => {
				let components = collect_entity_components(world, id);
				protocol.entity_components_update(id, components);
			}
			_ => {}
		}
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

			Some(ComponentData {
				type_name: type_info.type_path().to_string(),
				short_name: type_info.type_path_table().short_path().to_string(),
				fields: vec![],
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
