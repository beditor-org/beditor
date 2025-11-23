// Bridge library for game-editor communication
// TODO: Move communication logic here

pub mod protocol;
pub mod transport;

use std::{
	io::Write,
	process::{Child, ChildStdin},
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc, Mutex, RwLock,
	},
};

use bevy::remote::BrpRequest;
pub use protocol::EditorProtocol;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use transport::EditorTransport;

// Simple entity info for UI display
#[derive(Debug, Clone)]
struct EntityInfo {
	id: u64,
	name: String,
}

// Request ID counter
static REQUEST_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

// Game process handle
#[derive(Debug)]
struct GameProcess {
	stdin: ChildStdin,
	_child: Child,
}

// Owned BRP response structures for deserialization
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OwnedBrpResponse {
	jsonrpc: String,
	id: u64,
	#[serde(flatten)]
	payload: OwnedBrpPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum OwnedBrpPayload {
	Result { result: serde_json::Value },
	Error { error: OwnedBrpError },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OwnedBrpError {
	code: i32,
	message: String,
}

fn send_brp_request(game_process: &Arc<Mutex<GameProcess>>, request: BrpRequest) {
	if let Ok(mut process) = game_process.lock() {
		if let Ok(json) = serde_json::to_string(&request) {
			if let Err(e) = writeln!(process.stdin, "{}", json) {
				eprintln!("❌ Failed to send BRP request to game: {}", e);
			}
		}
	}
}

fn send_brp_query_entities(game_process: &Arc<Mutex<GameProcess>>) {
	let request = BrpRequest {
		jsonrpc: "2.0".to_string(),
		method: "world.query".to_string(),
		id: Some(json!(REQUEST_ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
		params: Some(json!({
			"data": {
				"components": ["bevy_ecs::name::Name"],
				"option": "all",
				"has": []
			},
			"filter": {
				"with": [],
				"without": []
			},
			"strict": false
		})),
	};

	send_brp_request(game_process, request);
}

fn handle_brp_response(
	response: OwnedBrpResponse,
	// state: &Arc<RwLock<EditorState>>
) {
	match response.payload {
		OwnedBrpPayload::Result { result } => {
			// Try to parse as entity query result
			if let Some(entities_array) = result.as_array() {
				let mut entities = Vec::new();

				for entity_data in entities_array.iter() {
					if let Some(components) = entity_data.get("components") {
						// Extract entity ID
						let entity_id = entity_data.get("entity").and_then(|e| e.as_u64()).unwrap_or(0);

						//Extract Name component
						if let Some(name_val) = components.get("bevy_ecs::name::Name") {
							let name = name_val.as_str().unwrap_or("Unknown").to_string();
							entities.push(EntityInfo { id: entity_id, name });
						}
					}
				}

				// if let Ok(mut s) = state.write() {
				// 	s.entities = entities;
				// }
			}
		}
		OwnedBrpPayload::Error { error } => {
			eprintln!("❌ BRP error: {}: {}", error.code, error.message);
		}
	}
}
