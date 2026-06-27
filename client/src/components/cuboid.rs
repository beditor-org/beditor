use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CuboidMesh {
	pub size: Vec3,
	pub color: Color,
}

pub fn cuboid_mesh(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<(Entity, &CuboidMesh), Changed<CuboidMesh>>,
) {
	for (entity, desc) in &query {
		commands.entity(entity).insert((
			Mesh3d(meshes.add(Cuboid::new(desc.size.x, desc.size.y, desc.size.z))),
			MeshMaterial3d(materials.add(StandardMaterial {
				base_color: desc.color,
				..default()
			})),
			Visibility::default(),
		));
	}
}
