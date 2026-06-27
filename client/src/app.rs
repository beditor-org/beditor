use std::time::Duration;

use bevy::{
	app::{PluginGroupBuilder, ScheduleRunnerPlugin},
	camera::RenderTarget,
	dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin},
	prelude::*,
	remote::RemotePlugin,
	window::ExitCondition,
	winit::WinitPlugin,
};
use bridge::{
	codec::json::JsonCodec,
	connection::Connection,
	multiplexer::Multiplexer,
	protocol::{
		camera::{CameraInputProtocol, MouseEvent},
		frame_stream::FrameStreamProtocol,
	},
};
use clap::Parser;
use tokio::io::{stdin, stdout, Stdin, Stdout};

use crate::{frame_capture::FrameCapturePlugin, BepPlugin};

/// Marker component from camera to render game to editor viewport
#[derive(Component)]
pub struct EditorCamera;

/// Orbit camera state: camera sits at `pivot + rotation(yaw,pitch) * Z * distance`
#[derive(Component)]
pub struct CameraRotation {
	pub pitch: f32,    // rotation around X axis (up/down)
	pub yaw: f32,      // rotation around Y axis (left/right)
	pub pivot: Vec3,   // the point the camera orbits around
	pub distance: f32, // distance from pivot to camera
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Run in editor mode
	#[arg(long)]
	editor_mode: bool,
	/// Shared memory file path for viewport frames (created by editor, passed on launch)
	#[arg(long)]
	viewport_shm: Option<String>,
}

pub trait IntoEditorPluginGroup {
	fn into_editor_builder(self) -> PluginGroupBuilder;
}

impl IntoEditorPluginGroup for DefaultPlugins {
	fn into_editor_builder(self) -> PluginGroupBuilder {
		self.build()
	}
}

impl IntoEditorPluginGroup for PluginGroupBuilder {
	fn into_editor_builder(self) -> PluginGroupBuilder {
		self
	}
}

pub fn setup_infinite_grid(mut commands: Commands) {
	commands.spawn(InfiniteGrid);
}

