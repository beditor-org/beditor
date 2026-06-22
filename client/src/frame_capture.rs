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
use flume::{bounded, Receiver, Sender};
use std::collections::hash_map::DefaultHasher;
use std::sync::{atomic::AtomicBool, Arc};

use crate::app::ViewportStream;

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

/// Channel for forwarding raw GPU pixels to the background JPEG encoder thread
#[derive(Resource)]
struct RawFrameSender(flume::Sender<Vec<u8>>);

/// Channel for receiving encoded JPEG frames from the background encoder thread
#[derive(Resource, Deref)]
struct EncodedFrameReceiver(flume::Receiver<Vec<u8>>);

// ============================================================================
// PLUGIN - entry point for the entire capture system
// ============================================================================

pub struct FrameCapturePlugin;

impl Plugin for FrameCapturePlugin {
	fn build(&self, app: &mut App) {
		// Render World → Main World channel (raw GPU pixels, bounded to drop under back-pressure)
		let (mw_sender, mw_receiver) = bounded(2);

		// Main World → encoder thread channel (raw pixels)
		let (raw_tx, raw_rx) = flume::bounded::<Vec<u8>>(2);
		// Encoder thread → Main World channel (encoded JPEG bytes)
		let (enc_tx, enc_rx) = flume::bounded::<Vec<u8>>(2);

		// Spawn background JPEG encoder thread so main/render threads are never blocked by encoding
		std::thread::spawn(move || {
			let mut last_hash: Option<u64> = None;
			// [POINT 3] encoder perf stats
			let mut enc_count: u32 = 0;
			let mut enc_sum_us: u64 = 0;
			let mut enc_max_us: u64 = 0;
			let mut enc_dropped: u32 = 0;
			let mut enc_window = std::time::Instant::now();
			loop {
				// Block until a frame arrives
				let first = match raw_rx.recv() {
					Ok(f) => f,
					Err(_) => break,
				};
				// Drain any queued frames — always encode the freshest one
				let image_data = std::iter::once(first)
					.chain(std::iter::from_fn(|| raw_rx.try_recv().ok()))
					.last()
					.unwrap();

				// Skip empty frames (first few while GPU hasn't rendered yet)
				if image_data.iter().all(|&b| b == 0) {
					continue;
				}

				// Dedup: skip if frame unchanged (static scene)
				let current_hash = {
					use std::hash::{Hash, Hasher};
					let mut hasher = DefaultHasher::new();
					image_data.hash(&mut hasher);
					hasher.finish()
				};
				if last_hash == Some(current_hash) {
					continue;
				}
				last_hash = Some(current_hash);

				// Strip GPU row-alignment padding
				let width = 1280u32;
				let height = 720u32;
				let row_bytes = width as usize * 4;
				let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
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

				// Encode to JPEG via mozjpeg
				let jpeg_start = std::time::Instant::now();
				let jpeg_bytes: Vec<u8> = {
					let mut buf = Vec::new();
					let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBA);
					comp.set_size(width as usize, height as usize);
					comp.set_quality(75.0);
					comp.set_fastest_defaults();
					let mut started = comp.start_compress(&mut buf).expect("mozjpeg start failed");
					started.write_scanlines(&actual_data).expect("mozjpeg write failed");
					started.finish().expect("mozjpeg finish failed");
					buf
				};
				let jpeg_us = jpeg_start.elapsed().as_micros() as u64;

				let _ = enc_tx.try_send(jpeg_bytes);

				// [POINT 3] Update encoder stats
				enc_count += 1;
				enc_sum_us += jpeg_us;
				if jpeg_us > enc_max_us {
					enc_max_us = jpeg_us;
				}
				let now = std::time::Instant::now();
				if now.duration_since(enc_window).as_secs_f32() >= 1.0 {
					if enc_count > 0 {
						eprintln!(
							"[PERF:3] encoder_fps={enc_count} jpeg_avg={:.1}ms total_max={:.1}ms dropped={enc_dropped}",
							enc_sum_us as f32 / enc_count as f32 / 1000.0,
							enc_max_us as f32 / 1000.0
						);
					}
					enc_count = 0;
					enc_sum_us = 0;
					enc_max_us = 0;
					enc_dropped = 0;
					enc_window = now;
				}
			}
		});

		// Main World: receive raw frames, forward to encoder, and send encoded frames out
		app.insert_resource(MainWorldReceiver(mw_receiver))
			.insert_resource(RawFrameSender(raw_tx))
			.insert_resource(EncodedFrameReceiver(enc_rx))
			.add_systems(First, setup_cpu_image)
			.add_systems(Last, (save_captured_frames, send_encoded_frames).chain());

