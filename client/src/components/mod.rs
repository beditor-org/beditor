use bevy::prelude::*;

pub mod cuboid;
pub mod robot;

pub struct SceneComponentsPlugin;

impl Plugin for SceneComponentsPlugin {
	fn build(&self, app: &mut App) {
		app.register_type::<cuboid::CuboidMesh>()
			.add_systems(Update, cuboid::cuboid_mesh)
			.register_type::<robot::RobotMesh>()
			.add_systems(Update, robot::robot_mesh);
	}
}
