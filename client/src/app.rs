use std::{
	io::{stdin, stdout, Stdin, Stdout},
	time::Duration,
};

use bevy::{
	app::{PluginGroupBuilder, ScheduleRunnerPlugin},
	camera::RenderTarget,
	prelude::*,
	remote::RemotePlugin,
	window::ExitCondition,
	winit::WinitPlugin,
};
use bridge::{
	codec::{base64::Base64Codec, json::JsonCodec},
	connection::Connection,
	multiplexer::Multiplexer,
	protocol::{camera::CameraInputProtocol, frame_stream::FrameStreamProtocol},
};
use clap::Parser;

use flume::{unbounded, Receiver, RecvTimeoutError, Sender};
use serde::{Deserialize, Serialize};

use crate::{frame_capture::FrameCapturePlugin, BrpProtocolPlugin};

/// Marker component from camera to render game to editor viewport
#[derive(Component)]
pub struct EditorCamera;

/// Stores the camera rotation as Euler angles (in radians)
#[derive(Component)]
pub struct CameraRotation {
	pub pitch: f32, // rotation around X axis (up/down)
	pub yaw: f32,   // rotation around Y axis (left/right)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Run in editor mode
	#[arg(long)]
	editor_mode: bool,
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

pub fn setup_editor_camera(
	mut commands: Commands,
	mut cameras: Query<(Entity, &mut Camera), With<EditorCamera>>,
	mut images: ResMut<Assets<Image>>,
) {
	let Ok((entity, mut camera)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

	// Initialize camera rotation component
	commands.entity(entity).insert(CameraRotation { pitch: 0.0, yaw: 0.0 });

	let size = bevy::render::render_resource::Extent3d {
		width: 640,
		height: 480,
		..Default::default()
	};

	let mut render_target_image = Image::new_target_texture(
		size.width,
		size.height,
		bevy::render::render_resource::TextureFormat::bevy_default(),
	);
	render_target_image.texture_descriptor.usage |= bevy::render::render_resource::TextureUsages::COPY_SRC;

	let image_handle = images.add(render_target_image);
	camera.target = RenderTarget::Image(image_handle.clone().into());

	eprintln!(
		"📷 Configured camera {:?} for editor capture ({}x{})",
		entity, size.width, size.height
	);
}

pub fn controll_editor_camera(
	controls_stream: Res<ControlsStream>,
	mut cameras: Query<(Entity, &mut Transform, &mut CameraRotation), With<EditorCamera>>,
) {
	let Ok((entity, mut transform, mut rotation)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

	while let Ok(json_str) = controls_stream.rx.try_recv() {
		info!("🎮 Controlling editor camera {:?} with data: {}", entity, json_str);
		match serde_json::from_str::<ControlsEvent>(&json_str) {
			Ok(event) => {
				// Sensitivity factor (smaller = less sensitive)
				let sensitivity = 0.01;

				// Update Euler angles
				rotation.yaw -= event.x * sensitivity; // horizontal mouse movement
				rotation.pitch -= event.y * sensitivity; // vertical mouse movement

				// Clamp pitch to avoid gimbal lock
				rotation.pitch = rotation
					.pitch
					.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);

				// Convert Euler angles to quaternion
				transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.yaw, rotation.pitch, 0.0);

				info!(
					"✅ Camera rotation applied - pitch: {:.2}, yaw: {:.2}",
					rotation.pitch, rotation.yaw
				);
			}
			Err(e) => {
				eprintln!("❌ Failed to parse camera control JSON: {:?}", e);
			}
		}
	}
}

#[derive(Resource)]
pub struct ResMultiplexer {
	pub multiplexer: Multiplexer<Stdin, Stdout>,
}

#[derive(Resource)]
pub struct ViewportStream {
	pub rx: Receiver<String>,
	pub tx: Sender<String>,
}

#[derive(Resource)]
pub struct ControlsStream {
	pub rx: Receiver<String>,
	pub tx: Sender<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlsEvent {
	x: f32,
	y: f32,
}
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
			let mut multiplexer = Multiplexer::new(stdin(), stdout());

			let viewport_reader = multiplexer.register_for_type::<FrameStreamProtocol>();
			let viewport_writer = multiplexer.get_writer_for_type::<FrameStreamProtocol>();

			multiplexer.start();

			let (viewport_sender, viewport_receiver) = unbounded::<String>();

			let vr = viewport_receiver.clone();
			std::thread::spawn(move || {
				let mut viewport_stream = Connection::new(Base64Codec, viewport_reader, viewport_writer);

				while let Ok(msg) = vr.recv() {
					viewport_stream.send(msg);
				}
			});

			let controls_reader = multiplexer.register_for_type::<CameraInputProtocol>();
			let controls_writer = multiplexer.get_writer_for_type::<CameraInputProtocol>();
			let (controls_sender, controls_receiver) = unbounded::<String>();
			std::thread::spawn({
				let controls_sender = controls_sender.clone();
				move || {
					let mut camera_stream = Connection::new(JsonCodec, controls_reader, controls_writer);
					loop {
						match camera_stream.reader.recv_timeout(std::time::Duration::from_millis(100)) {
							Ok(data) => match String::from_utf8(data) {
								Ok(json_str) => {
									info!("📹 Camera event received: {}", json_str);
									controls_sender.send(json_str).unwrap();
								}
								Err(e) => eprintln!("❌ Invalid UTF-8 in camera event: {:?}", e),
							},
							Err(RecvTimeoutError::Timeout) => {
								continue;
							}
							Err(_) => {
								eprintln!("❌ Camera stream disconnected");
								break;
							}
						}
					}
				}
			});

			self.add_plugins(RemotePlugin::default())
				.add_plugins(ScheduleRunnerPlugin::run_loop(
					// Run 60 times per second.
					Duration::from_secs_f64(1.0 / 60.0),
				))
				.add_plugins((FrameCapturePlugin, BrpProtocolPlugin))
				.insert_resource(ResMultiplexer { multiplexer })
				.insert_resource(ViewportStream {
					rx: viewport_receiver.clone(),
					tx: viewport_sender,
				})
				.insert_resource(ControlsStream {
					rx: controls_receiver.clone(),
					tx: controls_sender,
				})
				.add_systems(PostStartup, setup_editor_camera)
				.add_systems(Update, controll_editor_camera);
		}
		self
	}

	fn with_default_plugins(&mut self, default_plugins: impl IntoEditorPluginGroup) -> &mut Self {
		if self.is_editor_mode() {
			self.add_plugins(
				DefaultPlugins
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