pub fn setup_editor_camera(
	mut commands: Commands,
	mut cameras: Query<(Entity, &mut Camera, &Transform), With<EditorCamera>>,
	mut images: ResMut<Assets<Image>>,
) {
	let Ok((entity, _camera, transform)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

	// Derive initial orbit state from the camera's existing transform.
	// pivot = world origin; distance = length of camera's current translation.
	let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
	let distance = transform.translation.length().max(1.0);
	commands.entity(entity).insert(CameraRotation {
		pitch,
		yaw,
		pivot: Vec3::ZERO,
		distance,
	});

	let size = bevy::render::render_resource::Extent3d {
		width: 1280,
		height: 720,
		..Default::default()
	};

	let mut render_target_image = Image::new_target_texture(
		size.width,
		size.height,
		bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
		None,
	);
	render_target_image.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;

	let image_handle = images.add(render_target_image);
	commands
		.entity(entity)
		.insert(RenderTarget::Image(image_handle.clone().into()));

	eprintln!(
		"📷 Configured camera {:?} for editor capture ({}x{})",
		entity, size.width, size.height
	);
}

pub fn controll_editor_camera(
	controls_stream: Res<ControlsStream>,
	mut cameras: Query<(Entity, &mut Transform, &mut CameraRotation), With<EditorCamera>>,
) {
	let Ok((_entity, mut transform, mut rotation)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

	// Recompute camera transform from current orbit state.
	// Camera sits at: pivot + rotation(yaw,pitch) * Vec3::Z * distance
	let apply = |transform: &mut Transform, r: &CameraRotation| {
		let rot = Quat::from_euler(EulerRot::YXZ, r.yaw, r.pitch, 0.0);
		transform.translation = r.pivot + rot * Vec3::new(0.0, 0.0, r.distance);
		transform.rotation = rot;
	};

	let orbit_sensitivity = 0.005;
	let dolly_speed = 0.1;
	// Pan speed scales with distance so it feels consistent at any zoom level
	let pan_speed = 0.001;

	while let Ok(Some(event)) = controls_stream.mouse.try_recv() {
		if event.scroll != 0.0 {
			// Dolly: change distance to pivot, clamp to avoid flipping through it
			rotation.distance = (rotation.distance + event.scroll * dolly_speed * rotation.distance).max(0.1);
			apply(&mut transform, &rotation);
		}
		if event.pan_x != 0.0 || event.pan_y != 0.0 {
			// Pan: move pivot along camera right/up — camera follows
			let right = transform.right();
			let up = transform.up();
			let speed = pan_speed * rotation.distance;
			rotation.pivot -= right * event.pan_x * speed;
			rotation.pivot += up * event.pan_y * speed;
			apply(&mut transform, &rotation);
		}
		if event.x != 0.0 || event.y != 0.0 {
			// Orbit: rotate around pivot
			rotation.yaw -= event.x * orbit_sensitivity;
			rotation.pitch = (rotation.pitch - event.y * orbit_sensitivity)
				.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
			apply(&mut transform, &rotation);
		}
	}
}

#[derive(Resource)]
pub struct ResMultiplexer {
	pub multiplexer: Multiplexer<Stdin, Stdout>,
}

#[derive(Resource)]
pub struct ControlsStream {
	mouse: Connection<JsonCodec<MouseEvent>>,
}

/// Sender end of the viewport frame channel.
/// `send_encoded_frames` puts JPEG bytes here; a background thread writes
/// them to the shared-memory file and signals the editor via FrameStreamProtocol.
#[derive(Resource)]
pub struct ViewportSender(pub flume::Sender<Vec<u8>>);

pub trait EditorApp {
	fn with_default_plugins(&mut self, default_plugins: impl IntoEditorPluginGroup) -> &mut Self;
	fn with_editor_plugins(&mut self) -> &mut Self;
	fn is_editor_mode(&self) -> bool {
		Args::parse().editor_mode
	}
}

impl EditorApp for App {
	fn with_editor_plugins(&mut self) -> &mut Self {
		if self.is_editor_mode() {
			let args = Args::parse();
			let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
			let _guard = rt.enter();
			let mut multiplexer = Multiplexer::new(stdin(), stdout());
			multiplexer.start();

			let frame_stream = multiplexer.register_protocol::<FrameStreamProtocol>();
			let controls = multiplexer.register_protocol::<CameraInputProtocol>();

			// JPEG frames go to a background thread that writes them to shared memory
			// and sends a tiny notification through FrameStreamProtocol (stdio/multiplexer).
			// The editor maps the same file and reads the frame on notification.
			let (frame_tx, frame_rx) = flume::bounded::<Vec<u8>>(4);
			if let Some(shm_path) = args.viewport_shm {
				let notify_conn = frame_stream.connection;
				std::thread::spawn(move || {
					use memmap2::MmapMut;
					let file = std::fs::OpenOptions::new()
						.read(true)
						.write(true)
						.open(&shm_path)
						.expect("Failed to open viewport shm");
					let mut mmap = unsafe { MmapMut::map_mut(&file).expect("Failed to mmap viewport shm") };

					while let Ok(jpeg) = frame_rx.recv() {
						let len = jpeg.len();
						if 4 + len > mmap.len() {
							continue; // frame too large for shm — should not happen with 4 MB
						}
						// Write [len: u32 BE][jpeg data] to shm
						mmap[0..4].copy_from_slice(&(len as u32).to_be_bytes());
						mmap[4..4 + len].copy_from_slice(&jpeg);
						// Signal editor: frame is ready in shm
						let _ = notify_conn.send(&vec![]);
					}
				});
			}

			// Keep runtime alive for the duration of the app
			std::mem::forget(rt);

			self.add_plugins(RemotePlugin::default())
				.insert_resource(ResMultiplexer { multiplexer })
				.insert_resource(ViewportSender(frame_tx))
				.insert_resource(ControlsStream {
					mouse: controls.connection,
				})
				.add_plugins(ScheduleRunnerPlugin::run_loop(
					// Run 60 times per second.
					Duration::from_secs_f64(1.0 / 60.0),
				))
				.add_plugins((FrameCapturePlugin, BepPlugin, InfiniteGridPlugin))
				.add_systems(PostStartup, setup_editor_camera)
				.add_systems(Startup, setup_infinite_grid)
				.add_systems(Update, controll_editor_camera);
		}
		self
	}

	fn with_default_plugins(&mut self, default_plugins: impl IntoEditorPluginGroup) -> &mut Self {
		if self.is_editor_mode() {
			let asset_path = std::env::var("BEDITOR_ASSET_PATH").unwrap_or_else(|_| "assets".to_string());
			self.add_plugins(
				DefaultPlugins
					.set(AssetPlugin {
						file_path: asset_path,
						..default()
					})
					.set(ImagePlugin::default_nearest())
					// Not strictly necessary, as the inclusion of ScheduleRunnerPlugin below
					// replaces the bevy_winit app runner and so a window is never created.
					.set(WindowPlugin {
						primary_window: None,
						// Don’t automatically exit due to having no windows.
						// Instead, the code in `update()` will explicitly produce an `AppExit` event.
						exit_condition: ExitCondition::DontExit,
						..default()
					})
					// WinitPlugin will panic in environments without a display server.
					.disable::<WinitPlugin>(),
			);
		} else {
			self.add_plugins(default_plugins.into_editor_builder());
		};
		self
	}
}
