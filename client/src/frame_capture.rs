use bevy::{
	camera::RenderTarget,
	prelude::*,
	render::{
		render_asset::RenderAssets,
		render_resource::{
			Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, TexelCopyBufferInfo,
			TexelCopyBufferLayout,
		},
		renderer::{RenderDevice, RenderQueue},
		texture::GpuImage,
		Extract, RenderApp,
	},
};
use bridge::protocol::frame_stream::FrameStreamProtocol;
use flume::{bounded, unbounded, Receiver, Sender};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::sync::{atomic::AtomicBool, Arc};

use crate::app::{ResMultiplexer, ViewportStream};

// ============================================================================
// RESOURCES - for communication between Main World and Render World
// ============================================================================

/// Receiver in Main World - receives data from Render World
/// This channel allows async frame data transfer between threads
#[derive(Resource, Deref)]
pub struct MainWorldReceiver(Receiver<Vec<u8>>);

/// Sender in Render World - sends data to Main World
/// Clone of this sender will be used in render systems to send frames
#[derive(Resource, Deref)]
pub struct RenderWorldSender(Sender<Vec<u8>>);

// ============================================================================
// PLUGIN - entry point for the entire capture system
// ============================================================================

pub struct FrameCapturePlugin;

impl Plugin for FrameCapturePlugin {
	fn build(&self, app: &mut App) {
		// Create unbounded channel (no queue size limit)
		let (sender, receiver) = unbounded();

		// Main World: add receiver and save system
		app.insert_resource(MainWorldReceiver(receiver))
			.add_systems(First, setup_cpu_image)
			.add_systems(Last, save_captured_frames);

		// Render World: setup entire render pipeline
		let render_app = app.sub_app_mut(RenderApp);

		// Add sender to Render World
		render_app.insert_resource(RenderWorldSender(sender));

		// Extract system (Main → Render copy)
		render_app.add_systems(bevy::render::ExtractSchedule, extract_image_copiers);

		// Copy texture → buffer (runs after rendering, before receive)
		render_app.add_systems(
			bevy::render::Render,
			copy_image_to_buffer.after(bevy::render::RenderSystems::Render),
		);

		// System for reading from buffer (after copying)
		render_app.add_systems(
			bevy::render::Render,
			receive_image_from_buffer
				.after(bevy::render::RenderSystems::Render)
				.after(copy_image_to_buffer),
		);

		eprintln!("✅ FrameCapturePlugin initialized");
	}
}

/// Creates CPU-side Image for saving frames (runs once)
fn setup_cpu_image(mut commands: Commands, mut images: ResMut<Assets<Image>>, existing: Query<&ImageToSave>) {
	if !existing.is_empty() {
		return; // Already created
	}
	let cpu_image = Image::new_target_texture(1280, 720, bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, None);
	let handle = images.add(cpu_image);
	commands.spawn(ImageToSave(handle));
}

// ============================================================================
// COMPONENTS - components that live on entities
// ============================================================================

/// Component that holds GPU buffer for texture copying
/// Spawned on entity together with render target image handle
#[derive(Component, Clone)]
pub struct ImageCopier {
	/// GPU buffer where we copy texture to (has BufferUsages::MAP_READ)
	pub buffer: Buffer,
	/// Handle to render target Image (source of copy)
	pub src_image: Handle<Image>,
	/// Whether this copier is active (can be disabled)
	pub enabled: Arc<AtomicBool>,
}

impl ImageCopier {
	/// Creates new ImageCopier with GPU buffer
	pub fn new(src_image: Handle<Image>, size: Extent3d, render_device: &RenderDevice) -> Self {
		// Calculate padded bytes per row (GPU requires 256-byte alignment)
		// Formula: align_to_256(width * 4 bytes_per_pixel)
		let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row((size.width as usize) * 4);

		// Create GPU buffer of sufficient size
		let cpu_buffer = render_device.create_buffer(&BufferDescriptor {
			label: Some("editor_frame_capture_buffer"),
			size: (padded_bytes_per_row * size.height as usize) as u64,
			// MAP_READ - allows reading on CPU
			// COPY_DST - allows copying here from texture
			usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});

		Self {
			buffer: cpu_buffer,
			src_image,
			enabled: Arc::new(AtomicBool::new(true)),
		}
	}

