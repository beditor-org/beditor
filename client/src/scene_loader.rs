use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for procedural meshes
#[derive(Component, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub enum ProceduralMesh {
	Cube { size: f32 },
	Plane { width: f32, height: f32 },
	Sphere { radius: f32 },
}

/// Marker component for materials
#[derive(Component, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Component, Serialize, Deserialize)]
pub struct ColorMaterial {
	pub color: [f32; 3],
}

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<ProceduralMesh>()
			.register_type::<ColorMaterial>()
			.add_systems(Update, materialize_procedural_meshes);
	}
}

/// System that converts ProceduralMesh components into actual Mesh3d + Material
fn materialize_procedural_meshes(
	mut commands: Commands,
	query: Query<(Entity, &ProceduralMesh, Option<&ColorMaterial>), (Without<Mesh3d>, Without<Handle<Mesh>>)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for (entity, procedural, color_mat) in query.iter() {
		let mesh = match procedural {
			ProceduralMesh::Cube { size } => meshes.add(Cuboid::new(*size, *size, *size)),
			ProceduralMesh::Plane { width, height } => meshes.add(Plane3d::default().mesh().size(*width, *height)),
			ProceduralMesh::Sphere { radius } => meshes.add(Sphere::new(*radius).mesh().ico(5).unwrap()),
		};

		let color = if let Some(mat) = color_mat {
			Color::srgb(mat.color[0], mat.color[1], mat.color[2])
		} else {
			Color::WHITE
		};

		commands.entity(entity).insert((Mesh3d(mesh), MeshMaterial3d(materials.add(color))));
	}
}
