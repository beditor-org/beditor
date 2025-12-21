use std::time::Duration;

use bevy::{
	app::{PluginGroupBuilder, ScheduleRunnerPlugin},
	camera::RenderTarget,
	prelude::*,
	remote::RemotePlugin,
	window::ExitCondition,
	winit::WinitPlugin,
};
use clap::Parser;

use crate::{frame_capture::FrameCapturePlugin, BrpProtocolPlugin};

/// Marker component from camera to render game to editor viewport
#[derive(Component)]
pub struct EditorCamera;

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

pub fn setup_editor_camera(mut cameras: Query<(Entity, &mut Camera), With<EditorCamera>>, mut images: ResMut<Assets<Image>>) {
	let Ok((entity, mut camera)) = cameras.single_mut() else {
		eprintln!("⚠️  Warning: No EditorCamera found or multiple cameras marked with EditorCamera");
		return;
	};

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

pub trait EditorApp {
	fn with_default_plugins(&mut self, default_plugins: impl IntoEditorPluginGroup) -> &mut Self;
	fn with_editor_plugins(&mut self) -> &mut Self;
	fn is_editor_mode(&self) -> bool {
		Args::parse().editor_mode
	}
}

impl EditorApp for App {
	fn with_editor_plugins(&mut self) -> &mut Self {
		self.add_plugins(RemotePlugin::default())
			.add_plugins(ScheduleRunnerPlugin::run_loop(
				// Run 60 times per second.
				Duration::from_secs_f64(1.0 / 60.0),
			))
			.add_plugins((FrameCapturePlugin, BrpProtocolPlugin))
			.add_systems(PostStartup, setup_editor_camera);
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