	/// Checks if copier is active
	pub fn is_enabled(&self) -> bool {
		self.enabled.load(std::sync::atomic::Ordering::Relaxed)
	}
}

/// Marker component that points to Image handle for file saving
/// This Image will be filled with data from GPU buffer
#[derive(Component, Deref, DerefMut)]
pub struct ImageToSave(pub Handle<Image>);

/// Resource in Render World - aggregates all ImageCopiers for RenderGraph access
/// Extract system collects all ImageCopiers from Main World and puts them in this Vec
#[derive(Resource, Default, Deref, DerefMut)]
pub struct ImageCopiers(pub Vec<ImageCopier>);

// ============================================================================
// EXTRACT SYSTEMS - copy data from Main World → Render World
// ============================================================================

/// Extract system that creates ImageCopier for EditorCamera and copies to Render World
/// Runs every frame before rendering
fn extract_image_copiers(
	mut commands: Commands,
	cameras: Extract<Query<(&Camera, &RenderTarget), With<super::app::EditorCamera>>>,
	render_device: Res<RenderDevice>,
) {
	let mut copiers = Vec::new();

	// For each EditorCamera create ImageCopier
	for (_camera, render_target) in cameras.iter() {
		let RenderTarget::Image(img_target) = render_target else {
			continue; // Skip cameras without Image target
		};

		let image_handle = img_target.handle.clone();
		let size = Extent3d {
			width: 1280,
			height: 720,
			..Default::default()
		};

		// Create ImageCopier IN RENDER WORLD (RenderDevice is available here!)
		let copier = ImageCopier::new(image_handle, size, &render_device);
		copiers.push(copier);
	}

	// Insert as Resource in Render World
	commands.insert_resource(ImageCopiers(copiers));
}

// ============================================================================
// RENDER SYSTEM - executes GPU copy texture → buffer (replaces RenderGraph node)
// ============================================================================

/// System that copies textures to buffers
/// Runs after RenderSystems::Render, before receive_image_from_buffer
fn copy_image_to_buffer(
	image_copiers: Res<ImageCopiers>,
	gpu_images: Res<RenderAssets<GpuImage>>,
	render_device: Res<RenderDevice>,
	render_queue: Res<RenderQueue>,
) {
	// For each copier copy its texture to buffer
	for image_copier in image_copiers.iter() {
		if !image_copier.is_enabled() {
			continue; // Skip disabled copiers
		}

		// Find GPU texture by handle
		let Some(gpu_image) = gpu_images.get(&image_copier.src_image) else {
			eprintln!("GPU image not found for handle");
			continue;
		};

		// Create command encoder for GPU commands
		let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
			label: Some("image_copy_encoder"),
		});

		// Get texture format info (block size, dimensions)
		let block_dimensions = gpu_image.texture_descriptor.format.block_dimensions();
		let block_size = gpu_image.texture_descriptor.format.block_copy_size(None).unwrap_or(4);

		// Calculate padded bytes per row (GPU requires alignment)
		let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(
			(gpu_image.texture_descriptor.size.width as usize / block_dimensions.0 as usize) * block_size as usize,
		);

		// MAIN COMMAND: copy texture → buffer
		encoder.copy_texture_to_buffer(
			// Source (render target texture)
			gpu_image.texture.as_image_copy(),
			// Destination (mappable buffer)
			TexelCopyBufferInfo {
				buffer: &image_copier.buffer,
				layout: TexelCopyBufferLayout {
					offset: 0,
					bytes_per_row: Some(std::num::NonZero::<u32>::new(padded_bytes_per_row as u32).unwrap().into()),
					rows_per_image: None,
				},
			},
			// Size to copy (texture size)
			gpu_image.texture_descriptor.size,
		);

		// Submit commands to GPU queue
		render_queue.submit(std::iter::once(encoder.finish()));
	}
}

// ============================================================================
// RENDER SYSTEMS - systems in Render World
// ============================================================================

