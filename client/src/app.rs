use std::time::Duration;

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
use flume::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use tokio::io::{stdin, stdout, Stdin, Stdout};

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
	let Ok((_entity, mut transform, mut rotation)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

	let sensitivity = 0.01;
	while let Ok(Some(event)) = controls_stream.mouse.try_recv() {
		rotation.yaw -= event.x * sensitivity;
		rotation.pitch = (rotation.pitch - event.y * sensitivity)
			.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
		transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.yaw, rotation.pitch, 0.0);
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
	mouse: Connection<JsonCodec<ControlsEvent>>,
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

			let (viewport_reader, viewport_writer) = multiplexer.register_for_type::<FrameStreamProtocol>();

			multiplexer.start();

			let (viewport_sender, viewport_receiver) = unbounded::<String>();

			let vr = viewport_receiver.clone();
			std::thread::spawn(move || {
				let mut viewport_stream = Connection::<Base64Codec>::new(viewport_reader, viewport_writer);

				while let Ok(msg) = vr.recv() {
					viewport_stream.send(&msg);
				}
			});

			// read camera controls from editor
			let (controls_reader, controls_writer) = multiplexer.register_for_type::<CameraInputProtocol>();
			let controls_connection = Connection::<JsonCodec<ControlsEvent>>::new(controls_reader, controls_writer);

			self.add_plugins(RemotePlugin::default())
				.insert_resource(ResMultiplexer { multiplexer })
				.insert_resource(ViewportStream {
					rx: viewport_receiver.clone(),
					tx: viewport_sender,
				})
				.insert_resource(ControlsStream {
					mouse: controls_connection,
				})
				.add_plugins(ScheduleRunnerPlugin::run_loop(
					// Run 60 times per second.
					Duration::from_secs_f64(1.0 / 60.0),
				))
				.add_plugins((FrameCapturePlugin, BrpProtocolPlugin))
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
