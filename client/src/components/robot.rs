use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RobotMesh {
	pub head_color: Color,
	pub body_color: Color,
	pub eye_color: Color,
	pub arm_color: Color,
}

pub fn robot_mesh(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	query: Query<(Entity, &RobotMesh), Added<RobotMesh>>,
) {
	for (entity, desc) in &query {
		// Robot — parent entity with children
		// let robot = commands
		// 	.spawn((Transform::from_xyz(0.0, 0.0, 0.0), Visibility::default(), Name::new("Robot")))
		// 	.id();

		let body = commands
			.spawn((
				Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 0.6))),
				MeshMaterial3d(materials.add(desc.body_color)),
				Transform::from_xyz(0.0, 1.5, 0.0),
				Name::new("Body"),
			))
			.id();

		let head = commands
			.spawn((
				Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
				MeshMaterial3d(materials.add(desc.head_color)),
				Transform::from_xyz(0.0, 1.1, 0.0),
				Name::new("Head"),
			))
			.id();

		let left_eye = commands
			.spawn((
				Mesh3d(meshes.add(Sphere::new(0.1))),
				MeshMaterial3d(materials.add(desc.eye_color)),
				Transform::from_xyz(-0.2, 0.1, 0.36),
				Name::new("Left Eye"),
			))
			.id();

		let right_eye = commands
			.spawn((
				Mesh3d(meshes.add(Sphere::new(0.1))),
				MeshMaterial3d(materials.add(desc.eye_color)),
				Transform::from_xyz(0.2, 0.1, 0.36),
				Name::new("Right Eye"),
			))
			.id();

		let left_arm = commands
			.spawn((
				Mesh3d(meshes.add(Cuboid::new(0.3, 1.2, 0.3))),
				MeshMaterial3d(materials.add(desc.arm_color)),
				Transform::from_xyz(-0.75, 0.0, 0.0),
				Name::new("Left Arm"),
			))
			.id();

		let right_arm = commands
			.spawn((
				Mesh3d(meshes.add(Cuboid::new(0.3, 1.2, 0.3))),
				MeshMaterial3d(materials.add(desc.arm_color)),
				Transform::from_xyz(0.75, 0.0, 0.0),
				Name::new("Right Arm"),
			))
			.id();

		// Build hierarchy: Robot > Body > (Head > (Left Eye, Right Eye), Left Arm, Right Arm)
		commands.entity(head).add_children(&[left_eye, right_eye]);
		commands.entity(body).add_children(&[head, left_arm, right_arm]);
		commands.entity(entity).add_children(&[body]);
	}
}