		// Render World: setup entire render pipeline
		let render_app = app.sub_app_mut(RenderApp);

		render_app
			.insert_resource(RenderWorldSender(mw_sender))
			// Pre-allocate ImageCopiers so the extract system can reuse the buffer each frame
			.init_resource::<ImageCopiers>()
			.add_systems(bevy::render::ExtractSchedule, extract_image_copiers)
			.add_systems(
				bevy::render::Render,
				copy_image_to_buffer.after(bevy::render::RenderSystems::Render),
			)
			.add_systems(
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

/// Extract system that creates ImageCopier for EditorCamera and copies to Render World.
/// The GPU buffer is allocated once and reused every frame to avoid per-frame allocation cost.
fn extract_image_copiers(
	mut image_copiers: ResMut<ImageCopiers>,
	cameras: Extract<Query<(&Camera, &RenderTarget), With<super::app::EditorCamera>>>,
	render_device: Res<RenderDevice>,
) {
	// Buffer already exists — reuse it (unmap() was called after last readback)
	if !image_copiers.is_empty() {
		return;
	}

	for (_camera, render_target) in cameras.iter() {
		let RenderTarget::Image(img_target) = render_target else {
			continue;
		};

		let image_handle = img_target.handle.clone();
		let size = Extent3d {
			width: 1280,
			height: 720,
			..Default::default()
		};
		image_copiers.push(ImageCopier::new(image_handle, size, &render_device));
	}
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
	// // [POINT 1] Count actual Bevy render FPS
	// *frame_count += 1;
	// let now = std::time::Instant::now();
	// let win = window_start.get_or_insert(now);
	// if now.duration_since(*win).as_secs_f32() >= 1.0 {
	// 	eprintln!("[PERF:1] render_fps={}", *frame_count);
	// 	*frame_count = 0;
	// 	*window_start = Some(now);
	// }
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

		// // [POINT 2] Measure GPU poll latency
		// let poll_start = std::time::Instant::now();

		// Block until GPU finishes copying
		// On native this blocks thread, on WebGPU - awaits
		render_device
			.poll(bevy::render::render_resource::PollType::wait_indefinitely())
			.expect("Failed to poll device");

		// let poll_us = poll_start.elapsed().as_micros() as u64;

		// Wait for callback (buffer ready for reading)
		r.recv().expect("Failed to receive map notification");

		// READ DATA FROM GPU BUFFER → CPU Vec
		let image_bytes = {
			let data = buffer_slice.get_mapped_range();
			data.to_vec() // Copy to owned Vec and drop data
		}; // data dropped here

		// Send to Main World through channel.
		// try_send: if channel is full (main world throttled), drop the frame
		// rather than letting old frames accumulate and exhaust memory.
		let _ = sender.try_send(image_bytes);

		// Must unmap before next use
		image_copier.buffer.unmap();
	}
}

// ============================================================================
// MAIN WORLD SYSTEMS - data processing and saving
// ============================================================================

/// Drain raw GPU pixels from the render-world channel and forward to the background encoder.
/// Runs in Last schedule — must be near-instant (no encoding here).
fn save_captured_frames(receiver: Res<MainWorldReceiver>, raw_sender: Res<RawFrameSender>) {
	// Drain channel, keep only the freshest frame
	let mut latest: Option<Vec<u8>> = None;
	while let Ok(data) = receiver.try_recv() {
		latest = Some(data);
	}
	if let Some(data) = latest {
		// try_send: drop silently if encoder is still busy with previous frame
		let _ = raw_sender.0.try_send(data);
	}
}

/// Drain encoded frames from the background encoder and send to the editor.
/// Chained after save_captured_frames in the Last schedule.
fn send_encoded_frames(
	enc_receiver: Res<EncodedFrameReceiver>,
	viewport_stream: Res<ViewportStream>,
	mut wire_stats: Local<(u32, Option<std::time::Instant>)>,
) {
	let mut latest: Option<Vec<u8>> = None;
	while let Ok(encoded) = enc_receiver.0.try_recv() {
		latest = Some(encoded);
	}
	if let Some(encoded) = latest {
		let _ = viewport_stream.viewport.send(&encoded);

		// [POINT 4] Count wire send FPS
		let (count, win_start) = &mut *wire_stats;
		*count += 1;
		let now = std::time::Instant::now();
		let win = win_start.get_or_insert(now);
		if now.duration_since(*win).as_secs_f32() >= 1.0 {
			eprintln!("[PERF:4] wire_fps={count}");
			*count = 0;
			*win_start = Some(now);
		}
	}
}
