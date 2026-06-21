use bevy::prelude::*;
use client::{EditorApp, EditorCamera};

fn main() {
	let mut app = App::new();
	app.with_default_plugins(
		DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "🎮 Game Viewport".to_string(),
					mode: bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current),
					..default()
				}),
				..default()
			})
			.set(bevy::log::LogPlugin {
				level: bevy::log::Level::ERROR,
				filter: "warn".to_string(),
				..default()
			})
			.set(AssetPlugin {
				file_path: "../../../client/examples".to_string(),
				..default()
			}),
	)
	.with_editor_plugins()
	.add_systems(Startup, (load_scene, add_meshes))
	.add_systems(Update, bounce_ball)
	.run();
}

#[derive(Component)]
struct BouncingBall;

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Load scene from file - only transforms and light
	let scene_handle: Handle<DynamicWorld> = asset_server.load("demo_scene.scn.ron");
	commands.spawn(DynamicWorldRoot(scene_handle));
}

fn add_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Ground
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
		Transform::from_xyz(0.0, 0.0, 0.0),
		Name::new("Ground"),
	));

	// Robot — parent entity with children
	let robot = commands
		.spawn((Transform::from_xyz(0.0, 0.0, 0.0), Visibility::default(), Name::new("Robot")))
		.id();

	let body = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 0.6))),
			MeshMaterial3d(materials.add(Color::srgb(0.4, 0.6, 0.9))),
			Transform::from_xyz(0.0, 1.5, 0.0),
			Name::new("Body"),
		))
		.id();

	let head = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
			MeshMaterial3d(materials.add(Color::srgb(0.9, 0.8, 0.5))),
			Transform::from_xyz(0.0, 1.1, 0.0),
			Name::new("Head"),
		))
		.id();

	let left_eye = commands
		.spawn((
			Mesh3d(meshes.add(Sphere::new(0.1))),
			MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
			Transform::from_xyz(-0.2, 0.1, 0.36),
			Name::new("Left Eye"),
		))
		.id();

	let right_eye = commands
		.spawn((
			Mesh3d(meshes.add(Sphere::new(0.1))),
			MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
			Transform::from_xyz(0.2, 0.1, 0.36),
			Name::new("Right Eye"),
		))
		.id();

	let left_arm = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(0.3, 1.2, 0.3))),
			MeshMaterial3d(materials.add(Color::srgb(0.4, 0.6, 0.9))),
			Transform::from_xyz(-0.75, 0.0, 0.0),
			Name::new("Left Arm"),
		))
		.id();

	let right_arm = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(0.3, 1.2, 0.3))),
			MeshMaterial3d(materials.add(Color::srgb(0.4, 0.6, 0.9))),
			Transform::from_xyz(0.75, 0.0, 0.0),
			Name::new("Right Arm"),
		))
		.id();

	// Build hierarchy: Robot > Body > (Head > (Left Eye, Right Eye), Left Arm, Right Arm)
	commands.entity(head).add_children(&[left_eye, right_eye]);
	commands.entity(body).add_children(&[head, left_arm, right_arm]);
	commands.entity(robot).add_children(&[body]);

	// A separate tree: Lamp post
	let lamp_post = commands
		.spawn((
			Transform::from_xyz(4.0, 0.0, 0.0),
			Visibility::default(),
			Name::new("Lamp Post"),
		))
		.id();

	let pole = commands
		.spawn((
			Mesh3d(meshes.add(Cuboid::new(0.15, 4.0, 0.15))),
			MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
			Transform::from_xyz(0.0, 2.0, 0.0),
			Name::new("Pole"),
		))
		.id();

	let lamp = commands
		.spawn((
			PointLight {
				intensity: 2000.0,
				shadow_maps_enabled: true,
				..default()
			},
			Transform::from_xyz(0.0, 4.2, 0.0),
			Name::new("Lamp"),
		))
		.id();

	commands.entity(lamp_post).add_children(&[pole, lamp]);

	// Main ambient light + camera
	commands.spawn((
		PointLight {
			intensity: 800.0,
			..default()
		},
		Transform::from_xyz(-4.0, 6.0, -4.0),
		Name::new("Fill Light"),
	));

	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(-4.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
		Name::new("Main Camera"),
		EditorCamera,
	));

	// Pink ball — placed to the left of the robot
	commands.spawn((
		Mesh3d(meshes.add(Sphere::new(0.5))),
		MeshMaterial3d(materials.add(StandardMaterial {
			base_color: Color::srgb(1.0, 0.08, 0.58),
			metallic: 0.0,
			perceptual_roughness: 0.3,
			..default()
		})),
		Transform::from_xyz(-3.0, 0.5, 0.0),
		Name::new("Pink Ball"),
		BouncingBall,
	));
}

fn bounce_ball(time: Res<Time>, mut query: Query<&mut Transform, With<BouncingBall>>) {
	for mut transform in query.iter_mut() {
		let t = time.elapsed_secs();
		transform.translation.y = 0.5 + 2.5 * (t * 2.5).sin().abs();
	}
}