/// System that reads data from GPU buffer after copying
/// Runs AFTER RenderGraph (after ImageCopyDriver)
/// Sends data through channel to Main World
fn receive_image_from_buffer(image_copiers: Res<ImageCopiers>, render_device: Res<RenderDevice>, sender: Res<RenderWorldSender>) {
	for image_copier in image_copiers.iter() {
		if !image_copier.is_enabled() {
			continue;
		}

		// Get buffer slice (whole area)
		let buffer_slice = image_copier.buffer.slice(..);

		// Create channel for async callback
		// map_async doesn't block - notifies through channel when ready
		let (s, r) = bounded(1);

		// Request buffer access (mapping)
		buffer_slice.map_async(bevy::render::render_resource::MapMode::Read, move |result| match result {
			Ok(_) => s.send(()).expect("Failed to send map notification"),
			Err(err) => panic!("Failed to map buffer: {err}"),
		});

		// Block until GPU finishes copying
		// On native this blocks thread, on WebGPU - awaits
		render_device
			.poll(bevy::render::render_resource::PollType::wait_indefinitely())
			.expect("Failed to poll device");

		// Wait for callback (buffer ready for reading)
		r.recv().expect("Failed to receive map notification");

		// READ DATA FROM GPU BUFFER → CPU Vec
		let image_bytes = {
			let data = buffer_slice.get_mapped_range();
			data.to_vec() // Copy to owned Vec and drop data
		}; // data dropped here

		// Send to Main World through channel
		// Ignore error if receiver already closed (app exit)
		let _ = sender.send(image_bytes);

		// Must unmap before next use
		image_copier.buffer.unmap();
	}
}

// ============================================================================
// MAIN WORLD SYSTEMS - data processing and saving
// ============================================================================

/// System in Main World that receives data from Render World and saves to file
/// Runs in Last schedule
fn save_captured_frames(
	receiver: Res<MainWorldReceiver>,
	mut frame_counter: Local<u32>,
	mut last_frame_time: Local<Option<std::time::Instant>>,
	mut last_frame_hash: Local<Option<u64>>,
	viewport_stream: Res<ViewportStream>,
) {
	// Throttle to ~60 FPS for output
	let now = std::time::Instant::now();
	if let Some(last_time) = *last_frame_time {
		if now.duration_since(last_time).as_secs_f32() < 0.0167 {
			// Skip this frame - too soon (60 FPS = ~16.67ms)
			return;
		}
	}

	// Use try_recv to not block (non-blocking)
	// May have multiple frames in queue - take latest
	let mut latest_frame: Option<Vec<u8>> = None;
	while let Ok(data) = receiver.try_recv() {
		latest_frame = Some(data);
	}

	let Some(image_data) = latest_frame else {
		return; // No new frames
	};

	// Update last frame time
	*last_frame_time = Some(now);

	// Quick hash check - skip if frame didn't change (static scene optimization)
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
	let mut hasher = DefaultHasher::new();
	image_data.hash(&mut hasher);
	let current_hash = hasher.finish();

	if let Some(prev_hash) = *last_frame_hash {
		if current_hash == prev_hash {
			// Frame unchanged - skip encoding/sending entirely
			return;
		}
	}
	*last_frame_hash = Some(current_hash);

	// Dimensions (hardcoded, same as during creation)
	let width = 1280u32;
	let height = 720u32;
	let row_bytes = width as usize * 4; // RGBA = 4 bytes per pixel
	let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);

	// If there's padding - need to remove it
	let actual_data: Vec<u8> = if row_bytes == aligned_row_bytes {
		image_data
	} else {
		image_data
			.chunks(aligned_row_bytes)
			.take(height as usize)
			.flat_map(|row| &row[..row_bytes])
			.copied()
			.collect()
	};

	// Skip empty frames (first few while GPU hasn't finished rendering)
	let non_zero_count = actual_data.iter().filter(|&&b| b != 0).count();
	if non_zero_count == 0 {
		return;
	}

	// JPEG with LOW quality (fast encoding, small size, hardware decode in browser)
	let mut bevy_img = Image::new_target_texture(
		width,
		height,
		bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
		None,
	);
	bevy_img.data = Some(actual_data);

	let Ok(dynamic_img) = bevy_img.try_into_dynamic() else {
		return;
	};

	let mut jpeg_bytes = std::io::Cursor::new(Vec::new());
	let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 90);
	if dynamic_img.write_with_encoder(encoder).is_err() {
		return;
	}

	use base64::{engine::general_purpose, Engine as _};
	let base64_data = general_purpose::STANDARD.encode(jpeg_bytes.into_inner());

	let _ = viewport_stream.viewport.send(&base64_data);

	*frame_counter += 1;
}
