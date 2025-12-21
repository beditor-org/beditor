use bevy::prelude::*;
use client::{EditorApp, EditorCamera};
fn main() {
	let mut app = App::new();
	app.with_default_plugins(
		DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "🎮 Game Viewport".to_string(),
					..default()
				}),
				..default()
			})
			.set(bevy::log::LogPlugin {
				// Disable all Bevy logs to prevent stdout pollution
				// Only our eprintln! (stderr) and binary frames (stdout) will be output
				level: bevy::log::Level::ERROR,
				filter: "warn".to_string(),
				..default()
			}),
	)
	.with_editor_plugins()
	.add_systems(Startup, setup_scene)
	.add_systems(Update, rotate_cube)
	.run();
}

fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Spawn a cube
	commands.spawn((
		Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
		Transform::from_xyz(0.0, 0.5, 0.0),
		Name::new("Spinning Cube"),
	));

	// Light
	commands.spawn((
		PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0, 8.0, 4.0),
		Name::new("Main Light"),
	));

	// Camera
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		Name::new("Main Camera"),
		EditorCamera,
	));

	// Ground plane
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
		Transform::from_xyz(0.0, 0.0, 0.0),
		Name::new("Ground"),
	));
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<Name>>) {
	for mut transform in query.iter_mut() {
		// Only rotate the cube
		if transform.translation.y > 0.3 {
			transform.rotate_y(time.delta_secs() * 0.5);
		}
	}
}
