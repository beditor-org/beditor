use bevy::prelude::*;
use client::{EditorApp, EditorCamera, SceneComponentsPlugin};

fn main() {
	let mut app = App::new();
	app.with_default_plugins(
		DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: "🎮 Game Viewport".to_string(),
					mode: bevy::window::WindowMode::Windowed,
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
				file_path: "./examples".to_string(),
				..default()
			}),
	)
	.with_editor_plugins()
	.add_plugins(SceneComponentsPlugin)
	.add_systems(Startup, (load_scene))
	// .add_systems(Update, (bounce_ball, flicker_lamp))
	.run();
}

#[derive(Component)]
struct BouncingBall;

#[derive(Component)]
struct TorchLight;

fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
	// Load scene from file - only transforms and light
	let scene_handle: Handle<DynamicWorld> = asset_server.load("demo_scene.scn.ron");
	commands.spawn(DynamicWorldRoot(scene_handle));

	// Main ambient light + camera
	commands.spawn(AmbientLight {
		color: Color::srgb(0.15, 0.15, 0.2),
		brightness: 600.0,
		affects_lightmapped_meshes: true,
	});

	commands.spawn((
		PointLight {
			intensity: 200_000.0,
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
}

fn add_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	// Ground
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
		Transform::from_xyz(0.0, 0.0, 0.0),
		Name::new("Ground"),
	));

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
				color: Color::srgb(1.0, 0.55, 0.1),
				intensity: 80_000.0,
				range: 20.0,
				shadow_maps_enabled: true,
				..default()
			},
			Transform::from_xyz(0.0, 6.0, 0.0),
			Name::new("Lamp"),
			TorchLight,
		))
		.id();

	commands.entity(lamp_post).add_children(&[pole, lamp]);

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

fn flicker_lamp(time: Res<Time>, mut query: Query<&mut PointLight, With<TorchLight>>) {
	for mut light in query.iter_mut() {
		let t = time.elapsed_secs();
		let flicker = 1.0 + 0.18 * (t * 11.3).sin() + 0.12 * (t * 23.7).sin() + 0.07 * (t * 47.9).cos() + 0.04 * (t * 83.1).sin();
		light.intensity = 500_000.0 * flicker.max(0.4);
	}
}
