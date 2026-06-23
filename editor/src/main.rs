use std::sync::{Arc, RwLock};

use editor::{
	components::App,
	plugin::{
		asset_browser::asset_browser_plugin,
		bep::plugin::bep_plugin,
		core::plugin::core_plugin,
		game_process::game_process_plugin,
		scene_editor::plugin::scene_editor_plugin,
		transport::stdio::stdio_transport_plugin,
		viewport::plugin::{viewport_plugin, ViewportShm},
		PluginBuilder,
	},
	EditorConfig, EditorContext,
};

fn main() {
	let config = EditorConfig::load();
	let editor_state = EditorContext::default();

	// Shared handle for the viewport shm mmap.
	// Populated by the viewport plugin after the game connects;
	// read by the custom protocol handler on every frame request.
	let shm_handle: Arc<std::sync::Mutex<Option<memmap2::Mmap>>> = Arc::new(std::sync::Mutex::new(None));
	let shm_for_handler = shm_handle.clone();

	let window = dioxus::desktop::WindowBuilder::new()
		.with_title(config.window.title.to_string())
		.with_decorations(config.window.decorations)
		.with_resizable(config.window.resizable);

	let window_cfg = dioxus::desktop::Config::new()
		.with_window(window)
		.with_asynchronous_custom_protocol("beditor", move |_id, req, responder| {
			use dioxus::desktop::wry::http::{header, Response as HttpResponse};
			let shm = shm_for_handler.clone();
			tokio::spawn(async move {
				// Only handle beditor://frame
				let is_frame = req.uri().host().map_or(false, |h| h == "frame") || req.uri().path().contains("frame");
				if !is_frame {
					let _ = responder.respond(
						HttpResponse::builder()
							.status(404)
							.body(std::borrow::Cow::Borrowed(b"not found".as_slice()))
							.unwrap(),
					);
					return;
				}

				let guard = shm.lock().unwrap();
				if let Some(mmap) = guard.as_ref() {
					if mmap.len() >= 4 {
						let len = u32::from_be_bytes(mmap[0..4].try_into().unwrap()) as usize;
						if len > 0 && 4 + len <= mmap.len() {
							let data: Vec<u8> = mmap[4..4 + len].to_vec();
							drop(guard);
							let _ = responder.respond(
								HttpResponse::builder()
									.status(200)
									.header(header::CONTENT_TYPE, "application/octet-stream")
									.header(header::CACHE_CONTROL, "no-store")								.header("Access-Control-Allow-Origin", "*")									.body(std::borrow::Cow::Owned(data))
									.unwrap(),
							);
							return;
						}
					}
				}
				drop(guard);
				let _ = responder.respond(
					HttpResponse::builder()
						.status(204)
						.body(std::borrow::Cow::Borrowed(b"".as_slice()))
						.unwrap(),
				);
			});
		});

	let plugins: Vec<PluginBuilder> = vec![
		core_plugin,
		stdio_transport_plugin,
		game_process_plugin,
		viewport_plugin,
		asset_browser_plugin,
		bep_plugin,
		scene_editor_plugin,
	];
	dioxus::LaunchBuilder::desktop()
		.with_cfg(window_cfg)
		.with_context(Arc::new(RwLock::new(editor_state)))
		.with_context(plugins)
		.with_context(config)
		.with_context(shm_handle)
		.launch(App);
}
